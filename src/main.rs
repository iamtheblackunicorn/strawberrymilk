/*
STRAWBERRY MILK by Alexander Abraham,
a.k.a. "The Black Unicorn", a.k.a. "Angeldust Duke".
Licensed under the MIT license.
*/

use std::fs;
use std::env;
use std::path::Path;
use markdown::to_html;
use serde_json::from_str;
use std::collections::HashMap;

/// Strawberry Milk's operational constants.
fn constants() -> HashMap<String, String> {
    let mut constants:HashMap<String, String> = HashMap::new();
    constants.insert(String::from("config_file"), String::from("config.json"));
    return constants;
}

// Returns a vector of strings from a character split for a string.
/// Both the string and split character have to be strings.
fn clean_split(subject: String, split_char: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for item in subject.split(&split_char) {
        let new_item: String = item.to_string();
        result.push(new_item);
    }
    return result;
}

// Checks whether a file exists and
/// returns a boolean to that effect.
fn file_is(filename: String) -> bool {
    let mut result: Vec<bool> = Vec::new();
    let contents = fs::read_to_string(filename);
    match contents {
        Ok(_n) => result.push(true),
        Err(_x) => result.push(false)
    }
    return result[0];
}

/// Tries to create a file and returns
/// a boolean depending on whether the
/// operation succeeded.
fn create_file(filename: String) -> bool {
    let mut result: Vec<bool> = Vec::new();
    let new_file = fs::File::create(filename);
    match new_file {
        Ok(_n) => result.push(true),
        Err(_x) => result.push(false)
    }
    return result[0];
}

/// Tries to write to a file and returns
/// a boolean depending on whether the
/// operation succeeded.
fn write_to_file(filename: String, contents: String) -> bool {
    let mut result: Vec<bool> = Vec::new();
    let fname_copy: String = filename.clone();
    if file_is(filename) == true {
        let write_op = fs::write(fname_copy, contents);
        match write_op {
            Ok(_n) => result.push(true),
            Err(_x) => result.push(false)
        }
    }
    return result[0];
}

/// Tries to read a file and return
/// its contents.
fn read_file(filename: String) -> String {
    let mut result: String = String::from("");
    let fname_copy: String = filename.clone();
    if file_is(filename) == true {
        result = fs::read_to_string(fname_copy).unwrap();
    }
    else {}
    return result;
}

/// Reads a JSON string and returns a [HashMap] from it.
fn get_json(subject: String) -> HashMap<String, String> {
    return from_str(&subject).unwrap();
}

/// Getting site settings.
fn get_config(config_path: String) -> HashMap<String, String> {
    let result = get_json(read_file(config_path));
    return result;
}

/// Gets Markdown from a file and returns the HTML
/// output as a string.
fn get_html_from_markdown(md_path:String) -> String {
    let mut result: String = String::from("");
    let md_path_clone = md_path.clone();
    if file_is(md_path) == true {
        result = markdown::to_html(&read_file(md_path_clone));
    }
    else {}
    return result;
}

/// Lists files with their full paths and returns
/// [Vec<String>]
fn raw_list_files(dir: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    if Path::new(&dir).exists() {
        let paths = fs::read_dir(&dir).unwrap();
        for path in paths {
            let raw_path = path.unwrap().path().display().to_string();
            result.push(raw_path);
        }
    }
    else {}
    return result;
}


/// Compiles a Strawberry Milk project!
fn toolchain(project_path: String){
    let project_path_clone = project_path.clone();
    let config_path: String = format!("{}/{}", project_path, constants()["config_file"]);
    let settings: HashMap<String, String> = get_config(config_path);
    if settings.contains_key(&String::from("name")) && settings.contains_key(&String::from("content")) && settings.contains_key(&String::from("styles")) && settings.contains_key(&String::from("output")) {
        let content_path = format!("{}/{}", project_path_clone, settings["content"].clone());
        let content_files: Vec<String> = raw_list_files(content_path);
        if content_files.len() == 0 {
            println!("Invalid content path set!");
        }
        else {
            let mut content_list: Vec<String> = Vec::new();
            for file in content_files {
                content_list.push(markdown::to_html(&read_file(file)));
            }
            let content: String = content_list.join("");
            let heading_one: String = format!("<h1>{}</h1>",settings["name"].clone());
            let final_content: String = format!("<!DOCTYPE html>\n<html>\n<head>\n<title>{}</title>\n<link rel=\"stylesheet\" type=\"text/css\" href=\"{}\">\n</head>\n<body>\n{}{}\n</body>\n</html>", settings["name"].clone(),settings["styles"].clone(), heading_one, content);
            create_file(settings["output"].clone());
            write_to_file(settings["output"].clone(), final_content);
        }

    }
    else {
        println!("There was an error in your project configuration.")
    }
}

/// A small error message
/// in case someone misuses JAML.
fn error_message() {
    println!("Wrong usage!");
}

/// Strawberry Milk's command-line interface.
fn cli(){
    let args: Vec<String> = env::args().collect();
    let arg_len = args.len();
    if arg_len == 2 {
        if Path::new(args[1].clone()).exists() {
            toolchain(args[1].clone());
        }
        else {
            println!("The supplied directory doesn't exist!");
        }
    }
    else {
        error_message();
    }
}

/// Main entry point for the Rust compiler.
fn main(){
    cli();
}
