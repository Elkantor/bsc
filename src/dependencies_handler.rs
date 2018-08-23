use std::error::Error;
use std::io::prelude::*;
use std::fs::File;

pub fn update_dependencies_file(project_url: &str, project_path: &str, project_name: &str){
    let dependencies_file_name = "dependencies.bsc".to_string();

    let mut file = match File::create(format!("{}{}", &project_path, dependencies_file_name)){
        Err(why) => panic!("Error: couldn't create {}: {}", dependencies_file_name, why.description()),
        Ok(file) => file,
    };

    let content_dependencies_file = format!(
        "BSC_PROJECT: \
        \n\t[{}]:^0.1.0\t|\t{} \
        \n\nBSC_DEPENDENCIES: \
        \n", &project_name, &project_url
    );

    // To complete to update the dependencies (for each module inside bsc_modules folder).

    match file.write_all(content_dependencies_file.as_bytes()){
        Err(why) => panic!("Error: couldn't write to {}: {}", dependencies_file_name, why.description()),
        Ok(_) => println!("Dependencies file correctly updated."),
    }
}