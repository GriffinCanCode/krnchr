use crate::types::OutputFormat;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub struct OutputWriter;

impl OutputWriter {
    pub fn new() -> Self {
        Self
    }

    pub fn write(&self, data: &[u8], format: &OutputFormat) -> Result<String, String> {
        let filename = format!("crunched.{}", format.extension());
        let path = PathBuf::from(&filename);

        let mut file = File::create(&path)
            .map_err(|e| format!("Failed to create output file: {}", e))?;

        file.write_all(data)
            .map_err(|e| format!("Failed to write output: {}", e))?;

        Ok(path.display().to_string())
    }
}

