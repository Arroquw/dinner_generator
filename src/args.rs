use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// input file which contains all possible options
    #[arg(short, long, value_name = "Input file")]
    inputfile: Option<String>,
    /// output file to write the <day count> amount of options to
    #[arg(short, long, value_name = "Output file")]
    outputfile: Option<String>,
    /// Amount of days to generate for (amount of entries in output file)
    #[arg(short, long, value_name = "Day count")]
    days: Option<usize>,
    /// Whether to reinitialise (clear output and start anew)
    #[arg(short, long, default_value_t = false, value_name = "Reinitialise")]
    reset: bool,
}
