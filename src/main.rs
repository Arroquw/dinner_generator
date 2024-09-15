use clap::Parser;

mod args;
mod commands;
mod file_utils;
mod generate;

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

fn main() {
    let args = Args::parse();

    let inputfile = args.inputfile.unwrap_or("input.txt".to_owned());
    let outputfile: String = args.outputfile.unwrap_or("output.txt".to_owned());
    let days = args.days.unwrap_or(7_usize);

    let data = generate::Generate::read_entries(&inputfile, &outputfile, days, args.reset).unwrap();

    // Output the results
    println!("Output: ");
    data.print_output();
    let _ = data.write_file(&outputfile);
    //    print_output(&selected_entries);
    //    write_file(&outputfile, &selected_entries);
}
