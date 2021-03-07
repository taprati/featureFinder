//
// Feature Finder
// Author: Tyler Aprati
// 


// use std::path::PathBuf;
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
struct Cli {
   
    // Input file
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    input: std::path::PathBuf,

    // Output file, stdout if not present
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: Option<std::path::PathBuf>,
    
    // Number test
    #[structopt(short = "n", long = "number")]
    num: i32, 

}


fn main() {
    // Parse args 
    let cli = Cli::from_args();

    // Print arg to debug
    println!("{:?}", cli);
    
    let number = &cli.num;

    println!("number = {}", number);

}


