use std::{
    io,
    io::Write
};
use crate::io::{Input, InputData, Output, OutputData};
use serde::Deserialize;
use serde::Serialize;
#[derive(Deserialize, Serialize, Debug)]
pub struct TTYInputConfig {
    pub prompt: String,
}

pub struct TTYIn {
    stdin: std::io::Stdin,
    stdout: std::io::Stdout,
    prompt: String,
}
impl TTYIn {
    pub fn new(c: TTYInputConfig) -> Self {
        Self { 
            stdin: io::stdin(),
            stdout: io::stdout(),
            prompt: c.prompt,
        }
    }
}
impl Input for TTYIn {
    fn get_input(&mut self) -> Result<InputData, OutputData> {
        print!("{}", self.prompt);
        let mut input = String::new();
        self.stdout.flush();
        self.stdin.read_line(&mut input)
            .map_err(|_| OutputData::Error("read_line"))?;

        return Ok(InputData::String(input.trim_end().to_string()))
    }
}


#[derive(Deserialize)]
pub struct TTYOutputConfig {
    pub prompt: String,
}

pub struct TTYOut {
    stdout: std::io::Stdout,
    prompt: String,
}
impl TTYOut {
    pub fn new(c: TTYOutputConfig) -> Self {
        Self{
            stdout: io::stdout(),
            prompt: c.prompt,
        }
    }
}

impl Output for TTYOut {
    fn output(&mut self, data: &OutputData) {
        match data {
            OutputData::String(d) => println!("{} {}", self.prompt, d),
            OutputData::Error(e) => println!("ERR! {}", e),
            _ => {},
        }
    }
}
