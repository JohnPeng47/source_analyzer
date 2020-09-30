use std::io;
use std::io::{BufReader, BufRead}; //BufRead implements the lines trait
use std::env;
use std::path::{Path, PathBuf};
use std::fs::{self, DirEntry, File};
use regex::Regex;

const function_pats: &'static [&'static str] = &[r"(public|private).+{$"];

struct SourceFiles {
    files: Vec<SourceFile>,
}

impl SourceFiles {
    fn new() -> SourceFiles {
        SourceFiles {
            files: Vec::new()
        }
    }
}

struct SourceFile {
    func_list: Vec<FuncSignature>,
    file_path: PathBuf,
}

struct FuncSignature {
    name: String,
}


trait ParseSource {
    // str is a reference to a statically allocated str? Or does it work with String too?
    // LifeTime
    // the lifetime parameters say that the output variable's scope is tied to the scope of line
    // this assertion is checked by the Rust compiler during compile to make sure no dangling
    // pointers are being used
    fn find_func<'a>(line: &'a str, pat: &str) -> Option<&'a str>;

    // Is it better to use &mut self here or self and return Self?
    fn parse_source(&mut self) -> ();
}

impl ParseSource for SourceFile {
    fn find_func<'a>(line:&'a str, pat:&str) -> Option<&'a str> {
        let pattern = Regex::new(pat).unwrap();
        // is there no better way to handle this?
        // Get someone's opinion on this
        let arg = pattern.captures(line);
        match arg {
            None => None,
            Some(x) => {
                let matched_arg = x.get(1)
                    .expect("Error getting first matching group")
                    .as_str();
                return Some(matched_arg)
            }
        }
    }
    // parse functions name and args
    // parse parents class structure
    fn parse_source(&mut self) -> () {
        //TODO: change this to use ? and modify function signature to return error
        let input = File::open(&self.file_path).unwrap();
        let buffered = io::BufReader::new(input);
        println!("filepath: {}", self.file_path.to_str().unwrap());
        
        // Parse loop
        for line in buffered.lines() {
            let line = line.unwrap();
            if line.contains("class") {
                println!("CLASS DETECTED: {}", line);
            }
            // get function names and arguments
            if line.contains("public") || line.contains("private") {
                println!("{}", line);
            }

        } 
    }
}

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

fn test(s: SourceFile) {
    ()
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
