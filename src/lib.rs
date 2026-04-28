pub mod config;
pub mod ai;
pub mod io;

use io::{Input, Output, OutputData};
use ai::{AIEngine};

#[derive(Debug)]
pub enum Error {
    ConfigError(config::ConfigError),
    OutputError(io::OutputError)
}
//use std::fmt;
//impl fmt::Debug for Error {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        f.
//    }
//}
impl From<config::ConfigError> for Error {
    fn from(e: config::ConfigError) -> Error {
        Self::ConfigError(e)
    }
}
impl From<io::OutputError> for Error {
    fn from(e: io::OutputError) -> Error {
        Self::OutputError(e)
    }
}

#[derive(Debug)]
pub enum ControlActions {
    Quit,
}

pub struct Controller {
    pub quit: bool,
    ai:       Box<dyn AIEngine>,
    input:    Box<dyn Input>,
    output:   Box<dyn Output>,
}
impl Controller {
    pub fn new(ai: Box<dyn AIEngine>, input: Box<dyn Input>, output: Box<dyn Output>) -> Self {
        Self {
            quit: false,
            ai,
            input,
            output
        }
    }

    fn parse_action(&mut self, a: ControlActions) {
        match a {
            ControlActions::Quit => self.quit = true,
        }
    }

    fn parse_response(&mut self, resp: OutputData) -> Result<(), Error> {
        match resp {
            OutputData::String(_) => Ok(self.output.output(&resp)?),
            OutputData::Action(action) => Ok(self.parse_action(action)),
            OutputData::Error(e) => Err(e.into()), //TODO error
        }
    }

    pub fn update(&mut self) -> Result<(), Error> {
        let ipt = self.input.get_input();
        let resp: OutputData = match ipt {
            Ok(msg) => self.ai.respond(&msg).into(),
            Err(e)  => e,
        };
        self.parse_response(resp)?;
        Ok(())
    } 
}
