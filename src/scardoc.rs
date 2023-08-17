use walkdir::{DirEntry, WalkDir};
use std::path::Path;
use serde::{Serialize, Deserialize};

use crate::scarfile::*;

#[derive(Serialize, Deserialize)]
pub struct ScarDoc {
    pub sources: Vec<ScarSourceFile>
}

pub fn generate_scardoc<P: AsRef<Path>>(dir_path: P) -> Result<ScarDoc, &'static str> {
    let mut results = Vec::new();

    for entry in WalkDir::new(dir_path) {
        let entry = entry.map_err(|_| "Failed to access directory entry")?;
        if is_scar_file(&entry) {
            let file_path = entry.path().to_str().unwrap().to_string();
            print!("Reading scar file: {}", file_path);
            match get_scar_sourcefile(file_path) {
                Err(e) => {
                    println!(" ... failed");
                    eprintln!("\tFailed reading scar file with error: {}", e)
                },
                Ok(src) => {
                    println!(" ... ok");
                    if src.functions.len() > 0 {
                        println!("\tSuccessfully read {} scardoc functions", src.functions.len());
                    }
                    results.push(src)
                }
            }
        }
    }

    Ok(ScarDoc { sources: results })
}

fn is_scar_file(entry: &DirEntry) -> bool {
    entry.file_type().is_file() && entry.path().extension() == Some("scar".as_ref())
}

mod tests {

    #[test]
    fn can_get_scardoc() {
        let result = super::generate_scardoc("scar");
        assert_eq!(result.is_ok(), true);
        let scardoc = result.unwrap();
        assert_eq!(scardoc.sources.len() > 0, true)
    }

}
