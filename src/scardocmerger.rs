use std::collections::HashMap;

use crate::{scardoc::{ScarDoc, categorise_functions, ScarDocCategory, ScarGlobal}, scarfile::{ScarFunction, ScarSourceFile, ScarParameter}, scarenum::{ScarEnum, ScarEnumValue}};

pub fn merge_scardoc(main: &ScarDoc, second: &ScarDoc) -> ScarDoc {
    let categories = merge_scardoc_functions(main, second);
    let enums = merge_scardoc_enums(main, second);
    let globals = merge_scardoc_globals(main, second);
    ScarDoc { categories, enums, globals }
}

fn merge_scardoc_functions(main: &ScarDoc, second: &ScarDoc) -> Vec<ScarDocCategory> {
    let mut funcs = HashMap::new();
    for category in main.categories.as_slice() {
        for func in category.category_functions.as_slice() {
            funcs.insert(func.name.clone(), func.clone());
        }
    }
    for category in second.categories.as_slice() {
        for func in category.category_functions.as_slice() {
            funcs.entry(func.name.clone())
            .or_insert_with(|| { println!("Introducing function {}",  func.name.clone()); func.clone() })
            .merge_with(&func);
        }
    }
    let temp_source_file = vec![ScarSourceFile{
        source_name: String::from("temp"),
        functions: funcs.into_values().collect()
    }];
    categorise_functions(temp_source_file)
}

impl PartialEq for ScarParameter {
    fn eq(&self, other: &Self) -> bool {
        self.arg_name == other.arg_name && self.arg_type == other.arg_type && self.arg_description == other.arg_description
        && self.arg_required == other.arg_required
    }
}

impl ScarFunction {
    pub fn eq(&mut self, other: &Self) -> bool {
        self.name == other.name && self.description_extended == other.description_extended
        && self.description_short == other.description_short && self.example == other.example
        && self.groups == other.groups && self.return_description == other.return_description
        && self.return_type == other.return_type && self.source_file == other.source_file
        && self.parameters == other.parameters
    }
    pub fn merge_with(&mut self, other: &Self) {
        if self.eq(other) {
            return
        }
        println!("Merging function {}", self.name);
        if self.description_extended != other.description_extended {
            self.description_extended = other.description_extended.clone()
        }
        self.description_short = match (self.description_short.clone(), other.description_short.clone()) {
            (Some(_), Some(b)) => Some(b.clone()),
            (None, Some(b)) => Some(b.clone()),
            _ => self.description_short.clone()
        };
        self.example = match (self.example.clone(), other.example.clone()) {
            (Some(_), Some(b)) => Some(b.clone()),
            (None, Some(b)) => Some(b.clone()),
            _ => self.example.clone()
        };
        self.return_description = match (self.return_description.clone(), other.return_description.clone()) {
            (Some(_), Some(b)) => Some(b.clone()),
            (None, Some(b)) => Some(b.clone()),
            _ => self.return_description.clone()
        };        
        self.return_type = match (self.return_type.clone(), other.return_type.clone()) {
            (Some(_), Some(b)) => Some(b.clone()),
            (None, Some(b)) => Some(b.clone()),
            _ => self.return_type.clone()
        };
        self.source_file = match (self.source_file.clone(), other.source_file.clone()) {
            (Some(_), Some(b)) => Some(b.clone()),
            (None, Some(b)) => Some(b.clone()),
            _ => self.source_file.clone()
        };
        self.description_extended = match (self.description_extended.len(), other.description_extended.len()) {
            (_, i) if i > 0 => other.description_extended.clone(),
            _ => self.description_extended.clone()
        };
        self.parameters = match (self.parameters.len(), other.parameters.len()) {
            (_, i) if i > 0 => other.parameters.clone(),
            _ => self.parameters.clone()
        }
    }
}

fn merge_scardoc_enums(main: &ScarDoc, second: &ScarDoc) -> Vec<ScarEnum> {
    let mut enums = HashMap::new();
    for enum_def in main.enums.as_slice() {
        enums.insert(enum_def.name.clone(), enum_def.clone());
    }
    for enum_def in second.enums.as_slice() {
        enums.entry(enum_def.name.clone())
        .or_insert_with(|| { println!("Introducing enum {}", enum_def.name.clone()); enum_def.clone() })
        .merge_with(&enum_def);
    }
    enums.into_values().collect()
}

impl ScarEnum {
    pub fn has_value(&self, other:&ScarEnumValue) -> bool {
        self.values.as_slice()
        .into_iter()
        .any(|x| x.name == other.name && x.value == other.value)
    }
    pub fn eq(&mut self, other: &Self) -> bool {
        self.name == other.name && self.values.len() == other.values.len() && self.values.as_slice().into_iter().all(|x| other.has_value(&x))
    }
    pub fn merge_with(&mut self, other: &Self) {
        if self.eq(other) {
            return;
        }
        println!("Merging enum {}", self.name);
        self.values = match (self.values.len(), other.values.len()) {
            (_, i) if i > 0 => other.values.clone(),
            _ => self.values.clone()
        }
    }
}

fn merge_scardoc_globals(main: &ScarDoc, second: &ScarDoc) -> Vec<ScarGlobal> {
    let mut globals = HashMap::new();
    for global in main.globals.as_slice() {
        globals.insert(global.name.clone(), global.clone());
    }
    for global in second.globals.as_slice() {
        globals.entry(global.name.clone())
        .or_insert_with(|| { println!("Introducing global {}", global.name.clone()); global.clone() })
        .merge_with(&global);
    }
    globals.into_values().collect()
}

impl ScarGlobal {
    pub fn eq(&mut self, other: &Self) -> bool {
        self.name == other.name && self.description == other.description && self.global_type == other.global_type && self.value == other.value
    }
    pub fn merge_with(&mut self, other: &Self) {
        if self.eq(other) {
            return;
        }
        println!("Merging global {}", self.name);
        self.description = match (self.description.clone(), other.description.clone()) {
            (Some(_), Some(b)) => Some(b.clone()),
            (None, Some(b)) => Some(b.clone()),
            _ => self.description.clone()
        };
        self.global_type = match (self.global_type.clone(), other.global_type.clone()) {
            (Some(_), Some(b)) => Some(b.clone()),
            (None, Some(b)) => Some(b.clone()),
            _ => self.global_type.clone()
        };
        self.value = match (self.value.clone(), other.value.clone()) {
            (Some(_), Some(b)) => Some(b.clone()),
            (None, Some(b)) => Some(b.clone()),
            _ => self.value.clone()
        };
    }
}
