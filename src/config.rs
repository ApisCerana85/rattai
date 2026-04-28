use serde_json::{Value};

use crate::{
    ai::{
        AIEngine, DummyEngine, chatgpt::ChatGPTConfig
    }, io::{
        Input, Output, OutputError, tty::{
            TTYIn, TTYInputConfig,
            TTYOut, TTYOutputConfig,
        }
    }
};

#[cfg(feature = "piper")]
use crate::io::piper::{PiperOut, PiperConfig};

#[derive(Debug)]
pub enum ConfigError {
    Parse(String),
    File
}
impl From<serde_json::Error> for ConfigError {
    fn from(e: serde_json::Error) -> ConfigError {
        use serde_json::error::Category;
        match e.classify() {
            Category::Io => ConfigError::File,
            Category::Syntax => ConfigError::Parse(format!("Syntax Error at {}:{}", e.line(), e.column())),
            Category::Data => ConfigError::Parse(format!("Invalid Data at {}:{} (make sure you have every field)", e.line(), e.column())),
            Category::Eof => ConfigError::Parse(format!("Unexpected EoF {}:{}", e.line(), e.column())),
        }
    }
}


pub enum AIConfig {
    ChatGPT(ChatGPTConfig),
    DummyEngine,
}
impl AIConfig {
    fn from_json(v: &Value) -> Result<Self, ConfigError> {
        match &v["type"] {
            Value::String(t) if *t == "dummy".to_string() => Ok(Self::DummyEngine),
            Value::Null => Err(ConfigError::Parse("no ai type field!".to_string())),
            _ => Err(ConfigError::Parse("unsuported ai type".to_string()))
        }
    }
}

impl From<AIConfig> for Box<dyn AIEngine> {
    fn from(c: AIConfig) -> Box<dyn AIEngine> {
        match c {
            AIConfig::ChatGPT(_) => unimplemented!(),
            AIConfig::DummyEngine => Box::new(DummyEngine::new())
        }
    }
}

pub enum InputConfig {
   TTY(TTYInputConfig)
}

impl InputConfig {
    fn from_json(v: &Value) -> Result<Self, ConfigError> {
        match v["type_in"]
            .as_str()
            .ok_or(ConfigError::Parse("couldn't find IO type".to_string()))? 
            {
            "tty" => {
                Ok(
                    Self::TTY(
                        serde_json::from_value(v["config_in"].clone())? //high cortisol clone
                    )
                )
            },
            &_ => Err(ConfigError::Parse("invaild Output type".to_string())),
        }
    }
}
impl From<InputConfig> for Box<dyn Input> {
    fn from(c: InputConfig) -> Box<dyn Input> {
        match c {
            InputConfig::TTY(c) => Box::new(TTYIn::new(c)),
        }
    }
}

pub enum OutputConfig {
   TTY(TTYOutputConfig),
   #[cfg(feature = "piper")]
   Piper(PiperConfig)
}

impl OutputConfig {
    fn from_json(v: &Value) -> Result<Self, ConfigError> {
        match v["type_out"]
            .as_str()
            .ok_or(ConfigError::Parse("couldn't find IO type".to_string()))? 
            {
            "tty" => {
                Ok(Self::TTY(
                        serde_json::from_value(v["config_out"].clone())? //high cortisol clone
                ))
            },
            #[cfg(feature = "piper")]
            "piper" => {
                Ok(Self::Piper(
                    serde_json::from_value(v["config_out"].clone())?
                ))
            }
            &_ => Err(ConfigError::Parse("invaild Output type (make sure you have correct features enabled)".to_string())),
        }
    }
}
impl From<OutputConfig> for Result<Box<dyn Output>, OutputError> {
    fn from(c: OutputConfig) -> Result<Box<dyn Output>, OutputError> {
        match c {
            OutputConfig::TTY(c) => Ok(Box::new(TTYOut::new(c))),
            #[cfg(feature = "piper")]
            OutputConfig::Piper(c) => Ok(Box::new(PiperOut::new(c)?))
        }
    }
}


pub struct Config {
    pub ai: AIConfig,
    pub input: InputConfig,
    pub output: OutputConfig,
}

impl Config {
    pub fn from_file(path: &'static str) -> Result<Self, ConfigError> {
        let file = std::fs::File::open(path).map_err(|_| ConfigError::File)?;

        let json: Value = serde_json::from_reader(file)
            .map_err(|e| ConfigError::Parse(format!("{} : {}", e.line(), e.column())))?;

        let ai = AIConfig::from_json(&json["AI"])?;
        let input = InputConfig::from_json(&json["IO"])?;
        let output = OutputConfig::from_json(&json["IO"])?;

        return Ok(Self {ai, input, output})
    }
}
