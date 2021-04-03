//
// Feature Finder
// Author: Tyler Aprati
// 

// TODO: make output stdout if no file provided
// TODO: add license
// TODO: make all params optional?

mod cli;

use std::fs;
use std::io::Write;
use crate::cli::Cli;
use structopt::StructOpt;


fn main() {

    // Parse args 
    let args = Cli::from_args();

    // TODO: remove this when finished 
    println!("{:?}", args);

    // put args into variables
    let input = fs::read_to_string(args.input).expect("File not Found!");
    let mut output = fs::File::create(args.output).expect("Output failed!");
    let chromosome = args.chrom;
    let feature = args.feature;
    let start = args.start;
    let end = args.end;
    let header = args.header;

    // parse file 
    'outer: for line in input.lines() {
        // parse line by tabs
        let fields: Vec<&str> = line.split("\t").collect();

        // check for header
        let head = &fields[0].chars().nth(0).unwrap();
        // if header add to output, then continue
        if head == &'#' {
            if header == true {
                write!(output, "{}\n", line).expect("Failed to write!");
            }
            continue 'outer;
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
                    write!(output, "{}\n", line).expect("Failed to write!");
                }
            }
        }
    }
}

