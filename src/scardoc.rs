use walkdir::{DirEntry, WalkDir};
use std::{path::Path, collections::HashMap};
use serde::{Serialize, Deserialize};

use crate::scarfile::*;
use crate::scarenum::*;

#[derive(Serialize, Deserialize)]
pub struct ScarDoc {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<ScarDocCategory>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub enums: Vec<ScarEnum>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub globals: Vec<ScarGlobal>
}

#[derive(Serialize, Deserialize)]
pub struct ScarDocCategory {
    pub category_name: String,
    pub category_functions: Vec<ScarFunction>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ScarGlobal {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub global_type: Option<String>
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
                        results.push(src)
                    }
                }
            }
        }
    }

    let categorised = categorise_functions(results)
        .into_iter()
        .filter(|x| !x.category_functions.is_empty())
        .collect();

    Ok(ScarDoc { categories: categorised, enums: Vec::new(), globals: Vec::new() })
    
}

fn is_scar_file(entry: &DirEntry) -> bool {
    entry.file_type().is_file() && entry.path().extension() == Some("scar".as_ref())
}

pub fn categorise_functions(sources: Vec<ScarSourceFile>) -> Vec<ScarDocCategory> {

    let mut map: HashMap<String, ScarDocCategory> = HashMap::new();

    for source in sources {
        for func in source.functions {
            let func_category = categorise_function(&func);
            map.entry(func_category.clone())
                .or_insert_with(|| ScarDocCategory {
                    category_name: func_category.clone(),
                    category_functions: Vec::new(),
                })
                .category_functions.push(func);
        }
    }

    map.into_values().collect()

}

fn categorise_function(func: &ScarFunction) -> String {
    match func.name.find('_') {
        Some(idx) => (func.name[..idx]).to_string(),
        None => match func.name.find(':') {
            Some(idx) => (func.name[..idx]).to_string(),
            None => if func.groups.len() > 0 {
                func.groups.get(0).unwrap_or(&func.name).to_string()
            } else {
                String::from("Other")
            }
        }
    }
}

mod tests {

    #[test]
    fn can_get_scardoc() {
        let result = super::generate_scardoc("scar");
        assert_eq!(result.is_ok(), true);
        let scardoc = result.unwrap();
        assert_eq!(scardoc.categories.len() > 0, true)
    }

}
