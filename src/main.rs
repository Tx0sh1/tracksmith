use walkdir::WalkDir;
// future: use lofty to read metadata from mp3 files

fn main() {

    let mut location = String::new();
    println!("Please enter the location of the file you want to search for:");
    std::io::stdin()
        .read_line(&mut location)
        .expect("Failed to read line");
    let location = location.trim();
    println!("Searching for files in: {}", location);
    // future: add error handling for invalid paths

    for entry in WalkDir::new(location).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            match entry.path().extension().and_then(|s| s.to_str()) {
                Some("mp3") => {
                    println!("Found an MP3 file: {}", entry.path().display());
                    todo!("Add code to move MP3 file to music folder");
                    // future: extract metadata
                }
                _ => {
                    println!("Found a file with an unknown extension: {}", entry.path().display());
                    todo!("Add code to handle files with unknown extensions");

                }
            } 
    }
}

}

