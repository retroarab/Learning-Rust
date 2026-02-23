use std::{
    env,
    error::Error,
    fs,
    process::{self, exit},
};
struct Config {
    query: String,
    file_path: String,
}
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("no mr. fish");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}
fn search<'a>(searched_word: &str, file_text: &'a str) -> Vec<&'a str> {
    file_text
        .lines()
        .filter(|line| line.contains(searched_word))
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Erro parsing sir, good day {}", err);
        process::exit(1);
    });
    run(config).unwrap_or_else(|err| {
        println!("An error occured {}", err);
        process::exit(1)
    })
}
