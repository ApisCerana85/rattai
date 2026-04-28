pub mod tty;

#[cfg(feature = "whisper")]
pub mod whisper;

#[cfg(feature = "piper")]
pub mod piper;
//#[cfg(feature = "parakeet")]
//pub mod parakeet;

use crate::ControlActions;

#[derive(Debug)]
pub enum InputData {
    String(String)
}

pub trait Input {
    fn get_input(&mut self) -> Result<InputData, OutputData>;
}

#[derive(Debug)]
pub enum OutputData {
    String(String),
    Error(OutputError),
    Action(ControlActions),
}

impl From<InputData> for OutputData {
    fn from(value: InputData) -> Self {
        match value {
            InputData::String(d) => OutputData::String(d.to_string()),
            _ => OutputData::String(format!("!!! InputData type not possible"))
        }
    }
}

#[derive(Debug)]
pub enum OutputError {
    Other(String),
    Unsuported
}

pub trait Output {
    fn output(&mut self, data: &OutputData) -> Result<(), OutputError>;
}