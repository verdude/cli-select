use std::env;
use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

// Returns the command line arguments
// not including the filename argument.
fn get_args() -> Vec<String> {
    // env::args Does not handle Unicode Values
    env::args().skip(1).collect()
}

// recursively traverse fs starting at path to find files that
// satisfy the cb predicate
// returns a list of strings of full paths
fn find(dir: &Path, cb: &dyn Fn(&DirEntry) -> bool) -> io::Result<Vec<String>> {
    let mut nlist = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                nlist.append(&mut find(&path, cb)?);
            }
            else if cb(&entry) {
                match entry.path().into_os_string().into_string() {
                    Ok(string) =>  {
                        nlist.push(String::from(string).clone());
                    }
                    _ => ()
                };
            }
        }
    }
    Ok(nlist)
}

fn print_options(options: &Vec<String>) {
    println!("Please choose one of the following:");

    let mut i = 1;
    for opt in options.iter() {
        println!("{}. {}", i, opt);
        i += 1;
    }
    println!();
}

// Returns the selected number
// or -1 if the input was invalid or unable to be parsed
fn select_option() -> i32 {
    let mut input: String = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) =>
            // read_line includes the newline in the new string
            // so we remove it with trim_end
            input.trim_end().parse().unwrap_or(-1),
        Err(_) => -1
    }
}

fn func2(f: &DirEntry, e: &str) -> bool {
    f.file_name().into_string().unwrap_or("".to_string()).ends_with(e)
}

fn func(f: &DirEntry) -> bool {
    [".mp4", ".mkv"].iter().any(|e| func2(f, e))
}

fn main() {
    let args = get_args();
    let default = String::from(".");
    let dir = args.first().unwrap_or(&default);

    let options: Vec<String> = match find(Path::new(dir.as_str()), &func) {
        Ok(entries) => entries,
        Err(msg) => {
            println!("{}: {}", "Whatup. bout to exit", msg);
            std::process::exit(1);
        }
    };

    if options.len() > 0 {
        print_options(&options);
        let option_num = select_option();
        let index = (option_num - 1) as usize;

        // Check if the selection is in the bounds
        if index < options.len() {
            println!("You selected: {}", options[index]);
        }
        else {
            println!("Invalid selection. {}", option_num);
        }
    }
}

