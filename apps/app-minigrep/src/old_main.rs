use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep::search;
use minigrep::search_case_insensitive;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args); // need to add `&` here, to prevent move, as injected owned parameter will be moved, hence the borrowed version


    // let default_value = String::from("No option given");
    // let query = args.get(1).unwrap_or(&default_value);
    // let file_name = args.get(2).unwrap_or(&default_value);

    // let config = Config::new(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = run(&config) {
        println!("Application error: {e}");
        process::exit(1);
    }

    // replace by the run function
    // dbg!(&args);
    // println!("The query is \'{}\' and the filename is \'{}\'", config.query, config.file_name);
    // let content = fs::read_to_string(&config.file_name).expect("Should have been able to read the file");
    // println!("Text in file {} is \n: {}", config.file_name, content);

}

struct Config {
    query: String,
    file_name: String,
    ignore_case: bool,
}

impl Config {

    fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments")
        }

        let query = args[1].clone();
        let file_name = args[2].clone();

        //capture environment variable
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {query, file_name, ignore_case})
    }

    // replace by build function, as developer prefer not to throw error through a function name `new`
    // fn new(args: &[String]) -> Self {
    //     if args.len() > 3 {
    //         panic!("Not enough arguments");
    //     }
    //     let query = args[1].clone();
    //     let file_name = args[2].clone();
    //     Config {query, file_name}
    // }
}

// replaced by Config::new
// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let file_name = args[2].clone();
//     Config {query, file_name}
// }

// a function to read file contents and print the text
fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    // let content = fs::read_to_string(&config.file_name).expect("Should have been able to read the file");
    let contents = fs::read_to_string(&config.file_name)?;
    
    let result = if config.ignore_case {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in result {
        println!("{line}");
    }
    Ok(())
}
