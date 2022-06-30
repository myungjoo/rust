Name:		rust
Summary:	The Rust Programming Language
Version:	1.62.0
Release:	0
Group:		Development/Tools
Packager:	MyungJoo Ham <myungjoo.ham@samsung.com>
License:	Apache-2.0 and MIT
Source0:	rust-%{version}.tar.gz
Source1001:	rust.manifest
Source1002:	config.toml.in

# Bootstraping!
Source2001:	cargo-1.60.0-x86_64-unknown-linux-gnu.tar.xz
Source2002:	rust-std-1.60.0-x86_64-unknown-linux-gnu.tar.xz
Source2003:	rustc-1.60.0-x86_64-unknown-linux-gnu.tar.xz

BuildRequires:	python3
BuildRequires:	ninja
BuildRequires:	curl
BuildRequires:	pkg-config
BuildRequires:  libopenssl


%description
Rust is a programming language for software ranging from system software to web applications.

%prep
%setup -q
cp %{SOURCE1001} .
cp %{SOURCE1002} .

%define cachepath build/cache/2022-04-07/
mkdir -p %{cachepath}
cp %{SOURCE2001} %{cachepath}
cp %{SOURCE2002} %{cachepath}
cp %{SOURCE2003} %{cachepath}

sed -i "s|@PREFIX@|%{buildroot}%{_prefix}|" config.toml.in
sed -i "s|@SYSCONFDIR@|%{buildroot}%{_sysconfdir}|" config.toml.in
sed -i "s|@BIN@|bin|" config.toml.in
sed -i "s|@LIB@|%{_lib}|" config.toml.in
mv config.toml.in config.toml

%build
python3 x.py build

%install
python3 x.py install

%files
%manifest rust.manifest
%{_bindir}/*
