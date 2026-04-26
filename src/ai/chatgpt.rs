use crate::{
    ai::AIEngine
};
use serde::Deserialize;


// TODO
#[derive(Deserialize)]
pub struct ChatGPTConfig {
    pub api_key: String,
}

pub struct ChatGPT {
    api_key: String,
}

impl ChatGPT {}
