use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader}; //BufRead implements the lines trait
use std::path::PathBuf;

pub struct SourceFiles {
    pub files: Vec<SourceFile>,
}

impl SourceFiles {
    pub fn new() -> SourceFiles {
        SourceFiles { files: Vec::new() }
    }
}

// COMMENT: System Design
// In general it is wise to hide as much implementation detail as possible, and to only expose
// the bare minimum to other components. This reduces the impact of future refactions. It also
// allows people to think more easily about the system, since more irrelevant detail is hidden about
// system interactions
pub struct SourceFile {
    // FuncSignature mapped to class names
    pub func_list: HashMap<String, FuncSignature>,
    pub file_path: PathBuf,
}

// maybe rename
pub struct FuncSignature {
    //TODO: add arguments and return type
    // name: String,
    content: Vec<String>,
}

impl SourceFile {
    // the lifetime parameters say that the output variable's scope is tied to the scope of line
    // this assertion is checked by the Rust compiler during compile to make sure no dangling
    // pointers are being used
    fn find_pat<'a>(line: &'a str, pat: &str) -> Option<&'a str> {
        let pattern = Regex::new(pat).unwrap();
        // is there no better way to handle this?
        // Get someone's opinion on this
        let arg = pattern.captures(line);
        match arg {
            None => None,
            Some(x) => {
                let matched_arg = x
                    .get(1)
                    .expect("Error getting first matching group")
                    .as_str();
                return Some(matched_arg);
            }
        }
    }

    // parse brackets in a line and modify the function scope stack
    // Cuurently assumes that brackets will be balanced ie. no declaration of "{" as a string
    // literal or something
    fn parse_brackets(line: &String, mut stack: &mut Vec<char>) -> bool {
        for c in line.chars() {
            match c {
                // stack could only hold { since anything else would have been popped off
                c if String::from(c) == "}" => {
                    stack.pop();
                    // empty stack means that the current scope have been closed
                    if stack.is_empty() {
                        return true;
                    }
                }
                c if String::from(c) == "{" => {
                    stack.push(c);
                }
                _ => (),
            }
            }
            false
        }

        // parse functions name and args
        // parse parents class structure
        pub fn parse_source(&mut self) -> () {
            //TODO: change this to use ? and modify function signature to return error
            let input = File::open(&self.file_path).unwrap();
            let buffered = BufReader::new(input);
            //println!("filepath: {}", self.file_path.to_str().unwrap());

            //OPTIMIZATION: allocate storage so that we dont need to reinitialize them each time
            //parse_source is called
            // Current function or class scope
            let class = String::from("");
            let function = String::from("");

            // Use these to determine the scoping levels
            // holds pairs of brackets
            // OPTIMIZATION: implement a push/pop that will automatically update some parameter holding
            // the current function scope
            let mut function_scope = Vec::<char>::new();
            let mut class_scope = Vec::<char>::new();

            // Holds the lines to the buffer
            let function_buf = Vec::<&String>::new();

            // Parse loop
            // Naive parsing implementation, we make assumptions:
            // 1.Look for function definitions (function signature followed by "}")
            // 2.Assume no inner function definitions
            // 3.Look for function calls
            for line in buffered.lines() {
                let line = line.unwrap();

                // get function names and arguments
                if line.contains("public") || line.contains("private") {
                    if line.contains("class") {
                        //println!("CLASS DETECTED: {}", line);
                        println!("class: {}", line.split(" ").nth(2).unwrap_or("problem"));
                    } else if line.contains("(") && line.contains(")") && line.trim_start().trim_end().
                        let mut line_trimmed = line.split_whitespace();
                    println!("        function name: {}", line);

                    let func_arg = line_trimmed.nth(1).unwrap();
                    let func_name = line_trimmed.nth(2).unwrap();

                    //println!("SPLIT ARGS: {} {}", func_arg, func_name);
                    for l in line_trimmed {
                        println!("{}", l);
                    }
                }
            }
            // get function names and arguments
            if line.contains("public") || line.contains("private") {}

            // parse brackets contained in the line and update our scopes
            let function_closed = SourceFile::parse_brackets(&line, &mut function_scope);
            let class_closeed = SourceFile::parse_brackets(&line, &mut class_scope);

            // if function scope is closed we have finished parsing a function
            if function_closed {
                for line in &function_buf {
                    println!("{}", line);
                }
            }
        }
    }
}
