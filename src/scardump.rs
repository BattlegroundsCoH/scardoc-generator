use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};

use regex::Regex;

use crate::{scardoc::{ScarDoc, categorise_functions, ScarGlobal}, scarfile::{ScarFunction, ScarSourceFile}, scarenum::{ScarEnum, ScarEnumValue}};

const MODE_SCARDOC_UNDEFINED:i32 = 0;
const MODE_SCARDOC_FUNCTIONS:i32 = 1;
const MODE_SCARDOC_GLOBALS:i32=2;
const MODE_SCARDOC_ENUMS:i32=3; // in dump they're refered to as unknowns

struct ScardumpUnknown {
    pub is_enum:bool,
    pub vals: Vec<String>
}

pub fn read_scardump(dump_file: String) -> Result<ScarDoc, String> {

    let file = File::open(&dump_file);
    if file.is_err() {
        return Err(String::from("Failed opening file: ")+&dump_file)
    }

    // Compile the regex
    let re = Regex::new(r"(\w+)(\[.*?\])?=(\w+)\((\d+)\)").unwrap();

    let reader = BufReader::new(file.unwrap());
    let mut mode = MODE_SCARDOC_UNDEFINED;

    let mut funcs = Vec::new();
    let mut globals = Vec::new();
    let mut unknowns = Vec::new();

    for line in reader.lines() {
        match line  {
            Err(_) => return Err(String::from("Failed reading line of dump file")),
            Ok(ln) => {
                if ln.eq_ignore_ascii_case("[ScarDoc:Functions]") {
                    mode = MODE_SCARDOC_FUNCTIONS
                } else if ln.eq_ignore_ascii_case("[ScarDoc:Globals]") {
                    mode = MODE_SCARDOC_GLOBALS
                } else if ln.eq_ignore_ascii_case("[ScarDoc:Unknowns]") {
                    mode = MODE_SCARDOC_ENUMS
                } else {
                    match mode {
                        MODE_SCARDOC_FUNCTIONS => {
                            funcs.push(read_scarfunction(ln))
                        }
                        MODE_SCARDOC_GLOBALS => {
                            match read_global(ln) {
                                None => {},
                                Some(v) => globals.push(v)
                            }
                        }
                        MODE_SCARDOC_ENUMS => {
                            match read_unknowns(ln, &re) {
                                None => {},
                                Some(v) => {
                                    unknowns.push(v)
                                }
                            }   
                        }
                        _ => return Err(String::from("unknown mode given"))
                    }
                }
            }
        }
    }

    let categories = categorise_functions(vec![ScarSourceFile{
        source_name: String::from("temp"),
        functions: funcs
    }]);

    let enums = map_unknowns_to_enum(unknowns);

    Ok(ScarDoc{categories, enums, globals})

}

fn read_scarfunction(ln: String) -> ScarFunction {
    ScarFunction { 
        name: ln, 
        description_short: None, 
        description_extended: Vec::new(), 
        example: None, 
        return_description: None, 
        return_type: None, 
        parameters: Vec::new(), 
        source_file: None, 
        groups: Vec::new() 
    }
}

fn read_global(ln: String) -> Option<ScarGlobal> {
    match ln.find("=") {
        None => None,
        Some(idx) => {
            let k = &ln[..idx];
            let v = &ln[idx+1..];
            Some(ScarGlobal{
                name:k.to_string(),
                value:Some(v.to_string()),
                description: None,
                global_type: None
            })
        }
    }
}

fn read_unknowns(ln: String, re: &Regex) -> Option<ScardumpUnknown> {
    if let Some(captures) = re.captures(ln.as_str()) {
        return Some(ScardumpUnknown { is_enum: true, vals: vec![
            String::from(captures.get(1).unwrap().as_str()), 
            String::from(captures.get(3).unwrap().as_str()), 
            String::from(captures.get(4).unwrap().as_str())
        ] });
    }
    None
}

fn map_unknowns_to_enum(u:Vec<ScardumpUnknown>) -> Vec<ScarEnum> {
    let mut map: HashMap<String, Vec<(String,String)>> = HashMap::new();
    for entry in u {
        if entry.is_enum {
            let enum_value_name = entry.vals.get(0).unwrap();
            let enum_name = entry.vals.get(1).unwrap();
            let enum_value_number = entry.vals.get(2).unwrap();
            map.entry(enum_name.clone())
            .or_insert_with(|| vec![(enum_value_name.to_string(), enum_value_number.to_string())])
            .push((enum_value_name.to_string(), enum_value_number.to_string()))
        }
    }
    map.into_iter().map(|x| {
        let values = x.1.into_iter().map(|y| ScarEnumValue{ 
            name: y.0, 
            value: Some(y.1)
        }).collect();
        ScarEnum{
            name: x.0,
            values
    }}).collect()
}
