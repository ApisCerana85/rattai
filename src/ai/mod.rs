pub mod chatgpt;

use crate::{
    ControlActions,
    io::{InputData, OutputData},
};

//message to send to ai engine
//pub enum AIMessage {
//    Text(&'static str),
//}

pub enum AIError {
    Unsuported
}

pub trait AIEngine {
    fn respond(&self, msg: &InputData) -> OutputData;
}

pub struct DummyEngine {}

impl DummyEngine {
    pub fn new() -> Self { Self{} }
}
impl DummyEngine { 
    fn parse_string(&self, s: String) -> OutputData {
        match s.as_str() {
            "quit" => OutputData::Action(ControlActions::Quit),
            &_ => OutputData::String(s),
        }
    }
}

impl AIEngine for DummyEngine {
    fn respond(&self, msg: &InputData) -> OutputData {
       match msg {
            InputData::String(d) => self.parse_string(d.to_string()),
            _ => OutputData::Error(crate::io::OutputError::Unsuported)
       } 
    }
}
