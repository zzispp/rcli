mod process;
mod cli;

pub use cli::{Opts,SubCommand,Base64SubCommand};
pub use process::{process_csv,process_genpass,process_decode,process_encode};