use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Json,
    Text,
    Xml,
    Yaml,
    Csv,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "json" => Ok(Self::Json),
            "text" | "txt" => Ok(Self::Text),
            "xml" => Ok(Self::Xml),
            "yaml" | "yml" => Ok(Self::Yaml),
            "csv" => Ok(Self::Csv),
            _ => Err(format!("Unknown format: {}. Supported formats: json, text, xml, yaml, csv", s)),
        }
    }

    pub fn extension(&self) -> &str {
        match self {
            Self::Json => "json",
            Self::Text => "txt",
            Self::Xml => "xml",
            Self::Yaml => "yaml",
            Self::Csv => "csv",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileData {
    pub path: PathBuf,
    pub content: Vec<u8>,
    pub size: u64,
}

impl FileData {
    pub fn new(path: PathBuf, content: Vec<u8>) -> Self {
        let size = content.len() as u64;
        Self { path, content, size }
    }
}

