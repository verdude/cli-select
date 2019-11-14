use std::env;

// Returns the command line arguments
// not including the filename argument.
fn get_args() -> Vec<String> {
    // env::args Does not handle Unicode Values
    env::args().skip(1).collect()
}

fn main() {
    let options = get_args();

    if options.len() > 0 {
        println!("Please choose one of the following:");

        let mut i = 1;
        for opt in options.iter() {
            println!("{}. {}", i, opt);
            i += 1;
        }
    }

}

