Name:           selinux-explain
Version:        0.5.1
Release:        1%{?dist}
Summary:        A CLI tool to explain SELinux denials and provide actionable fixes

License:        MIT
URL:            https://github.com/mattiabandini1/selinux-explain
Source0:        https://github.com/mattiabandini1/selinux-explain/archive/refs/tags/v%{version}.tar.gz

BuildRequires:  cargo
BuildRequires:  rust

%description
selinux-explain is a lightweight, zero-dependency CLI tool written in Rust that
translates cryptic SELinux AVC denials into human-readable explanations.
It also provides exact commands to fix the issues.

%prep
%autosetup

%build
# Compila il progetto in Rust
cargo build --release

%install
# Installa l'eseguibile nella cartella bin di sistema
install -D -m 755 target/release/selinux-explain %{buildroot}%{_bindir}/selinux-explain

# Crea la cartella vuota per le regole custom del sysadmin
install -d -m 755 %{buildroot}%{_sysconfdir}/selinux-explain

%files
%license LICENSE
%doc README.md
%{_bindir}/selinux-explain
%dir %{_sysconfdir}/selinux-explain

%changelog
* Sat Apr 18 2026 Mattia Bandini <mattia.bandini.mb1@gmail.com> - 0.5.1-1
- Initial COPR package release
