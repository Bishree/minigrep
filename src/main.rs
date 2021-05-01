use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = pars_config(&args);

    println!("{:?}", args);

    println!("Searching for  {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(filename)
        .expect("something went wrong reading the file");
    println!("with text:\n{}", contents);

struct Config {
    query: String,
    filename: String,
}
impl Config {
    fn new (args:&[String]) -> Config {
        let query = args [1].clone();
        let filename = args [2].clone();

        Config {query, filename}
    }
}

//
// fn pars_config (args: &[String]) -> Config {
//     let query = args[1].clone();
//     let filename = args[2].clone();
//
//     Config {query, filename}
}


}