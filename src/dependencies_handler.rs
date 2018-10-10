#![allow(dead_code)]

#[path = "common.rs"] mod common;
use std::io::BufRead;


pub fn update_dependencies_file(project_url: &str, project_path: &str, project_name: &str){
    let content_dependencies_file = format!(
        "BSC_PROJECT: \
        \n\t[{}]:^0.1.0\t|\t{} \
        \n\nBSC_DEPENDENCIES: \
        \n", &project_name, &project_url
    );

    // To complete

    common::create_file(&project_path, "dependencies.bsc", &content_dependencies_file);
}

pub fn add_module_to_dependencies_file(project_path: &str, module_name: &str, module_version: &str, module_url: &str){
    let mut file_content_lines = Vec::new();
    let file_path = format!("{}dependencies.bsc", &project_path);
    common::get_file_content(&file_path, &mut file_content_lines);

    let mut file_new_content_lines = String::new();
    let mut current_index_line = 0;
    let module_line = &format!("\t[{}]:^{}\t|\t{}", &module_name, &module_version, &module_url);

    for line in file_content_lines.lines(){
        let mut current_line = line.unwrap();
        if current_line.contains(module_line){
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

    file_new_content_lines += &format!("\n{}", &module_line);
    common::set_content_file(&file_path, &file_new_content_lines.into_bytes());
}