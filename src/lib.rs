pub mod cli;
mod process;

pub use cli::{Base64SubCommand, CsvOpts, Opts, SubCommand};
pub use process::base64_ed::{process_base64_decode, process_base64_encode};
pub use process::csv_convert::process_csv;
pub use process::gen_pass::process_gen_pass;
