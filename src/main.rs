use std::{env, fs::File, io::Write};

mod scardoc;
mod scarfile;

fn main() {
    
    println!("Scardoc generator");

    let second_arg: Option<String> = env::args().nth(1);
    match second_arg {
        Some(val) => {
            println!("Generating scardoc for directory: {}", val);
            match scardoc::generate_scardoc(val) {
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
        },
        None => {
            println!("No target directory was provided");
            return;
        }
    }

}

fn save_to_json(doc: &scardoc::ScarDoc, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(doc)?;
    let mut file = File::create(filepath)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
