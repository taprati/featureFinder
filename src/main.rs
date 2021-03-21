//
// Feature Finder
// Author: Tyler Aprati
// 

use std::io::Write;
use structopt::StructOpt;


// TODO: fill out args with descriptions
// TODO: order args in help?
// Create command line structure
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
    let args = Cli::from_args();

    // TODO: remove this when finished 
    // Print arg to debug
    println!("{:?}", args);
    
    // put args into variables
    let input = std::fs::read_to_string(args.input)
                            .expect("File not Found!");
    let mut output = std::fs::File::create("output.txt")
                            .expect("Unable to create output file!");
    let chromosome = args.chrom;
    let feature = args.feature;
    let start = args.start;
    let end = args.end;

    // parse file 
    'outer: for line in input.lines() {
        // parse line by tabs
        let fields: Vec<&str> = line.split("\t").collect();

        // TODO: is there a better way to do this? / write to file?
        // TODO: make header optional?
        // check for header
        let head = &fields[0].chars().nth(0).unwrap();
        // if header add to output, then continue
        if head == &'#' {
            println!("{}", line);
            output.write_all(line.as_bytes())
                                .expect("Write failed!");
            continue 'outer ;
        }
        
        // parse fields into variables
        let chrom = fields[0].trim();
        let feat = fields[2].trim();
        let s = fields[3].trim().parse::<u32>().unwrap();
        let e = fields[4].trim().parse::<u32>().unwrap(); 

        // check line against criteria
        if chrom == chromosome {
            if feat == feature {
                if (e >= start) & (s <= end) {
                    println!("{}", line);
                    output.write(line)
                                    .expect("Write failed!");
                }
            }
        }
    }
}



