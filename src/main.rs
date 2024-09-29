mod args;
mod commands;
mod file_utils;
mod generate;
mod gui;

fn main() {
    //
    // let data = generate::Generate::read_entries(&inputfile, &outputfile, days, args.reset).unwrap();

    println!("Output: ");
    commands::run();
    // Output the results
    /* data.print_output(); */
    /* let _ = data.write_file(&outputfile); */
    //    print_output(&selected_entries);
    //    write_file(&outputfile, &selected_entries);
}
