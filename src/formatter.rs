use crate::types::{FileData, OutputFormat};
use serde_json::json;

pub struct Formatter {
    format: OutputFormat,
}

impl Formatter {
    pub fn new(format: OutputFormat) -> Self {
        Self { format }
    }

    pub fn format(&self, files: &[FileData]) -> Result<Vec<u8>, String> {
        match self.format {
            OutputFormat::Json => self.format_json(files),
            OutputFormat::Text => self.format_text(files),
            OutputFormat::Xml => self.format_xml(files),
            OutputFormat::Yaml => self.format_yaml(files),
            OutputFormat::Csv => self.format_csv(files),
        }
    }

    fn format_json(&self, files: &[FileData]) -> Result<Vec<u8>, String> {
        let json_data = json!({
            "files": files.iter().map(|f| {
                json!({
                    "path": f.path.display().to_string(),
                    "size": f.size,
                    "content": base64_encode(&f.content),
                })
            }).collect::<Vec<_>>(),
            "total_files": files.len(),
            "total_size": files.iter().map(|f| f.size).sum::<u64>(),
        });

        serde_json::to_vec_pretty(&json_data)
            .map_err(|e| format!("JSON serialization error: {}", e))
    }

    fn format_text(&self, files: &[FileData]) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();
        
        for file in files {
            let separator = format!("\n{}\n File: {}\n Size: {} bytes\n{}\n\n",
                "=".repeat(80), file.path.display(), file.size, "=".repeat(80));
            output.extend_from_slice(separator.as_bytes());
            output.extend_from_slice(&file.content);
            output.extend_from_slice(b"\n\n");
        }

        Ok(output)
    }

    fn format_xml(&self, files: &[FileData]) -> Result<Vec<u8>, String> {
        let mut output = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<files>\n");
        
        for file in files {
            output.push_str(&format!(
                "  <file path=\"{}\" size=\"{}\">\n    <content>{}</content>\n  </file>\n",
                escape_xml(&file.path.display().to_string()),
                file.size,
                base64_encode(&file.content)
            ));
        }

        output.push_str("</files>\n");
        Ok(output.into_bytes())
    }

    fn format_yaml(&self, files: &[FileData]) -> Result<Vec<u8>, String> {
        let mut output = String::from("files:\n");
        
        for file in files {
            output.push_str(&format!(
                "  - path: \"{}\"\n    size: {}\n    content: |\n      {}\n",
                file.path.display(),
                file.size,
                base64_encode(&file.content)
            ));
        }

        Ok(output.into_bytes())
    }

    fn format_csv(&self, files: &[FileData]) -> Result<Vec<u8>, String> {
        let mut output = String::from("path,size,content_base64\n");
        
        for file in files {
            output.push_str(&format!(
                "\"{}\",{},\"{}\"\n",
                file.path.display().to_string().replace('"', "\"\""),
                file.size,
                base64_encode(&file.content)
            ));
        }

        Ok(output.into_bytes())
    }
}

fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut output = Vec::with_capacity((data.len() + 2) / 3 * 4);
    
    for chunk in data.chunks(3) {
        let b1 = chunk[0];
        let b2 = chunk.get(1).copied().unwrap_or(0);
        let b3 = chunk.get(2).copied().unwrap_or(0);
        
        output.push(CHARS[(b1 >> 2) as usize]);
        output.push(CHARS[(((b1 & 0x03) << 4) | (b2 >> 4)) as usize]);
        output.push(if chunk.len() > 1 { CHARS[(((b2 & 0x0f) << 2) | (b3 >> 6)) as usize] } else { b'=' });
        output.push(if chunk.len() > 2 { CHARS[(b3 & 0x3f) as usize] } else { b'=' });
    }
    
    String::from_utf8(output).unwrap()
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

