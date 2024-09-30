use generate::Generate;

mod args;
mod commands;
mod file_utils;
mod generate;
mod gui;

fn main() -> eframe::Result {
    //
    // let data = generate::Generate::read_entries(&inputfile, &outputfile, days, args.reset).unwrap();

    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Days Viewer", // Window title
        native_options,
        Box::new(|_| {
            let generate = Generate::read_entries("input.txt", "output.txt", 7, false).unwrap();
            let _ = generate.write_file("output.txt");
            let app = gui::DinnerViewer::new(generate);
            Ok(Box::new(app))
        }),
    )
    //commands::run();
    // Output the results
    /* data.print_output(); */
    /* let _ = data.write_file(&outputfile); */
    //    print_output(&selected_entries);
    //    write_file(&outputfile, &selected_entries);
}
