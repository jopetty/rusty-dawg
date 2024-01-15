use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read,Write};
use std::convert::AsRef;
use std::path::Path;

#[derive(Deserialize, Serialize)]
pub struct CdawgMetadata {
    source: usize,  // Index of source node.
    sink: usize,  // Index of sink node.
    e: usize,  // Number of nodes in graph.
}

impl CdawgMetadata {
    pub fn load_json<P: AsRef<Path>>(file_path: P) -> Result<Self> {
        let mut file = File::open(file_path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        Ok(serde_json::from_str(&data)?)
    }

    pub fn save_json<P: AsRef<Path>>(&self, file_path: P) -> Result<()> {
        let json_data = serde_json::to_string(self)?;
        let mut file = File::create(file_path)?;
        file.write_all(json_data.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_save_load_json() {
        let mut file = NamedTempFile::new().expect("Failed to create file");
        let path = file.path();
        let blob = CdawgMetadata {source: 42, sink: 35, e: 4};
        blob.save_json(path);

        let blob2 = CdawgMetadata::load_json(path).unwrap();
        assert_eq!(blob2.source, 42);
        assert_eq!(blob2.sink, 35);
        assert_eq!(blob2.e, 4);
    }
}