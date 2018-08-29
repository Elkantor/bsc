extern crate git2;
extern crate console;
#[path = "common.rs"] mod common;
use std::error::Error;
use std::io::BufRead;


/*********/
/* ENUMS */
/*********/

pub enum ModuleType {
    Local,
    Git,
    Web,
    Official
}

/*************/
/* FUNCTIONS */
/*************/
pub fn add_dependency(path: &str, module_url: &str, module_type: ModuleType){
    let mut module_name = String::new();
    module_name = "bsc_test_2".to_string();
    // common::create_folder(&format!("{}/bsc_modules", &path));
    // common::destroy_folder(&format!("{}/bsc_modules/tmp", &path));
    // copy_module_to_tmp(&path, &module_url, module_type);
    // common::get_module_name(&format!("{}/bsc_modules/tmp/", &path), &mut module_name);

    if check_already_added(&path, &format!("{}{}", "bsc_modules/", &module_name), &module_name){
        println!("{} is already added to the project", console::style(&module_name).cyan());
        common::destroy_folder(&format!("{}/bsc_modules/tmp", &path));
        return;
    }

    // common::rename_folder(&format!("{}{}", &path, "bsc_modules/"), "tmp", &module_name);
    add_module_header_to_main_cmakelists_file(&path, &format!("{}{}", "bsc_modules/", &module_name), &module_name);
    add_module_header_to_secondary_cmakelists_file(&format!("{}{}", &path, "src/"), &format!("{}{}{}", &path, "bsc_modules/", &module_name), &module_name);
    println!("{} is correclty added to the project.", console::style(&module_name).cyan());


}

pub fn check_already_added(path: &str, module_path: &str, module_name: &str) -> bool{
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
            let repo = match git2::Repository::clone(&module_url.to_string(), format!("{}/bsc_modules/tmp/", &path)){
                Ok(repo) => repo,
                Err(why) => panic!("Error: failed to clone {}.", why.description()),
            };
        },
        ModuleType::Local => {
            common::copy_folder(&module_url, &format!("{}/bsc_modules/tmp", &path));
        },
        _ => unreachable!(),
    }
}

pub fn add_module_header_to_main_cmakelists_file(path: &str, module_path: &str, module_name: &str){
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

pub fn add_module_header_to_secondary_cmakelists_file(path: &str, module_path: &str, module_name: &str){
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
