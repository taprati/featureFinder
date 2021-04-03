// 
// Module for the command line structure
//

use structopt::StructOpt;

// Create command line structure
#[derive(Debug, StructOpt)]
#[structopt(
    about = "This program parses gff files according to criteria",
    setting = structopt::clap::AppSettings::ColoredHelp,
    setting = structopt::clap::AppSettings::DisableVersion,
    setting = structopt::clap::AppSettings::DeriveDisplayOrder,
    setting = structopt::clap::AppSettings::ArgRequiredElseHelp,
    
)]
pub struct Cli {
    // Input file
    #[structopt(
        short = "i",
        long = "input",
        parse(from_os_str),
        help = "GFF file to search through."
    )]
    pub input: std::path::PathBuf,

    // Output file
    #[structopt(
        short = "o", 
        long = "output", 
        default_value = "featureFinderOutput.out",
        help = "Output file."
    )]
    pub output: String,

    // include header in output
    #[structopt(
        short = "k",
        long = "keepHeader",
        help = "Flag for including the input file header in the output. By default header is removed."
    )]
    pub header: bool,

    // chromosome
    #[structopt(
        short = "c",
        long = "chromosome",
        help = "Chromosome of interest"
    )]
    pub chrom: String,

    // feature type
    #[structopt(
        short = "t",
        long = "type",
        help = "The type of feature you want. (mRNA, gene, exon, etc)."
    )]
    pub feature: String,

    // Start index
    #[structopt(
        short = "s",
        long = "start",
        help = "The start index of the region to search."
    )]
    pub start: u32,

    // End index
    #[structopt(
        short = "e",
        long = "end",
        help = "The end index of the region to search."
    )]
    pub end: u32,
}

