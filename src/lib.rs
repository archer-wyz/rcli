pub mod cli;
mod process;

pub use cli::{Base64SubCommand, CsvOpts, Opts, SubCommand, TextSubCommand};
pub use process::base64_ed::{process_base64_decode, process_base64_encode};
pub use process::csv_convert::process_csv;
pub use process::gen_pass::process_gen_pass;
pub use process::text_sv::{process_text_generate, process_text_sign, process_text_verify};
