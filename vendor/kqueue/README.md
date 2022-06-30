# kqueue

[![Gitlab Pipelines](https://gitlab.com/worr/rust-kqueue/badges/master/pipeline.svg)](https://gitlab.com/worr/rust-kqueue/-/commits/master)
[![Travis Build Status](https://travis-ci.com/worr/rust-kqueue.svg?branch=master)](https://travis-ci.com/worr/rust-kqueue)

`kqueue(2)` library for rust

`kqueue(2)` is a powerful API in BSDs that allows you to get events based on
fs events, buffer readiness, timers, process events and signals.

This is useful for code that's either BSD-specific, or as a component in an
abstraction over similar APIs in cross-platform code.

## Docs

I don't recommend using https://docs.rs for documentation, since the builds
aren't done on BSD nodes. I host documentation at
https://docs.worrbase.com/rust/kqueue/ .

## Examples

There are some basic usage examples in `examples/`.
