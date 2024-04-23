pub mod cli;
mod process;

pub use cli::*;
use enum_dispatch::enum_dispatch;
pub use process::base64_ed::{process_base64_decode, process_base64_encode};
pub use process::csv_convert::process_csv;
pub use process::gen_pass::process_gen_pass;
pub use process::http_serve::process_http_serve;
pub use process::jwt::{process_jwt_sign, process_jwt_verify};
pub use process::text_op::{
    process_text_decrypt, process_text_encrypt, process_text_generate, process_text_sign,
    process_text_verify,
};

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExector {
    async fn execute(self) -> anyhow::Result<()>;
}
