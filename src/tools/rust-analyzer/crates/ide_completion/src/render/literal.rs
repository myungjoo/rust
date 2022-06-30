//! Renderer for `enum` variants.

use hir::{db::HirDatabase, Documentation, HasAttrs, StructKind};
use ide_db::SymbolKind;

use crate::{
    context::{CompletionContext, PathCompletionCtx},
    item::CompletionItem,
    render::{
        compute_ref_match, compute_type_match,
        variant::{
            format_literal_label, render_record_lit, render_tuple_lit, visible_fields,
            RenderedLiteral,
        },
        RenderContext,
    },
    CompletionItemKind, CompletionRelevance,
};

pub(crate) fn render_variant_lit(
    ctx: RenderContext<'_>,
    local_name: Option<hir::Name>,
    variant: hir::Variant,
    path: Option<hir::ModPath>,
) -> Option<CompletionItem> {
    let _p = profile::span("render_enum_variant");
    let db = ctx.db();

    let name = local_name.unwrap_or_else(|| variant.name(db));
    render(ctx, Variant::EnumVariant(variant), name, path)
}

pub(crate) fn render_struct_literal(
    ctx: RenderContext<'_>,
    strukt: hir::Struct,
    path: Option<hir::ModPath>,
    local_name: Option<hir::Name>,
) -> Option<CompletionItem> {
    let _p = profile::span("render_struct_literal");
    let db = ctx.db();

    let name = local_name.unwrap_or_else(|| strukt.name(db));
    render(ctx, Variant::Struct(strukt), name, path)
}

fn render(
    ctx @ RenderContext { completion, .. }: RenderContext<'_>,
    thing: Variant,
    name: hir::Name,
    path: Option<hir::ModPath>,
) -> Option<CompletionItem> {
    let db = completion.db;
    let kind = thing.kind(db);
    let has_call_parens =
        matches!(completion.path_context, Some(PathCompletionCtx { has_call_parens: true, .. }));

    let fields = thing.fields(completion)?;
    let (qualified_name, short_qualified_name, qualified) = match path {
        Some(path) => {
            let short = hir::ModPath::from_segments(
                hir::PathKind::Plain,
                path.segments().iter().skip(path.segments().len().saturating_sub(2)).cloned(),
            );
            (path, short, true)
        }
        None => (name.clone().into(), name.into(), false),
    };
    let qualified_name = qualified_name.to_string();
    let snippet_cap = ctx.snippet_cap();

    let mut rendered = match kind {
        StructKind::Tuple if !has_call_parens => {
            render_tuple_lit(db, snippet_cap, &fields, &qualified_name)
        }
        StructKind::Record if !has_call_parens => {
            render_record_lit(db, snippet_cap, &fields, &qualified_name)
        }
        _ => RenderedLiteral { literal: qualified_name.clone(), detail: qualified_name.clone() },
    };

    if snippet_cap.is_some() {
        rendered.literal.push_str("$0");
    }

    let mut item = CompletionItem::new(
        CompletionItemKind::SymbolKind(thing.symbol_kind()),
        ctx.source_range(),
        format_literal_label(&qualified_name, kind),
    );

    item.detail(rendered.detail);

    match snippet_cap {
        Some(snippet_cap) => item.insert_snippet(snippet_cap, rendered.literal),
        None => item.insert_text(rendered.literal),
    };

    if qualified {
        item.lookup_by(format_literal_label(&short_qualified_name.to_string(), kind));
    }
    item.set_documentation(thing.docs(db)).set_deprecated(thing.is_deprecated(&ctx));

    let ty = thing.ty(db);
    item.set_relevance(CompletionRelevance {
        type_match: compute_type_match(ctx.completion, &ty),
        ..ctx.completion_relevance()
    });
    if let Some(ref_match) = compute_ref_match(completion, &ty) {
        item.ref_match(ref_match);
    }

    if let Some(import_to_add) = ctx.import_to_add {
        item.add_import(import_to_add);
    }
    Some(item.build())
}

#[derive(Clone, Copy)]
enum Variant {
    Struct(hir::Struct),
    EnumVariant(hir::Variant),
}

impl Variant {
    fn fields(self, ctx: &CompletionContext) -> Option<Vec<hir::Field>> {
        let fields = match self {
            Variant::Struct(it) => it.fields(ctx.db),
            Variant::EnumVariant(it) => it.fields(ctx.db),
        };
        let (visible_fields, fields_omitted) = match self {
            Variant::Struct(it) => visible_fields(ctx, &fields, it)?,
            Variant::EnumVariant(it) => visible_fields(ctx, &fields, it)?,
        };
        if !fields_omitted {
            Some(visible_fields)
        } else {
            None
        }
    }

    fn kind(self, db: &dyn HirDatabase) -> StructKind {
        match self {
            Variant::Struct(it) => it.kind(db),
            Variant::EnumVariant(it) => it.kind(db),
        }
    }

    fn symbol_kind(self) -> SymbolKind {
        match self {
            Variant::Struct(_) => SymbolKind::Struct,
            Variant::EnumVariant(_) => SymbolKind::Variant,
        }
    }

    fn docs(self, db: &dyn HirDatabase) -> Option<Documentation> {
        match self {
            Variant::Struct(it) => it.docs(db),
            Variant::EnumVariant(it) => it.docs(db),
        }
    }

    fn is_deprecated(self, ctx: &RenderContext<'_>) -> bool {
        match self {
            Variant::Struct(it) => ctx.is_deprecated(it),
            Variant::EnumVariant(it) => ctx.is_deprecated(it),
        }
    }

    fn ty(self, db: &dyn HirDatabase) -> hir::Type {
        match self {
            Variant::Struct(it) => it.ty(db),
            Variant::EnumVariant(it) => it.parent_enum(db).ty(db),
        }
    }
}
