<h1 align="center">🔐 selinux-explain</h1>

<p align="center">
  <em>Translates cryptic SELinux AVC denials into plain, human-readable English.</em>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust"/>
  <img src="https://img.shields.io/badge/Fedora-51A2DA?style=for-the-badge&logo=fedora&logoColor=white" alt="Fedora"/>
  <img src="https://img.shields.io/badge/SELinux-CC0000?style=for-the-badge&logo=redhat&logoColor=white" alt="SELinux"/>
  <img src="https://img.shields.io/badge/license-MIT-green?style=for-the-badge" alt="License"/>
</p>

<p align="center">
  <img src="assets/demo.gif" alt="selinux-explain demo" width="100%"/>
</p>
<p align="center">
  <em>Analyzing a real AVC denial — from raw log to human-readable fix, in under a second.</em>
</p>

---

## 🤔 Why this tool?

SELinux is a lifesaver, but reading its `audit.log` is often a nightmare.

When SELinux blocks something, most people either paste incomprehensible logs on StackOverflow or — worse — run `setenforce 0` and forget about it. Tools like `setroubleshoot` help, but they require a Python daemon running in the background, bring in heavy dependencies, and are not ideal for minimal, headless, or air-gapped servers.

`selinux-explain` is different:

- **Single static binary** — no daemon, no D-Bus, no runtime dependencies
- **Offline by default** — no API calls, no data sent outside your machine
- **Human-readable output** — tells you what happened, why it was blocked, and how to fix it without disabling SELinux
- **Works everywhere SELinux does** — Fedora, RHEL, Rocky Linux, AlmaLinux, CentOS Stream

> 💡 **TL;DR** — no daemon, no Python, no network. A single static binary that tells you what happened and how to fix it.

---

## 🚀 Installation

### Install via COPR (recommended for Fedora/RHEL)

```bash
sudo dnf copr enable matband/selinux-explain
sudo dnf install selinux-explain
```

Supports Fedora 42, 43, 44 and EPEL 9, 10 (RHEL/CentOS/AlmaLinux/Rocky Linux).

### Download pre-compiled binary

Head to the [Releases page](https://github.com/mattiabandini1/selinux-explain/releases) and download the latest binary for your architecture, then:

```bash
chmod +x selinux-explain
sudo mv selinux-explain /usr/local/bin/
```

### Build from source

```bash
git clone https://github.com/mattiabandini1/selinux-explain.git
cd selinux-explain
cargo build --release
sudo cp target/release/selinux-explain /usr/local/bin/
```

That's it. The binary is now available system-wide.

---

## 🛠️ Usage

**Analyze the latest AVC denial in your system log:**

```bash
sudo selinux-explain --last
```

**Analyze a specific log line:**

```bash
selinux-explain --text "type=AVC msg=audit(1612345678.123:456): avc: denied { read } for pid=1234 comm=\"nginx\" name=\"index.html\" scontext=system_u:system_r:httpd_t:s0 tcontext=unconfined_u:object_r:user_home_t:s0 tclass=file"
```

**Pipe directly from the audit log:**

```bash
grep "avc: denied" /var/log/audit/audit.log | selinux-explain
```

**Report an unmatched denial to help improve the rule database:**

```bash
selinux-explain --report --last
# or combined with --text
selinux-explain --report --text "type=AVC msg=audit(...)"
```

If the denial has no matching rule, `--report` generates a pre-filled GitHub issue link with the rule template already populated. Open it in your browser and hit Submit — no code required.

---

## 🗺️ Roadmap

- [x] Parse AVC denial log lines with regex
- [x] Human-readable output with color
- [x] `--last` flag to analyze the latest denial from `/var/log/audit/audit.log`
- [x] `--text` flag to analyze a specific log line
- [x] Context-aware suggestions for common cases (httpd_t, container_t).
- [x] Stdin / pipe support
- [x] Pre-compiled binaries via GitHub Releases
- [x] Extended suggestion engine via external `rules.toml`.
- [x] `--report` flag to generate a ready-to-paste rule template for unmatched denials
- [x] RPM package / COPR repository

---

## 🤝 Contributing

The suggestion engine is powered by a community-curated `rules.toml` file. If you have a real-world SELinux denial that isn't covered, you can contribute a new rule without touching any Rust code.

Each rule needs four fields:

```toml
[[rules]]
source_type = "the_process_t"    # SELinux type of the blocked process
action = "read"                  # the denied action (read, write, name_connect...)
tclass = "file"                  # the object class (file, dir, tcp_socket...)
suggestion = "Human-readable explanation of what happened."
fix = "The exact command to fix it."
```

Open a Pull Request adding your rule to `rules.toml`, or open an Issue with the raw log line and the fix that worked for you.

**System-wide overrides:** You can create your own custom rule file at `/etc/selinux-explain/rules.toml`. If found, the tool will prioritize it over the embedded rules.

---

<p align="center">
  Made with ❤️ and Rust by <a href="https://mattiabandini.com">Mattia Bandini</a>
</p>
