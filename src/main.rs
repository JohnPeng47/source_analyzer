pub mod source_file;

use std::io;
use std::io::{BufReader, BufRead}; //BufRead implements the lines trait
use std::env;
use std::path::{Path, PathBuf};
use std::fs::{self, DirEntry, File};
use std::collections::HashMap;
use regex::Regex;

use source_file::{SourceFiles, SourceFile, FuncSignature};

const function_pats: &'static [&'static str] = &[r"(public|private).+{$"];

fn main() {
    // get path from stdin
    let arg = env::args().nth(1).unwrap();
    let path = Path::new(&arg);
    println!("Source directory: {}", path.to_str().unwrap());

    // need to declare a mutable reference to the struct first
    let mut files = SourceFiles::new();
    
    // iterate directory
    visit_dirs(path, &path_cb, &mut files);

    // process files
    for source in &mut files.files {
        source.parse_source();
    }
}

fn path_cb(entry: &DirEntry, mut files: &mut SourceFiles) -> () {
    let path = entry.path();
    // this doesnt work: figure out why?
    //let str_path = path.to_str()?;
    let str_path = path.to_str().unwrap(); 

    if str_path.contains(".java") {

        let mut source_file = SourceFile {
            file_path : path,
            func_list : Vec::new()
        };

        files.files.push(source_file);
        
        //println!("{}", str_path);
    }   
}

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry, &mut SourceFiles), mut files: &mut SourceFiles) -> io::Result<()>{
    if dir.is_dir() {
        for entry in fs::read_dir(dir)?{
            // Because entry is a Result object; most things in Rust are
            let entry = entry?;
            let path = entry.path();
            
            
            // if path is directory recurse
            if path.is_dir() {
                // dont need to modify dir.path or dir so dont need mutable references
                // also at this point entry lifetime 
                visit_dirs(&path, &path_cb, &mut files);
            }
            // if file process with cb function
            else {
                cb(&entry, &mut files);
            }
        }
    }
    Ok(())
}
