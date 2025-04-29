# 🧠 Project Journal: Tracksmith
**Start Date:** 2025-04-29  
**Language/Tech Stack:** Rust, clap, walkdir, lofty (planned), serde (planned)  
**Goal:** Build a command-line tool to scan, organize, and manage my local music library by reading metadata and renaming files.  
**Repo:** [Add your GitHub repo link here]

---

## 📅 2025-04-29
**What I worked on:**  
- Decided to build a music library manager CLI tool in Rust  
- Explored the `clap` crate and understood how to create subcommands  
- Set up initial project structure and CLI arguments  

**Challenges:**  
- Wondering how to persist music library data (flat file vs embedded db)  
- Unsure what crate supports all audio formats best for metadata  

**What I learned / How I solved it:**  
- `clap` has a clean derive-based macro interface  
- Found crates like `walkdir` for recursive file scanning and `lofty` for reading tags  

**Next steps:**  
- Implement a `scan` command to list audio files in a directory  
- Try extracting metadata from MP3 files using `lofty`  
- Plan folder structure and a basic config file approach

---

## 💡 Reflections
- Excited about this project — it feels personal and useful  
- Want to build it out piece by piece instead of rushing  
- Journaling this feels strange but helpful; curious how it'll feel in a month
