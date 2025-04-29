
# Tracksmith 🎵
A simple CLI tool to organize and manage your local music library using Rust.

## 🚀 Features

- Recursively scans a user-specified directory for music files.
- Detects `.mp3` files (more formats coming soon).
- Future plans to:
  - Extract and organize files by metadata (artist, album, etc.)
  - Track processed files using a JSON log
  - Move files to designated music folders
  - Handle duplicates and missing metadata
  - Add concurrency for faster processing

## 🛠️ Built With

- [Rust](https://www.rust-lang.org/)
- [walkdir](https://docs.rs/walkdir/latest/walkdir/) – for directory traversal
- [lofty](https://docs.rs/lofty/latest/lofty/) *(planned)* – for reading music metadata

## 🗂️ Future Enhancements

- 🎶 Read and sort by metadata (artist, album, etc.)
- ⚙️ Config file to define behavior
- 🧵 Multithreaded file handling
- 📁 Watch a folder in real-time
- 🧪 Unit tests and CI integration

## 🤝 Contributing

Pull requests welcome! Please consider opening an issue first to discuss changes or ideas.

## 📄 License

[MIT](./LICENSE)

---

> *Made with ❤️ by Karabo*  
> *Goal: Learn Rust, build tools, and rewrite Linux... eventually 😉*
