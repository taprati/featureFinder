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

    // chromosome
    #[structopt(short = "c", long = "chromosome")]
    chrom: String,

    // feature type
    #[structopt(short = "t", long = "type")]
    feature: String,

    // Start index
    #[structopt(short = "s", long = "start")]
    start: u32, 

    // End index
    #[structopt(short = "e", long = "end")]
    end: u32,
   
}


fn main() {
    // Parse args 
    let cli = Cli::from_args();

    // Print arg to debug
    println!("{:?}", cli);
    
    let chromosome = &cli.chrom;
    let feature = &cli.feature;
    let start = &cli.start;
    let end = &cli.end;

}


