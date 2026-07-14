# 🦀 rshelp

> Rust enhanced help tool with beautiful terminal output

[![Build & Release](https://github.com/cumulus13/rshelp/actions/workflows/build.yml/badge.svg)](https://github.com/cumulus13/rshelp/actions/workflows/build.yml)
[![crates.io](https://img.shields.io/crates/v/rshelp.svg)](https://crates.io/crates/rshelp)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## 📋 Description

`rshelp` is a Rust command-line tool that lets you quickly view documentation (docstrings) for Rust functions, structs, traits, and modules directly from your terminal. It helps you inspect documentation without opening source files or browsing online docs.

## ✨ Features

- 🚀 **Fast**: Instant access to Rust documentation
- 🎨 **Beautiful**: Colored terminal output with emojis
- 🔍 **Search**: Find functions, structs, and traits quickly
- 📄 **Source Code**: View source code directly
- 🎯 **Interactive Mode**: Browse documentation interactively
- 📦 **Module Listing**: See all items in a module
- 🔧 **Detailed Mode**: Get comprehensive type information

## 📦 Installation

```bash
# From crates.io
cargo install rshelp

# From source
git clone https://github.com/cumulus13/rshelp.git
cd rshelp
cargo install --path .
```

## 🚀 Usage

```bash
# Show help for a specific item
rshelp std::fs::File

# Show source code
rshelp --source std::fs::File

# Interactive mode
rshelp --interactive

# Search for items
rshelp --search HashMap

# List all items in a module
rshelp --show-all std::collections

# Clear screen before showing results
rshelp --clear std::fs::File
```

## 🎯 Examples

```bash
rshelp std::fs::File
rshelp serde_json::from_str
rshelp --source regex::Regex
rshelp --search HashMap
rshelp --interactive
```

## 📚 Interactive Mode Commands

- `help`, `?` - Show help
- `clear`, `cls` - Clear screen
- `exit`, `quit` - Exit
- `c <query>` - Clear screen and show help
- `:source <q>` - Show source code
- `:list <q>` - List module items
- `:search <q>` - Search for items
- `:detailed` - Toggle detailed mode
- `:version` - Show version

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing`)
3. Make your changes
4. Run tests (`cargo test`)
5. Ensure code formatting (`cargo fmt`)
6. Run clippy (`cargo clippy`)
7. Submit a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👤 Author

**Hadi Cahyadi**
- GitHub: [@cumulus13](https://github.com/cumulus13)
- Email: cumulus13@gmail.com

## 🙏 Support

If you find this tool useful, consider supporting the author:

- ⭐ Star the repository on GitHub
- ☕ [Buy me a coffee](https://www.patreon.com/cumulus13)

---

Built with ❤️ in Rust
