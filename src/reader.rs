use crate::types::FileData;
use rayon::prelude::*;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct FileCollector;

impl FileCollector {
    pub fn new() -> Self {
        Self
    }

    pub fn collect(&self, path: &Path) -> Result<Vec<FileData>, String> {
        if !path.exists() {
            return Err(format!("Path does not exist: {}", path.display()));
        }

        let paths = self.discover_files(path)?;
        
        // Parallel file reading for maximum speed
        let files: Result<Vec<FileData>, String> = paths
            .par_iter()
            .map(|p| self.read_file(p))
            .collect();

        files
    }

    fn discover_files(&self, path: &Path) -> Result<Vec<PathBuf>, String> {
        if path.is_file() {
            return Ok(vec![path.to_path_buf()]);
        }

        let mut files = Vec::new();
        for entry in WalkDir::new(path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                files.push(entry.path().to_path_buf());
            }
        }

        files.sort();
        Ok(files)
    }

    fn read_file(&self, path: &Path) -> Result<FileData, String> {
        let mut file = fs::File::open(path)
            .map_err(|e| format!("Failed to open {}: {}", path.display(), e))?;

        let mut content = Vec::new();
        file.read_to_end(&mut content)
            .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

        Ok(FileData::new(path.to_path_buf(), content))
    }
}

