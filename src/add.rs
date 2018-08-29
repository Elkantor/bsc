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
    // common::create_folder(&format!("{}/bsc_modules", &path));
    // common::destroy_folder(&format!("{}/bsc_modules/tmp", &path));
    // copy_module_to_tmp(&path, &module_url, module_type);

    let mut module_name = String::new();
    // common::get_module_name(&format!("{}/bsc_modules/tmp/", &path), &mut module_name);
    common::get_module_name(&format!("{}/bsc_modules/bsc_test_2/", &path), &mut module_name);
    // common::rename_folder(&format!("{}{}", &path, "bsc_modules/"), "tmp", &module_name);
    add_module_header_to_main_cmakelists_file(&path, &format!("{}{}", "bsc_modules/", &module_name));
    println!("{} is correclty added to the project.", console::style(&module_name).cyan());
    

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

pub fn add_module_header_to_main_cmakelists_file(path: &str, module_path: &str){
    let mut file_content_lines = Vec::new();
    common::get_file_content(&format!("{}{}", &path, "CMakeLists.txt"), &mut file_content_lines);

    let mut line_founded: bool = false;
    let mut line_index = 0;
    let mut file_new_content_lines = String::new();
    for line in file_content_lines.lines() {
        let current_line = line.unwrap();
        if current_line.contains("## End of include libraries ##") {
            file_new_content_lines += &format!("\n\tinclude_directories (\"${{PROJECT_BINARY_DIR}}/../{}/include\")", &module_path);
        }
        file_new_content_lines += &format!("\n{}", &current_line);
    }

    common::set_content_file(&format!("{}{}", &path, "CMakeLists.txt"), &file_new_content_lines.into_bytes());
}

