// This is an unpolished program that does two things:
// Walks through the directory tree of current dir recursively and prints out the files
// It prints the number of files in current directory only (not recursively)
//
use std::fs;
use std::path::PathBuf;
fn main() -> std::io::Result<()> {
    get_entries(PathBuf::from("."));
    get_count_files();
    Ok(())
}

// print out the file names
fn get_entries(path: PathBuf) -> std::io::Result<()>{
    for entry in fs::read_dir(path)? {
        let dir = entry?;
        //Displays file name
        println!("{:?}", dir.path());
        // Displays file size
        println!("{:?} KB",dir.metadata()?.len()/1000);
        //Invokes get_entries recursively to list all files in subdirectory
        if dir.file_type()?.is_dir() {
            get_entries(dir.path())?
        }
    }
    Ok(())
   
}
// Gets count of files in directory
fn get_count_files() {
    let paths = fs::read_dir(PathBuf::from(".")).unwrap();
    println!("count of files: {}",paths.count());
}
