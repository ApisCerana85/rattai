use rattai::{
    Controller,
    Error,
    ai::AIEngine,
    config::Config,
    io::{Input, Output, OutputError},
};

fn main() -> Result<(), Error> {
    //use rattai::io::tty::TTYInputConfig;
    //let i = TTYInputConfig {
    //    prompt: "hello: ".to_string()
    //};
    //dbg!(serde_json::to_value(&i).unwrap());
    let config = Config::from_file("config.json")?;

    let ai: Box<dyn AIEngine> = config.ai.into();

    let input: Box<dyn Input> = config.input.into();
    let output: Result<Box<dyn Output>, OutputError> = config.output.into();

    let mut controller = Controller::new(ai, input, output?); 

    while !controller.quit {
        controller.update()?;
    }
    Ok(())
}
