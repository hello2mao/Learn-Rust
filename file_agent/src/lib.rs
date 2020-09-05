#[macro_use]
extern crate log;

mod types;
pub use types::{FileAgentRequest, HandleFileCommand, HandleFileInfo};
mod agent;
pub use agent::ocall_handle_file_request;