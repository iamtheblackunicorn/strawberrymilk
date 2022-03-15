/*
STRAWBERRY MILK by Alexander Abraham,
a.k.a. "The Black Unicorn", a.k.a. "Angeldust Duke".
Licensed under the MIT license.
*/

use std::fs;
use std::env;
use liquid::*;
use kstring::*;
use std::path::Path;
use markdown::to_html;
use serde_json::from_str;
use std::collections::HashMap;
use fs_extra::file::move_file;
use fs_extra::file::CopyOptions;
use serde_json::to_string_pretty;

/// Strawberry Milk's operational constants.
fn constants() -> HashMap<String, String> {
    let mut constants: HashMap<String, String> = HashMap::new();
    constants.insert(String::from("config_file"), String::from("config.json"));
    constants.insert(String::from("build_dir"), String::from("build"));
    return constants;
}

/// What content to write to which files in a new Strawberry Milk project.
fn project_file_contents_constants(name: String) -> HashMap<String, String> {
    let project_name_clone = name.clone();
    let mut project_constants: HashMap<String, String> = HashMap::new();
    let mut project_dummy_settings: HashMap<String, String> = HashMap::new();
    project_dummy_settings.insert(String::from("name"),project_name_clone);
    project_dummy_settings.insert(String::from("content"),String::from("content"));
    project_dummy_settings.insert(String::from("styles"),String::from("https://blckunicorn.art/assets/generic/strawberrymilk.css"));
    project_dummy_settings.insert(String::from("output"),String::from("index.html"));
    project_constants.insert(String::from("config_file"), to_string_pretty(&project_dummy_settings).unwrap());
    project_constants.insert(String::from("markdown_sample"), String::from("# YOUR PROJECT\nYour awesome content goes here."));
    project_constants.insert(String::from("gitignore"), String::from("/build\n.DS_Store"));
    return project_constants;
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

/// Tries to create a new file and returns
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

/// Tries to create a new directory and returns
/// a boolean depending on whether the
/// operation succeeded.
fn create_dir(path: String) -> bool {
    let mut result: Vec<bool> = Vec::new();
    let new_dir = fs::create_dir(path);
    match new_dir {
        Ok(_n) => result.push(true),
        Err(_x) => result.push(false)
    }
    return result[0];
}

/// Tries to move a file from [src] to [target]
/// and returns a boolean depending on whether the
/// operation succeeded.
fn file_move(src: String, target: String) -> bool {
    let mut result: Vec<bool> = Vec::new();
    let options = CopyOptions::new();
    let move_op = move_file(src, target, &options);
    match move_op {
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

/// Getting site settings as a [HashMap<String, String>].
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
        result = to_html(&read_file(md_path_clone));
    }
    else {}
    return result;
}

/// Lists files with their full paths and returns
/// [Vec<String>].
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

/// Scaffolds a new Strawberry Milk
/// project.
fn scaffold(name: String) {
    let project_name_clone_one = name.clone();
    let project_name_clone_two = project_name_clone_one.clone();
    let project_name_clone_three = project_name_clone_two.clone();
    let project_name_clone_four = project_name_clone_three.clone();
    let project_name_clone_five = project_name_clone_four.clone();
    let project_name_clone_six = project_name_clone_five.clone();
    let project_name_clone_seven = project_name_clone_six.clone();
    let project_name_clone_eight = project_name_clone_seven.clone();
    let std_config: String = format!("{}/{}",project_name_clone_three,String::from("config.json"));
    let std_md: String = format!("{}/{}/{}",project_name_clone_five,String::from("content"),String::from("01.markdown"));
    let std_git_ignore: String = format!("{}/{}",project_name_clone_seven,String::from(".gitignore"));
    let std_md_clone: String = std_md.clone();
    let std_config_clone: String = std_config.clone();
    let std_git_ignore_clone: String = std_git_ignore.clone();
    let full_content_dir_path = format!("{}/{}", project_name_clone_one, String::from("content"));
    create_dir(project_name_clone_two);
    create_dir(full_content_dir_path);
    create_file(std_config);
    write_to_file(std_config_clone,project_file_contents_constants(project_name_clone_four)["config_file"].clone());
    create_file(std_md);
    write_to_file(std_md_clone,project_file_contents_constants(project_name_clone_six)["markdown_sample"].clone());
    create_file(std_git_ignore);
    write_to_file(std_git_ignore_clone,project_file_contents_constants(project_name_clone_eight)["gitignore"].clone());
}
#[derive(liquid::{ObjectView,ValueView})]
struct Context {
    project: HashMap<String, String>,
    pages: Vec<HashMap<String, String>>
}

/// Compiles a Strawberry Milk project!
fn toolchain(project_path: String){
    let project_path_clone = project_path.clone();
    let project_path_clone_one = project_path_clone.clone();
    let project_path_clone_two = project_path_clone_one.clone();
    let config_path: String = format!("{}/{}", project_path, constants()["config_file"]);
    let config_path_clone = config_path.clone();
    if file_is(config_path) {
        let settings: HashMap<String, String> = get_config(config_path_clone);
        let settings_clone: HashMap<String, String> = settings.clone();
        if settings.contains_key(&String::from("name")) && settings.contains_key(&String::from("content")) && settings.contains_key(&String::from("styles")) && settings.contains_key(&String::from("output")) {
            let content_path = format!("{}/{}", project_path_clone, settings["content"].clone());
            let content_files: Vec<String> = raw_list_files(content_path);
            if content_files.len() == 0 {
                println!("Invalid content path set!");
            }
            else {
                if settings.contains_key(&String::from("use_template")) && settings.contains_key(&String::from("template_path")) {
                    if settings["use_template"].clone() == "true" && file_is(format!("{}/{}",project_path_clone_one,settings["template_path"].clone())) == true {
                        let mut content_list: Vec<HashMap<String,String>> = Vec::new();
                        for file in content_files {
                            let mut cont_hash: HashMap<String,String> = HashMap::new();
                            cont_hash.insert(String::from("name"),to_html(&read_file(file)));
                            content_list.push(cont_hash);
                        }
                        let template_path: String = format!("{}/{}",project_path_clone_two,settings["template_path"].clone());
                        let liquid_string = ParserBuilder::with_stdlib().build().unwrap().parse(&read_file(template_path)).unwrap();
                        let project = Context{
                            project:settings_clone,
                            pages:content_list
                        };
                        let mut globals = object!(project);
                        let output = liquid_string.render(&globals);
                        //create_file(settings["output"].clone());
                        //write_to_file(settings["output"].clone(), final_content);
                        //create_dir(constants()["build_dir"].clone());
                        //file_move(settings["output"].clone(),format!("{}/{}",constants()["build_dir"].clone(),settings["output"].clone()));
                    }
                    else {
                        println!("Template not found or the \'use_template\' was set to \'false\'!");
                    }
                }
                else {
                    let mut content_list: Vec<String> = Vec::new();
                    for file in content_files {
                        content_list.push(to_html(&read_file(file)));
                    }
                    let content: String = content_list.join("");
                    let heading_one: String = format!("<h1>{}</h1>",settings["name"].clone());
                    let final_content: String = format!("<!DOCTYPE html>\n<html>\n<head>\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n<title>{}</title>\n<link rel=\"stylesheet\" type=\"text/css\" href=\"{}\">\n</head>\n<body>\n{}{}\n</body>\n</html>", settings["name"].clone(),settings["styles"].clone(), heading_one, content);
                    create_file(settings["output"].clone());
                    write_to_file(settings["output"].clone(), final_content);
                    create_dir(constants()["build_dir"].clone());
                    file_move(settings["output"].clone(),format!("{}/{}",constants()["build_dir"].clone(),settings["output"].clone()));
                }
            }
        }
        else {
            println!("There was an error in your project configuration.")
        }
    }
    else {
        println!("'{}' could not be found.", constants()["config_file"]);
    }
}

/// A small error message
/// in case someone misuses Strawberry Milk.
fn error_message() {
    println!("Wrong usage!");
}

/// Strawberry Milk's command-line interface.
fn cli(){
    let args: Vec<String> = env::args().collect();
    let arg_len = args.len();
    if arg_len == 2 {
        if Path::new(&args[1].clone()).exists() {
            toolchain(args[1].clone());
        }
        else {
            println!("The supplied directory doesn't exist!");
        }
    }
    else if arg_len == 3 {
        if args[1].clone() == "new" {
            scaffold(args[2].clone());
        }
        else {
            error_message();
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
