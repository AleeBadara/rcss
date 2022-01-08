#[derive(Debug)]
#[allow(dead_code)]
pub struct Config {
    input_filename: String,
    output_filename: String,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("You must provide the input scss filename and the desired output filename");
        }
        Ok(Config {
            input_filename: args[1].clone(),
            output_filename: args[2].clone(),
        })
    }
}
