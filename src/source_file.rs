use regex::Regex;
use std::collections::HashMap;
use std::io::{BufReader, BufRead}; //BufRead implements the lines trait
use std::path::PathBuf;
use std::fs:: File;
use std::collections::HashMap;


pub struct SourceFiles {
    pub files: Vec<SourceFile>,
}

impl SourceFiles {
    pub fn new() -> SourceFiles {
        SourceFiles {
            files: Vec::new()
        }
    }
}


// COMMENT: System Design
// In general it is wise to hide as much implementation detail as possible, and to only expose
// the bare minimum to other components. This reduces the impact of future refactions. It also
// allows people to think more easily about the system, since more irrelevant detail is hidden about
// system interactions
pub struct SourceFile {
    // maps functions to the class where they are declared
    // dont think I need separate class object since all we need is the name
   // func_list: HashMap<String, FuncSignature>,
    pub func_list: HashMap<String, FuncSignature>,
    pub file_path: PathBuf,
}

pub struct FuncSignature {
    //TODO: add arguments and return type
    name: String,
}

impl SourceFile {
    // the lifetime parameters say that the output variable's scope is tied to the scope of line
    // this assertion is checked by the Rust compiler during compile to make sure no dangling
    // pointers are being used
    fn find_pat<'a>(line:&'a str, pat:&str) -> Option<&'a str> {
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

    // 
    fn parse_brackets(line: String, stack: Vec<u8>) {
        for c in line.chrs() {

        } 
    }
    
    // parse functions name and args
    // parse parents class structure
    pub fn parse_source(&mut self) -> () {
        //TODO: change this to use ? and modify function signature to return error
        let input = File::open(&self.file_path).unwrap();
        let buffered = BufReader::new(input);
        println!("filepath: {}", self.file_path.to_str().unwrap());
       
        //OPTIMIZATION: allocate storage so that we dont need to reinitialize them each time
        //parse_source is called
        // Current function or class scope
        let class = String::from("");
        let function = String::from("");
        
        // Use these to determine the scoping levels
        // holds pairs of brackets
        // OPTIMIZATION: implement a push/pop that will automatically update some parameter holding
        // the current function scope
        let function_scope = Vec::new(u8);
        let class_scope = Vec::new(u8);


        // Holds the lines to the buffer
        let function_buf = Vec::<&String>;
        
        // Parse loop
        // Naive parsing implementation, we make assumptions:
        // 1.Look for function definitions (function signature followed by "}")
        // 2.Assume no inner function definitions
        // 3.Look for function calls
        for line in buffered.lines() {
            let line = line.unwrap();
            if line.contains("class") {
                println!("CLASS DETECTED: {}", line);
            }
            // get function names and arguments
            if line.contains("public") || line.contains("private") {
                println!("{}", line);
            }
            
            // parse brackets contained in the line and update our scopes
            ParseSource::parse_brackets(line, function_scope);
            ParseSource::parse_brackets(line, class_scope);
                
            // check if scope is closed
            if function_scope

        } 
    } 
}
