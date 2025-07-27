# 🧹 windows-cleaning-lab

A fast, modular, and community-driven **Windows cleaner tool** written in **Rust**.  
`windows-cleaning-lab` allows you to clean up unnecessary files and system clutter from your Windows machine with speed and precision.

---

## ✨ Features

- 📂 Cleans temporary files (`%TEMP%`, prefetch, etc.)
- 📁 Clears recent files history
- 🌲 Runs `tree C:/` for disk structure insights
- 🧽 Leverages Windows’ built-in `cleanmgr` utility
- ⚙️ CLI-based and blazing fast (thanks to Rust)
- 🤝 Community-contributed cleanup rules and features
- 🛠️ Designed with extensibility in mind

---

## 📸 Preview

```bash
> windows-cleaning-lab
=============================================
=========== WINDOWS CLEANING LAB ============
=============================================


[✓] Done tre C:/
[✓] Cleaned `Recent Files`
[✓] Cleaned `Temp Files`
[✓] Cleaned `%TEMP% Files`
[✓] Cleaned `Prefetch Files`
[✓] Done `cleanmgr`


All cleaning tasks completed!
```

---

## 📦 Installation

### Option 1: Download Prebuilt Binary (Coming Soon)

> Precompiled `.exe` files will be available in the [Releases](https://github.com/mahikoishor/windows-cleaning-lab/releases) section.

### Option 2: Build from Source

```bash
git clone https://github.com/mahikoishor/windows-cleaning-lab.git
cd windows-cleaning-lab
cargo build --release
```

The executable will be in:

```
target/release/windows-cleaning-lab.exe
```

---

## 🔐 Admin Privileges

Some cleaning operations (like running `cleanmgr` or deleting system temp files) require **administrator privileges**.  
Make sure to run the terminal or app as **Administrator** for full functionality.

---

## 🛡 Disclaimer

Use this utility at your own risk.  
It performs file deletions and system-level cleanup tasks.  
Review the code and test the features before using it in production or automation.

---

## 🤝 Contributing

We welcome contributions of all kinds!

### Steps:

1. Fork the repository
2. Create your branch:  
   `git checkout -b feature/your-feature-name`
3. Make your changes
4. Submit a Pull Request

---

## 🧑‍💻 Guidelines

- Use idiomatic, safe, and readable Rust
- Keep features modular and testable
- Avoid hardcoded paths (use environment vars or config)
- Document your code
- Follow semantic versioning for releases

---

## 📚 Roadmap

- [ ] GUI support (Tauri or egui)
- [ ] Scheduling via Task Scheduler integration
- [ ] Auto-update functionality
- [ ] Multi-drive cleaning support
- [ ] Log output to file with timestamps
- [ ] Unit & integration testing

---

## 🧠 Motivation

Windows tends to accumulate clutter over time, and many cleaner tools are either bloated or closed-source.  
`windows-cleaning-lab` is designed to be a fast, transparent, and customizable alternative — powered by Rust and the open-source community.

---

## ⭐️ Show Your Support

If you like this project:

- ⭐️ Star the repository
- 🛠 Contribute a feature or improvement
- 💬 Share with your network or community
- 🐞 Report bugs or suggest ideas via Issues

---

Made with ⚙️ and ❤️ in Rust.
