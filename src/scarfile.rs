use std::{path::Path, fs::File, io::{BufReader, BufRead}};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ScarSourceFile {
    pub source_name: String,
    pub functions: Vec<ScarFunction>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ScarFunction {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description_short: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub description_extended: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_type: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ScarParameter>,
    //#[serde(skip_serializing)]
    pub source_file: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<String>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ScarParameter {
    pub arg_name: String,
    pub arg_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arg_description: Option<String>,
    pub arg_required: bool
}

pub fn get_scar_sourcefile(file_path: String) -> Result<ScarSourceFile, String> {

    // Ensure path exists
    if !Path::new(&file_path).exists() {
        return Result::Err("file already exists".to_string());
    }

    // Open the file
    let file = File::open(&file_path);
    if file.is_err() {
        return Result::Err("file failed to open for reading".to_string());
    }

    // Collect functions
    match get_scar_functions(file.unwrap(), &file_path) {
        Err(e) => Err(e),
        Ok(funcs) => Ok(ScarSourceFile{
            source_name: file_path,
            functions: funcs
        })
    }

}

fn get_scar_functions(file: File, scar_source: &String) -> Result<Vec<ScarFunction>, String> {

    let reader = BufReader::new(file);
    let mut funcs: Vec<ScarFunction> = Vec::new();

    let mut doc_data: Vec<String> = Vec::new();
    for line in reader.lines() {
        match line {
            Err(_) => return Err("failed to read line".to_string()),
            Ok(ln) => {
                if ln.starts_with("--? ") {
                    let content = (&ln[3..]).trim();
                    doc_data.push(content.to_string());
                } else if ln.trim().starts_with("function") {
                    if doc_data.len() == 0 {
                        continue;
                    }
                    let scardoc_data = doc_data.clone();
                    match get_scar_function(ln, scardoc_data) {
                        Err(e) => if e.starts_with("fatal:") { return Err(e) } else { println!("{}", e) },
                        Ok(mut f) => {
                            f.source_file = Some(scar_source.clone());
                            funcs.push(f)
                        }
                    }
                    doc_data.clear();
                } else {
                    doc_data.clear()
                }
            }
        }
    }

    Ok(funcs)

}

fn get_scar_function_name(ln: String) -> Option<String> {
    let start = ln.find(' ')?+1;
    let end = ln.find('(')?;
    if start >= end {
        return None;
    }
    return Some((&ln[start..end]).trim().to_string());
}

fn get_parameters(args: String, mandatory: bool) -> Option<Vec<ScarParameter>> {
    let by_comma = args.split(',');
    let mut parameters = Vec::new();
    let mut arg_index = 1;
    for p in by_comma {
        let pp = p.trim();
        match pp.find(' ') {
            None => {
                parameters.push(ScarParameter { 
                    arg_name: if pp == "..." { String::from("...") } else { format!("arg{}", arg_index) }, 
                    arg_type: if pp == "..." { String::from("Any") } else { pp.to_string() },
                    arg_description: None, 
                    arg_required: mandatory 
                })
            },
            Some(idx) => {
                let ty_name = &pp[..idx];
                let pa_name = &pp[idx+1..];
                parameters.push(ScarParameter { 
                    arg_name: pa_name.to_string(), 
                    arg_type: ty_name.to_string(), 
                    arg_description: None, 
                    arg_required: mandatory
                })
            }
        }
        arg_index+=1;
    }
    Some(parameters)
}

fn get_scar_function_args(ln: String) -> Option<Vec<ScarParameter>> {
    match ln.find('[') {
        None => get_parameters(ln, true),
        Some(idx) => {
            let mut mandatory_section = (&ln[..idx]).trim_end();
            if mandatory_section.ends_with(",") {
                mandatory_section = &mandatory_section[..(mandatory_section.len()-1)]
            }
            let mut mandatory = get_parameters(mandatory_section.to_string(), true)?;
            let mut optional_section = (&ln[idx+1..]).trim_end_matches("]");
            if optional_section.starts_with(",") {
                optional_section = &optional_section[1..];
            }
            match get_parameters(optional_section.to_string(), false) {
                None => None,
                Some(optionals) => {
                    mandatory.extend(optionals);
                    Some(mandatory)
                }
            }
        }
    }
}

fn get_scar_function(func_name: String, func_data: Vec<String>) -> Result<ScarFunction, String> {
    
    // Get complete script name
    let name = get_scar_function_name(func_name).ok_or("expected scar function name but found none")?;

    // Define function setup
    let mut description_short: Option<String> = None;
    let mut description_extended: Vec<String> = Vec::new();
    let mut return_type: Option<String> = None;
    let mut parameters: Vec<ScarParameter> = Vec::new();

    // Populate data
    let mut is_extended_desc = false;
    for dataline in func_data {
        if dataline.starts_with("@shortdesc") {
            description_short = Some((&dataline[10..]).trim().to_string());
            is_extended_desc = false
        } else if dataline.starts_with("@extdesc") {
            let content = (&dataline[8..]).trim().to_string();
            if content.len() > 0 {
                description_extended.push(content);
            }
            is_extended_desc = true
        } else if dataline.starts_with("@result") {
            return_type = Some((&dataline[7..]).trim().to_string());
            is_extended_desc = false
        } else if dataline.starts_with("@args") {
            let content = (&dataline[5..]).trim().to_string();
            let err_content = content.clone();
            let args = get_scar_function_args(content).ok_or(format!("failed to parse arguments directive '{}'", err_content))?;
            parameters.extend(args);
            is_extended_desc = false
        } else if is_extended_desc {
            description_extended.push(dataline)
        }
    }

    // Return result
    Ok(ScarFunction { 
        name: name, 
        description_short, 
        description_extended, 
        example: None, 
        return_description: None, 
        return_type, 
        parameters,
        source_file: None,
        groups: Vec::new()
    })
}

mod tests {

    #[test]
    fn can_get_function_names() {
        let names = [
            ("Util_ScarPos", "function Util_ScarPos(xpos, zpos, ypos)"), 
            ("Util_ScarPos", "function Util_ScarPos (xpos, zpos, ypos)"), 
            ("Util_ScarPos", "function      Util_ScarPos(xpos, zpos, ypos)"),
            ("", "		function(sgroupid, itemindex, squad)")
        ];
        for name in names {
            let s = name.1.to_string();
            if name.0 == "" {
                assert_eq!(super::get_scar_function_name(s).is_none(), true)
            } else {
                assert_eq!(super::get_scar_function_name(s).unwrap(), name.0.to_string())
            }
        }
    }

    #[test]
    fn can_get_function_parameters_names() {
        let args = [
            (vec![("Real", "xpos", true), ("Real", "zpos", true), ("Real", "ypos", true)], "Real xpos, Real zpos, Real ypos"),
            (vec![("Real", "xpos", true), ("Real", "zpos", true), ("Real", "ypos", false)], "Real xpos, Real zpos[, Real ypos]"),
            (vec![("LuaTable", "arg1", true)], "LuaTable"),
            (vec![("SyncWeaponID", "weapon", true), ("PlayerID", "player", false)], "SyncWeaponID weapon, [PlayerID player]"),
            (vec![("String", "race", true), ("String", "race2", false), ("Any", "...", false)], "String race[, String race2, ...]"),
        ];
        for arg in args {
            let result = super::get_scar_function_args(arg.1.to_string());
            assert_eq!(result.is_some(), true);
            let content = result.unwrap();
            assert_eq!(content.len(), arg.0.len());
            for i in 0..arg.0.len() {
                let result_arg = content.get(i).unwrap();
                let expected_arg = arg.0.get(i).unwrap();
                assert_eq!(result_arg.arg_type, expected_arg.0.to_string());
                assert_eq!(result_arg.arg_name, expected_arg.1.to_string());
                assert_eq!(result_arg.arg_required, expected_arg.2);
            }
        }
    }

    #[test]
    fn can_get_scar_sourcefile() {

        const SIMPLE_SCAR: &str = "scar/simple.scar";

        let result = super::get_scar_sourcefile(SIMPLE_SCAR.to_string());
        assert_eq!(result.is_ok(), true);

        let scarfile = result.unwrap();
        assert_eq!(scarfile.source_name, SIMPLE_SCAR);
        assert_eq!(scarfile.functions.len(), 1);

        // Assert found function
        let scarfn = scarfile.functions.get(0).unwrap();
        assert_eq!(scarfn.name, "Util_ScarPos");
        assert_eq!(scarfn.description_short, Some("Converts a 2D top down position to a 3D ScarPosition. returns Position, if y-height is nil, y-height = ground height, terrain ground or walkable".to_string()));
        assert_eq!(scarfn.description_extended.len(), 2); // We simply assert length and assume it worked

        // Assert arguments
        assert_eq!(scarfn.parameters.len(), 3);
        assert_eq!(scarfn.parameters.get(0).unwrap().arg_name, "xpos");
        assert_eq!(scarfn.parameters.get(0).unwrap().arg_type, "Real");
        assert_eq!(scarfn.parameters.get(0).unwrap().arg_required, true);
        assert_eq!(scarfn.parameters.get(1).unwrap().arg_name, "zpos");
        assert_eq!(scarfn.parameters.get(1).unwrap().arg_type, "Real");
        assert_eq!(scarfn.parameters.get(1).unwrap().arg_required, true);
        assert_eq!(scarfn.parameters.get(2).unwrap().arg_name, "ypos");
        assert_eq!(scarfn.parameters.get(2).unwrap().arg_type, "Real");
        assert_eq!(scarfn.parameters.get(2).unwrap().arg_required, true);

        // Assert return types
        assert_eq!(scarfn.return_type, Some("Position".to_string()));

    }

}
