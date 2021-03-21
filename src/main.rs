//
// Feature Finder
// Author: Tyler Aprati
// 


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
    let args = Cli::from_args();

    // Print arg to debug
    println!("{:?}", args);
    
    // put args into variables
    let input = std::fs::read_to_string(args.input)
                            .expect("File not Found!");
    let chromosome = &args.chrom;
    let feature = &args.feature;
    let start = &args.start;
    let end = &args.end;

    // parse file 
    'outer: for line in input.lines() {
        // parse line by tabs
        let fields: Vec<&str> = line.split("\t").collect();

        // TODO: is there a better way to do this?
        // check for header
        let head = &fields[0].chars().nth(0).unwrap();
        // if header skip
        if head == &'#' {
            continue 'outer ;
        }
        
        // parse fields into variables
        let chrom = fields[0].trim();

        // TODO: make the whole comparison
        if chrom == chromosome {
            println!("{}", chrom);
        }

        //println!("{}", chrom);
            
    }

}



