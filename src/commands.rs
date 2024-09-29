use crate::args::{EditArgs, EditType, FileArgs, GenerateArgs, ShowArgs};
use crate::file_utils::read_file;
use crate::generate::{Collection, Generate};
use clap::Parser;
use std::io;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub enum Command {
    #[command(name = "show", about = "Show the current entries")]
    Show(ShowArgs),
    #[command(name = "edit", about = "Edit an entry or the input")]
    Edit(EditArgs),
    #[command(name = "shuffle", about = "Shuffle the order of the current entries")]
    Shuffle(FileArgs),
    #[command(name = "generate", about = "Shuffle the order of the current entries")]
    Generate(GenerateArgs),
}

fn prepare_generate(file_args: &FileArgs, days: usize, reset: bool) -> Result<Generate, io::Error> {
    let inputfile = &file_args.input_file().unwrap_or("input.txt".to_owned());

    let outputfile = &file_args.output_file().unwrap_or("output.txt".to_owned());
    Generate::read_entries(inputfile, outputfile, days, reset)
}

fn generate(args: GenerateArgs) {
    let mut days = args.days().unwrap_or(7_usize);
    let mut generate = prepare_generate(args.file_args(), days, args.reset()).unwrap();
    let _ = generate.generate_days(&mut days);
    generate.print_output();
    let _ = generate.write_file(
        &args
            .file_args()
            .output_file()
            .unwrap_or("output.txt".to_owned()),
    );
}

fn edit(args: EditArgs) {
    println!("Edit: {:?}", args);

    let mut generate = prepare_generate(args.file_args(), 0, false).unwrap();
    generate.print_output();
    match args.edit_field() {
        EditType::AddToInput => {
            for i in args.entry().expect("Expected a value for entry to add!") {
                generate.add_to_pool(i.to_string());
            }
        }
        EditType::RemoveFromInput => {
            for i in args
                .entry()
                .expect("Expected a value for entries to remove!")
            {
                let _ = generate.remove_from_pool(i, &args.file_args().input_file().unwrap());
            }
        }
        EditType::Entry => {
            let _ = generate.edit_days_entry(
                generate
                    .find_entry(
                        Collection::Days,
                        &args.entry().expect("Please pass an entry to edit")[0],
                    )
                    .unwrap_or_else(|| {
                        panic!(
                            "Passed entry {} does not exist in current entries pool",
                            args.entry().unwrap()[0]
                        )
                    }),
                args.entry().unwrap_or_else(|| {
                    panic!(
                        "Please pass a new value for entry {}",
                        args.entry().unwrap()[0]
                    )
                })[1]
                    .to_owned(),
            );
        }
        EditType::RegenEntry => {
            let _ = generate.regenerate_entry(
                generate
                    .find_entry(Collection::Days, &args.entry().unwrap()[0])
                    .expect("Entry not found"),
            );
        }
        EditType::SwapEntry => {
            let old_pos = generate
                .find_entry(
                    Collection::Days,
                    &args
                        .entry()
                        .expect("Please pass an existing entry to be swapped")[0],
                )
                .unwrap_or_else(|| {
                    panic!(
                        "Passed entry {} does not exist in entries pool",
                        args.entry().unwrap()[0]
                    )
                });
            let new_pos = generate
                .find_entry(
                    Collection::Days,
                    &args.entry().expect("Please pass an existing entry to swap")[1],
                )
                .unwrap_or_else(|| {
                    panic!(
                        "Passed entry {} does not exist in entries pool",
                        args.entry().unwrap()[0]
                    )
                });
            let _ = generate.swap_days_entries(old_pos, new_pos);
        }
        EditType::InputEntry => {
            let _ = generate.edit_pool_entry(
                &args.entry().expect("Pool entry to edit not found")[0],
                args.entry().expect("Expected a value for pool entry")[1].to_owned(),
            );
        }
    }
}

fn shuffle(args: FileArgs) {
    println!("Shuffle: {:?}", args);
}

fn show(args: ShowArgs) {
    let outputfile = &args.output_path();

    println!("{:?}", read_file(outputfile));
}

pub fn run() {
    let args = Command::parse();

    match args {
        Command::Show(show_args) => {
            show(show_args);
        }
        Command::Shuffle(shuffle_args) => {
            shuffle(shuffle_args);
        }
        Command::Edit(edit_args) => {
            edit(edit_args);
        }
        Command::Generate(generate_args) => {
            generate(generate_args);
        }
    }
}
