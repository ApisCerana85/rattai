use serde::Deserialize;
use crate::io::{
    Output, OutputData, OutputError
};
use std::{
    io::{Read, Write},
    process::{Child, Command, Stdio}, thread,
};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

#[derive(Deserialize)]
pub struct PiperConfig {
    pub piper_path: String,
    pub model_path: String,
    pub model_config_path: String,
}

pub enum PiperError {
    Unsuported,
    Engine(String),
    Other(String),
    Cpal(String),
}
impl From<PiperError> for OutputError {
    fn from(value: PiperError) -> Self {
        match value {
            PiperError::Unsuported => OutputError::Unsuported,
            PiperError::Engine(s) => OutputError::Other(format!("engine error: {}", s)),
            PiperError::Other(s) => OutputError::Other(s.to_string()),
            PiperError::Cpal(s) => OutputError::Other(s.to_string()),
        }
    }
}

pub struct PiperOut {
    piper_process: Child,
    piper_stdout: std::process::ChildStdout,
    cpal_device: cpal::Device,
    cpal_config: cpal::SupportedStreamConfig,
}
impl PiperOut {
    fn start_cpal() -> Result<(cpal::Device, cpal::SupportedStreamConfig), PiperError> {
        println!("starting cpal!");
        let host = cpal::default_host();
        
        let device = host
            .default_output_device()
            .ok_or(PiperError::Cpal("no default output device".to_string()))?;
        
        let config = device
            .default_output_config()
            .map_err(|_| PiperError::Cpal("no default output config".to_string()))?;
        
        println!("output config: {:?}", config);
        
        //feed nothing into stream
        let stream = device.build_output_stream(
            &config.clone().into(), //chud clone!
            move |data: &mut [f32], _| {
                for sample in data.iter_mut() {
                    *sample = 0.0;
                }
            },
            |e| eprintln!("stream error: {}", e),
            None,
        ).map_err(|e| PiperError::Cpal(format!("failed to build stream: {}", e)))?;
        stream.play().map_err(|e| PiperError::Cpal(format!("cant play stream: {}", e)))?;

        Ok((device, config))
    }

    pub fn new(c: PiperConfig) -> Result<Self, PiperError> {
        let (cpal_device, cpal_config) = Self::start_cpal()?;

        let mut piper_process = Command::new(&c.piper_path)
            .args([
                "-m", &c.model_path,
                "-c", &c.model_config_path,
                "--output-raw"
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| PiperError::Engine(format!("{}", e)))?;
        let piper_stdout = piper_process.stdout.take()
            .ok_or(PiperError::Other("cannot get piper stdout".to_string()))?;
        Ok(Self {
            piper_process,
            piper_stdout,
            cpal_device,
            cpal_config,
        })
    }

    fn speak(&mut self, text: &String) -> Result<(), PiperError> {
        self.piper_process.stdin
            .as_mut()
            .ok_or(PiperError::Engine(format!("could't get piper stdin")))?
            .write_all(text.as_bytes())
            .map_err(|e| PiperError::Engine(format!("could't write to piper stdin: {}", e)))?;
        println!("{}", text);

        //self.piper_process.wait_with_output().unwrap();
        let (tx, rx) = std::sync::mpsc::channel::<f32>();
        
        //ngl this is completly vibe coded. i am tired
        thread::spawn(move || {
            let mut buf = [0u8; 2048];

            while let Ok(n) = self.piper_stdout.read(&mut buf) {
                if n == 0 { break; }

                for chunk in buf[..n].chunks_exact(2) {
                    let sample = i16::from_le_bytes([chunk[0], chunk[1]]);
                    let sample_f32 = sample as f32 / 32768.0;

                    tx.send(sample_f32).ok();
                }
            }
        });

        let stream = self.cpal_device.build_output_stream(
            &self.cpal_config.clone().into(), //chud clone!
            move |data: &mut [f32], _| {
                for sample in data.iter_mut() {
                    *sample = rx.try_recv().unwrap_or(0.0);
                }
            },
            |e| eprintln!("real stream error: {}", e),
            None
        ).map_err(|e| PiperError::Cpal(format!("failed to build real stream: {}", e)))?;

        stream.play().map_err(|e| PiperError::Cpal(format!("failed to play real stream: {}", e)))?;
        //let output = self.piper_process.wait_with_output().unwrap();
        //println!("{}", String::from_utf8_lossy(&output.stderr));

        Ok(())
    }
}
impl Output for PiperOut {
    fn output(&mut self, data: &OutputData) -> Result<(), OutputError> {
        match data {
            OutputData::String(s) => Ok(self.speak(s)?),
            OutputData::Error(e) => Ok(self.speak(&format!("error: {:?}", e))?),
            _ => Err(PiperError::Unsuported.into()),
        }
    }
}