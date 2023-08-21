use std::{env, fs::File, io::{Write, BufReader}, path::Path};

use scardoc::ScarDoc;

mod scardoc;
mod scarfile;
mod scarenum;
mod scardocmerger;
mod scardump;

fn main() {
    
    println!("Scardoc generator");

    let args: Vec<String> = env::args().collect();

    match args.as_slice() {
        [_, flag, ..] if flag == "-m" => {
            let paths = &args[2..];
            if paths.len() < 2 {
                println!("Please provide at least two files to merge.");
                return;
            }
            main_merge_scardocs(paths)
        }
        [_, dir_path] => {
            main_generate_scardoc(dir_path.clone());
        }
        [_, flag, dir_path] if flag == "-g" => {
            main_generate_scardoc(dir_path.clone());
        }
        [_, flag, file_path] if flag == "-d" => {
            main_generate_scardoc_from_dump(file_path.clone());
        }
        _ => {
            println!("Invalid arguments. Usage: \n\
                     -m file1.json file2.json, ... \n\
                     [-g] path/to/some/dir");
        }
    }

}

fn main_generate_scardoc_from_dump(dump_file: String) {
    match scardump::read_scardump(dump_file) {
        Err(e) => eprintln!("{}", e),
        Ok(doc) => {
            match save_to_json(&doc, "dump_scardoc.json") {
                Err(e) => eprintln!("{}", e),
                Ok(_) => {}
            }
        }
    }
}

fn main_generate_scardoc(dir_path: String) {
    println!("Generating scardoc for directory: {}", dir_path);
    match scardoc::generate_scardoc(dir_path) {
        Err(e) => eprintln!("{}", e),
        Ok(doc) => {
            println!("Loaded scardoc");
            match save_to_json(&doc, "scardoc.json") {
                Err(e) => eprintln!("{}", e),
                Ok(_) => {
                    println!("Saved scardoc to {}", "scardoc.json");
                }
            }
        }
    }
}

fn save_to_json(doc: &scardoc::ScarDoc, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(doc)?;
    let mut file = File::create(filepath)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

fn main_merge_scardocs(scardocs: &[String]) {
    let mut docs = Vec::new();
    for scardoc in scardocs {
        if !Path::new(&scardoc).exists() {
            eprintln!("Failed finding scardoc '{}'", scardoc);
            return;
        }
        match load_scardoc_from_json(scardoc.to_string()) {
            Ok(s) => { docs.push(s); println!("Loaded scardoc {}", scardoc) },
            Err(e) => eprintln!("{}",e)
        }
    }
    if docs.len() <= 1 {
        println!("Not enough scardocs to merge");
        return;
    }
    let mut first = docs.get(0).unwrap();
    let mut result: Option<ScarDoc> = None;
    for i in 1..docs.len() {
        let second = docs.get(i).unwrap();
        let new_doc = scardocmerger::merge_scardoc(first, second);
        result = Some(new_doc);
        first = second
    }
    match result {
        None => eprintln!("Failed generating merged scardocs"),
        Some(s) => match save_to_json(&s, "merged_scardoc.json") {
            Err(e) => eprintln!("{}", e),
            Ok(_) => {
                println!("Saved merged scardoc to {}", "merged_scardoc.json");
            }
        }
    }
}

fn load_scardoc_from_json(file_path: String) -> Result<scardoc::ScarDoc, Box<dyn std::error::Error>> {

    let file = File::open(&file_path);
    if file.is_err() {
        return Err(Box::new(file.err().unwrap()))
    }

    let reader = BufReader::new(file.unwrap());

    match serde_json::from_reader(reader) {
        Err(e) => Err(Box::new(e)),
        Ok(s) => Ok(s)
    }
}
