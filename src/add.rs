extern crate git2;
extern crate console;
#[path = "common.rs"] mod common;
use std::fs;
use std::error::Error;
use std::io::BufRead;


/*********/
/* ENUMS */
/*********/

pub enum ModuleType {
    Local,
    Git,
    Web
}

/*************/
/* FUNCTIONS */
/*************/
pub fn add_dependency(path: &str, module_url: &str, module_type: ModuleType){
    let mut module_name = String::new();

    common::create_folder(&format!("{}/bsc_modules", &path));
    common::destroy_folder(&format!("{}/bsc_modules/tmp", &path));
    copy_module_to_tmp(&path, &module_url, module_type);
    common::get_module_name(&format!("{}/bsc_modules/tmp/", &path), &mut module_name);

    // Check if the module to add is already in the project dependencies
    if check_already_added(&path, &format!("{}{}", "bsc_modules/", &module_name)){
        println!("{} is already added to the project", console::style(&module_name).cyan());
        common::destroy_folder(&format!("{}/bsc_modules/tmp", &path));
        return;
    }

    common::rename_folder(&format!("{}{}", &path, "bsc_modules/"), "tmp", &module_name);
    move_module_dependencies_to_parent_folder(&path, &format!("{}bsc_modules/{}/", &path, &module_name));


    let complete_module_path = &format!("{}{}{}", &path, "bsc_modules/", &module_name);
    add_module_headers_to_main_cmakelists_file(&path, &format!("{}{}", "bsc_modules/", &module_name));
    add_module_sources_files_to_secondary_cmakelists_file(&format!("{}{}", &path, "src/"), &complete_module_path, &module_name);
    add_module_sources_files_to_secondary_cmakelists_file(&format!("{}{}", &path, "test/"), &complete_module_path, &module_name);
    println!("{} is correclty added to the project.", console::style(&module_name).cyan());
}

pub fn check_already_added(path: &str, module_path: &str) -> bool{
    let mut file_content_lines = Vec::new();
    common::get_file_content(&format!("{}{}", &path, "CMakeLists.txt"), &mut file_content_lines);

    let mut previous_line = String::new();
    for line in file_content_lines.lines() {
        let current_line = line.unwrap();
        if current_line.contains("## End of include libraries ##") {
            if previous_line.contains(&format!("include_directories (\"${{PROJECT_BINARY_DIR}}/../{}/include\")", &module_path)){
                println!("true");
                return true;
            }
        }
        previous_line = String::from(current_line);
    }
    return false;
}

pub fn copy_module_to_tmp(path: &str, module_url: &str, module_type: ModuleType){
    match module_type {
        ModuleType::Git => {
            match git2::Repository::clone(&module_url.to_string(), format!("{}/bsc_modules/tmp/", &path)){
                Err(why) => panic!("Error: failed to clone {}.", why.description()),
                Ok(_) => (),
            };
        },
        ModuleType::Local => {
            common::copy_folder(&module_url, &format!("{}/bsc_modules/tmp", &path));
        },
        _ => unreachable!(),
    }
}

pub fn add_module_headers_to_main_cmakelists_file(path: &str, module_path: &str){
    let mut file_content_lines = Vec::new();
    common::get_file_content(&format!("{}{}", &path, "CMakeLists.txt"), &mut file_content_lines);

    let mut file_new_content_lines = String::new();
    let mut previous_line = String::new();
    let mut current_index_line = 0;
    for line in file_content_lines.lines() {
        let current_line = line.unwrap();
        if current_line.contains("## End of include libraries ##") {
            if previous_line.contains(&format!("\n\tinclude_directories (\"${{PROJECT_BINARY_DIR}}/../{}/include\")", &module_path)){
                return;
            }
            file_new_content_lines += &format!("\n\tinclude_directories (\"${{PROJECT_BINARY_DIR}}/../{}/include\")", &module_path);
        }
        if current_index_line == 0{
            file_new_content_lines += &format!("{}", &current_line);
        }else{
            file_new_content_lines += &format!("\n{}", &current_line);
        }
        previous_line = String::from(current_line);
        current_index_line += 1;
    }

    common::set_content_file(&format!("{}{}", &path, "CMakeLists.txt"), &file_new_content_lines.into_bytes());
}

pub fn add_module_sources_files_to_secondary_cmakelists_file(path: &str, module_path: &str, module_name: &str){
    let mut file_content_lines = Vec::new();
    common::get_file_content(&format!("{}{}", &path, "CMakeLists.txt"), &mut file_content_lines);

    let mut file_new_content_lines = String::new();
    let mut current_index_line = 0;
    let mut previous_line = String::new();
    let mut module_already_added: bool = false;
    let mut executable_line_passed: bool = false;

    for line in file_content_lines.lines(){
        let mut current_line = line.unwrap();
        if current_line.contains("## End of adding source files ##"){
            if !previous_line.contains(&format!("file (GLOB_RECURSE {}_source_files ../bscxx_modules/{}/src/*)", &module_name, &module_name)){
                file_new_content_lines += &format!(
                    "\n\tfile (GLOB_RECURSE {}_source_files ../bscxx_modules/{}/src/*)", 
                    &module_name,
                    &module_name
                );
            }else{
                module_already_added = true;
                return;
            }
        }
        if current_line.contains("## End of removing main.c files of modules ##"){
            if !module_already_added{
                file_new_content_lines += &format!(
                    "\n\tFOREACH(item ${{{}_source_files}}) \
                    \n\t\tIF(${{item}} MATCHES \"main.c\") \
                    \n\t\t\tLIST(REMOVE_ITEM {}_source_files ${{item}}) \
                    \n\t\tENDIF(${{item}} MATCHES \"main.c\") \
                    \n\tENDFOREACH(item)", &module_name, &module_name
                );
            }
        }
        if executable_line_passed{
            let mut new_executable_line = String::new();
            for c in current_line.chars(){
                new_executable_line.push(c);
                if c == '}' {
                    new_executable_line += &format!(" ${{{}_source_files}})", &module_name);
                    break;
                }
            }
            current_line = new_executable_line;
        }
        if current_line.contains("## Add executables ##"){
            if !module_already_added{
                executable_line_passed = true;
            }
        }
        if current_index_line == 0{
            file_new_content_lines += &format!("{}", &current_line);
        }else{
            file_new_content_lines += &format!("\n{}", &current_line);
        }
        current_index_line += 1;
        previous_line = String::from(current_line);
    }

    common::set_content_file(&format!("{}{}", &path, "CMakeLists.txt"), &file_new_content_lines.into_bytes());
}

pub fn move_module_dependencies_to_parent_folder(path: &str, module_path: &str){
    let folder_dependencies = &format!("{}{}", &module_path, "bsc_modules");
    let module_dependencies = match fs::read_dir(&folder_dependencies){
        Err(why) => panic!("Error: couldn't get the content of the {} folder. {}", &folder_dependencies, why.description()),
        Ok(module_dependencies) => module_dependencies,
    };

    for dependency_folder in module_dependencies{
        let content_dependency_folder = match dependency_folder {
            Err(why) => break,
            Ok(content_dependency_folder) => (content_dependency_folder),       
        };
        let path_dependency = content_dependency_folder.path();
        let path_dependency_text = path_dependency.to_str().unwrap();
        let folder_name = str::replace(&format!("{:?}", &content_dependency_folder.file_name()), "\"", "");
        let mut module_name = String::new();

        common::get_module_name(&format!("{}/", &path_dependency_text), &mut module_name);
        common::copy_folder(&format!("{}/", &path_dependency_text), &format!("{}{}", &path, "bsc_modules/"));
        common::rename_folder(&format!("{}{}", &path, "bsc_modules/"), &folder_name, &module_name);
        common::destroy_folder(&format!("{}/", &path_dependency_text));
        change_headers_file_from_main_cmakelists_file(&module_path, &module_name);
        change_sources_files_from_secondary_cmakelists_file();
        add_module_headers_to_main_cmakelists_file(&path, &format!("{}{}{}", &path, "bsc_modules/", &module_name));
        move_module_dependencies_to_parent_folder(&path, &format!("{}bsc_modules/{}/", &path, &folder_name));
    }
}

pub fn change_headers_file_from_main_cmakelists_file(module_path: &str, module_name: &str){
    let mut file_content_lines = Vec::new();
    let file_path = format!("{}{}", &module_path, "CMakeLists.txt");
    common::get_file_content(&file_path, &mut file_content_lines);
    println!("content of file : {}", &file_path);
    let mut file_new_content_lines = String::new();
    let mut current_index_line = 0;
    let mut previous_line = String::new();

    for line in file_content_lines.lines(){
        let mut current_line = line.unwrap();
        if current_line.contains(&format!("include_directories (\"${{PROJECT_BINARY_DIR}}/../bsc_modules/{}", &module_name)){
            let index_begin = current_line.find("bsc_modules/").unwrap();
            let new_line = format!("{}{}", &current_line[0..index_begin], &current_line[index_begin+12..]);
            file_new_content_lines += &format!("\n{}", &new_line);
            current_index_line += 1;
            continue;
        }
        if current_index_line == 0{
            file_new_content_lines += &String::from(current_line);
        }else{
            file_new_content_lines += &format!("\n{}", &current_line);
        }
        current_index_line += 1;
    } 
    common::set_content_file(&file_path, &file_new_content_lines.into_bytes());
}

pub fn change_sources_files_from_secondary_cmakelists_file(){

}