// 
// Module for the command line structure
//

use structopt::StructOpt;

// Create command line structure
#[derive(Debug, StructOpt)]
pub struct Cli {
    // Input file
    #[structopt(
        short = "i",
        long = "input",
        parse(from_os_str)
    )]
    pub input: std::path::PathBuf,

    // Output file
    #[structopt(
        short = "o", 
        long = "output", 
        default_value = "featureFinderOutput.out"
    )]
    pub output: String,

    // include header in output
    #[structopt(
        short = "k",
        long = "keepHeader"
    )]
    pub header: bool,

    // chromosome
    #[structopt(
        short = "c",
        long = "chromosome"
    )]
    pub chrom: String,

    // feature type
    #[structopt(
        short = "t",
        long = "type"
    )]
    pub feature: String,

    // Start index
    #[structopt(
        short = "s",
        long = "start"
    )]
    pub start: u32,

    // End index
    #[structopt(
        short = "e",
        long = "end"
    )]
    pub end: u32,
}

