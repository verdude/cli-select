use std::env;
use std::io;

// Returns the command line arguments
// not including the filename argument.
fn get_args() -> Vec<String> {
    // env::args Does not handle Unicode Values
    env::args().skip(1).collect()
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
        Ok(_) => {
            // read_line includes the newline in the new string
            // so we remove it with trim_end
            input.trim_end().parse().unwrap_or(-1)
        }
        Err(_) => -1
    }
}

fn main() {
    let options = get_args();

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

