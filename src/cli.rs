use crate::types::OutputFormat;
use std::env;
use std::path::PathBuf;

pub struct Args {
    pub path: PathBuf,
    pub format: OutputFormat,
}

impl Args {
    pub fn parse() -> Result<Self, String> {
        let args: Vec<String> = env::args().collect();

        if args.len() < 3 {
            return Err("Missing required arguments".to_string());
        }

        let path = PathBuf::from(&args[1]);
        let format = OutputFormat::from_str(&args[2])?;

        Ok(Self { path, format })
    }
}

