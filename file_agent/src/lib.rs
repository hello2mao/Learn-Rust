#[macro_use]
extern crate log;

use serde::{Deserialize, Deserializer, Serialize};

mod types;
pub use types::{FileAgentRequest, HandleFileCommand, HandleFileInfo};
mod agent;
pub use agent::ocall_handle_file_request;