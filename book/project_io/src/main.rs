use std::env;
fn examine_args(arguments: &Vec<String>) {
    if arguments.len() < 2 {
        println!("Usage is -- search file")
    }
    if arguments.contains(&"-help".to_string()) {
        println!("Help with what ?");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args); // this transfers them ?
    examine_args(&args); // if at any point we want to change the usage showing or the
    // help or anything this is done in one place only. TODO Implement rest
}
