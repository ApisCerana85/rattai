pub mod tty;

#[cfg(feature = "whisper")]
pub mod whisper;
//#[cfg(feature = "parakeet")]
//pub mod parakeet;

use crate::ControlActions;

// #[derive(Copy, Clone)]
pub enum InputData {
    String(String)
}

pub trait Input {
    fn get_input(&mut self) -> Result<InputData, OutputData>;
}

pub enum OutputData {
    String(String),
    Error(&'static str),
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

pub trait Output {
    fn output(&mut self, data: &OutputData);
}

pub struct CustomIO {
    inpt: Box<dyn Input>,
    out: Box<dyn Output>,
}
impl Input for CustomIO {
    fn get_input(&mut self) -> Result<InputData, OutputData> { self.inpt.get_input() }
}
impl Output for CustomIO {
    fn output(&mut self, data: &OutputData) { self.out.output(data) }
}
