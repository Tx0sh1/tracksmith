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
    }
}
// This code uses the `walkdir` crate to recursively list all files in the current directory and its subdirectories.
// It filters out any errors that occur while traversing the directory and only prints the paths of files.

