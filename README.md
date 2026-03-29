# 🦀 selinux-explain

> A lightweight, blazing-fast CLI tool written in Rust that translates cryptic SELinux AVC denials into plain, human-readable English.

---

## 🤔 Why this tool?

SELinux is a lifesaver, but reading its `audit.log` is often a nightmare.

While tools like `setroubleshoot` exist, they often:

* Require heavy Python daemons running in the background.
* Bring in a lot of dependencies.
* Are not ideal for minimal, headless, or air-gapped servers.

**`selinux-explain` is different:**

* **Zero Dependencies:** It's a single compiled Rust binary.
* **Offline by Default:** No API calls, no data sent outside. It parses your local logs using regular expressions.
* **Human-Readable:** It translates contexts and types into logical sentences.

---

## 🚀 Installation

*Note: Pre-compiled binaries are coming soon. For now, you can build it from source using Cargo.*

```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/selinux-explain.git
cd selinux-explain

# Build and run the project
cargo build --release
```

---

## 🛠️ Usage

You can use the tool in two ways:

### 1. Analyze the latest denial from your system log (requires root)

```bash
sudo cargo run -- --last
```

### 2. Analyze a specific log line manually

```bash
cargo run -- --text "type=AVC msg=audit(1612345678.123:456): avc:  denied  { read } for  pid=1234 comm=\"nginx\" name=\"index.html\" scontext=system_u:system_r:httpd_t:s0 tcontext=unconfined_u:object_r:user_home_t:s0"
```

---

## 🤝 Contributing

Pull requests are welcome! If you find a log that doesn't parse correctly, please open an issue with the raw log string.
