pub mod cli;
mod process;

pub use cli::{Base64SubCommand, CsvOpts, HttpSubCommand, Opts, SubCommand, TextSubCommand};
pub use process::base64_ed::{process_base64_decode, process_base64_encode};
pub use process::csv_convert::process_csv;
pub use process::gen_pass::process_gen_pass;
pub use process::http_serve::process_http_serve;
pub use process::text_op::{
    process_text_decrypt, process_text_encrypt, process_text_generate, process_text_sign,
    process_text_verify,
};
