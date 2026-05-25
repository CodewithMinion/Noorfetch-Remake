Name:          noorfetchre
Version:       4.0.0
Release:       1
License:       LGPL-3.0-or-later 
Group:         Unspecified
Summary:       Minimal and fast system information fetch tool written in Rust
Source0:       noorfetchre-%{version}.tar.gz 
URL:           https://codeberg.org/limforge/noorfetch

BuildRequires:      cargo  
BuildRequires:      gcc  
BuildRequires:      rust  

%description
Noorfetch Remake is a minimalistic and fast summary of information about your computer, written in Rust.

%prep
%setup -q -n noorfetchre

%build
cargo build --release

%install
install -D -m 0755 target/release/noorfetchre %{buildroot}%{_bindir}/noorfetchre

%files
 %{_bindir}/noorfetchre


