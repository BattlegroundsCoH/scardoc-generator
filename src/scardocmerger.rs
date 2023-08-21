use std::collections::HashMap;

use crate::{scardoc::{ScarDoc, categorise_functions}, scarfile::{ScarFunction, ScarSourceFile}};

pub fn merge_scardoc(main: &ScarDoc, second: &ScarDoc) -> ScarDoc {
    let mut funcs = HashMap::new();
    for category in main.categories.as_slice() {
        for func in category.category_functions.as_slice() {
            funcs.insert(func.name.clone(), func.clone());
        }
    }
    for category in second.categories.as_slice() {
        for func in category.category_functions.as_slice() {
            funcs.entry(func.name.clone())
            .or_insert_with(|| { println!("Introducing {}",  func.name.clone()); func.clone() })
            .merge_with(&func);
        }
    }
    let temp_source_file = vec![ScarSourceFile{
        source_name: String::from("temp"),
        functions: funcs.into_values().collect()
    }];
    let categories = categorise_functions(temp_source_file);
    ScarDoc { categories, enums: Vec::new(), globals: Vec::new() }
}

impl ScarFunction {
    pub fn merge_with(&mut self, other: &Self) {
        println!("Merging function {}", self.name)
        // TODO: Define the merge logic, e.g.:
        // self.some_field = self.some_field.or(other.some_field.clone());
        // self.another_field = other.another_field; // if you want to overwrite
    }
}

mod tests {
    use std::collections::HashMap;

    use crate::{scardoc::{ScarDoc, ScarDocCategory}, scarfile::{ScarFunction, ScarParameter}};


    #[test]
    fn can_merge_unique_scardocs() {

        let scardoc_a = ScarDoc{
            categories: vec![ScarDocCategory{
                category_name: String::from("Util"),
                category_functions: vec![ScarFunction{
                    name: String::from("Util_GetPos"),
                    description_short: Some(String::from("this is a short description")),
                    description_extended: Vec::new(),
                    example: None,
                    return_description: None,
                    return_type: Some(String::from("Position")),
                    parameters:vec![
                        ScarParameter{arg_name:String::from("xpos"),arg_type:String::from("Real"), arg_description:None,arg_required:true},
                        ScarParameter{arg_name:String::from("zpos"),arg_type:String::from("Real"), arg_description:None,arg_required:true},
                        ScarParameter{arg_name:String::from("ypos"),arg_type:String::from("Real"), arg_description:None,arg_required:true}
                    ],
                    source_file: None,
                    groups: Vec::new()
                }]
            }],
            enums: vec![],
            globals: Vec::new()
        };
        let scardoc_b = ScarDoc{
            categories: vec![ScarDocCategory{
                category_name: String::from("Util"),
                category_functions: vec![ScarFunction{
                    name: String::from("Util_SetPos"),
                    description_short: Some(String::from("this is a short description")),
                    description_extended: Vec::new(),
                    example: None,
                    return_description: None,
                    return_type: Some(String::from("Position")),
                    parameters:vec![
                        ScarParameter{arg_name:String::from("pos"),arg_type:String::from("Position"), arg_description:None,arg_required:true},
                        ScarParameter{arg_name:String::from("xpos"),arg_type:String::from("Real"), arg_description:None,arg_required:true},
                        ScarParameter{arg_name:String::from("zpos"),arg_type:String::from("Real"), arg_description:None,arg_required:true},
                        ScarParameter{arg_name:String::from("ypos"),arg_type:String::from("Real"), arg_description:None,arg_required:true}
                    ],
                    source_file: None,
                    groups: Vec::new()
                }]
            }],
            enums: vec![],
            globals: Vec::new()
        };

        let merged = super::merge_scardoc(&scardoc_a, &scardoc_b);
        assert_eq!(merged.categories.len(), 1);
        assert_eq!(merged.categories.get(0).unwrap().category_name, String::from("Util"));
        assert_eq!(merged.categories.get(0).unwrap().category_functions.len(), 2)

    }

}
