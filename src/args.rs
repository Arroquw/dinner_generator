use clap::{Args, ValueEnum};

#[derive(Args, Debug, Clone)]
pub struct FileArgs {
    /// input file which contains all possible options
    #[arg(short, long, value_name = "FILENAME")]
    input_file: Option<String>,
    /// output file to write the <day count> amount of options to
    #[arg(short, long, value_name = "FILENAME")]
    output_file: Option<String>,
}

impl FileArgs {
    pub fn input_file(&self) -> Option<String> {
        self.input_file.clone()
    }
    pub fn output_file(&self) -> Option<String> {
        self.output_file.clone()
    }
}

#[derive(Args, Debug)]
pub struct ShowArgs {
    #[arg(short, long)]
    output_path: String,
}

impl ShowArgs {
    pub fn output_path(&self) -> &str {
        &self.output_path
    }
}

#[derive(Args, Debug)]
pub struct GenerateArgs {
    #[command(flatten)]
    file_args: FileArgs,
    /// Amount of days to generate for (amount of entries in output file)
    #[arg(short, long, value_name = "COUNT")]
    days: Option<usize>,
    /// Whether to reinitialise (clear output and start anew)
    #[arg(short, long, default_value_t = false)]
    reset: bool,
}

impl GenerateArgs {
    pub fn file_args(&self) -> &FileArgs {
        &self.file_args
    }
    pub fn days(&self) -> Option<usize> {
        self.days
    }
    pub fn reset(&self) -> bool {
        self.reset
    }
}

#[derive(Args, Debug)]
pub struct EditArgs {
    #[command(flatten)]
    file_args: FileArgs,
    #[arg(long, short = 'x', value_enum)]
    edit_field: EditType,
    #[arg(long, short, value_name = "ENTRY", required_if_eq_any([
        ("edit_field", "add-to-input"),
        ("edit_field", "swap-entry"),
        ("edit_field", "remove-from-input"),
        ("edit_field", "input-entry"),
        ("edit_field", "entry")
    ]))]
    entry: Option<Vec<String>>,
}

impl EditArgs {
    pub fn file_args(&self) -> &FileArgs {
        &self.file_args
    }
    pub fn edit_field(&self) -> &EditType {
        &self.edit_field
    }

    pub fn entry(&self) -> Option<&Vec<String>> {
        self.entry.as_ref()
    }
}

#[derive(ValueEnum, Copy, Clone, Debug)]
pub enum EditType {
    AddToInput,
    RemoveFromInput,
    InputEntry,
    Entry,
    RegenEntry,
    SwapEntry,
}
