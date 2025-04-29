
# 🧠 Project Journal: **Tracksmith**
**Start Date:** 2025-04-29  
**Language/Tech Stack:** Rust, WalkDir  
**Goal:** Build a CLI tool to organize and manage a music library by scanning files, detecting `.mp3` formats, and moving them to designated directories.

**Repo:** [[tracksmith](https://github.com/Tx0sh1/tracksmith)]

---

## 📅 2025-04-29
**What I worked on:**  
- Built the basic structure for **Tracksmith**.
- Implemented file scanning using the `walkdir` crate to recursively find files in a user-specified directory.
- Integrated file extension checking to identify `.mp3` files for further processing.
- Planned future work for metadata extraction and file handling (like moving files based on type).

**Challenges:**  
- Figuring out how to filter specific file extensions while traversing directories.
- Ensuring the program works correctly when provided with user input (directory location).
- Managing code structure for scalability as I add more features (metadata extraction and moving files).

**What I learned / How I solved it:**  
- Learned how to use the `WalkDir` crate for recursive directory traversal.
- Discovered how to match file extensions using `match` with Rust, and structured my code for flexibility with a placeholder for future work.
- Improved error handling for invalid user input and potential file access issues (though this still needs further refinement).

**Next steps:**  
- Research metadata extraction for `.mp3` files using the `lofty` crate.
- Implement logic to move `.mp3` files into designated directories based on user preferences.
- Add a simple logging or JSON tracking system to remember which files have been processed.
- Test the program with various directories and file types.
