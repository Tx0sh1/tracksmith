use walkdir::WalkDir;

fn main() {

    let mut location = String::new();
    println!("Please enter the location of the file you want to search for:");
    std::io::stdin()
        .read_line(&mut location)
        .expect("Failed to read line");
    let location = location.trim();
    println!("Searching for files in: {}", location);
    // Use WalkDir to recursively list all files in the specified directory
    // and its subdirectories

    for entry in WalkDir::new(location).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            println!("{}", entry.path().display());
        }
    

    match entry.path().extension().and_then(|s| s.to_str()) {
        Some("mp3") => println!("Found an MP3 file: {}", entry.path().display()),
        Some("wav") => println!("Found a WAV file: {}", entry.path().display()),
        Some("flac") => println!("Found a FLAC file: {}", entry.path().display()),
        Some("ogg") => println!("Found an OGG file: {}", entry.path().display()),
        Some("m4a") => println!("Found an M4A file: {}", entry.path().display()),
        Some("aac") => println!("Found an AAC file: {}", entry.path().display()),
        Some("wma") => println!("Found a WMA file: {}", entry.path().display()),
        Some("mp4") => println!("Found an MP4 file: {}", entry.path().display()),
        _ => println!("Unknown file type"),
        
    }
}

}

// This code uses the `walkdir` crate to recursively list all files in the current directory and its subdirectories.
// It filters out any errors that occur while traversing the directory and only prints the paths of files.

