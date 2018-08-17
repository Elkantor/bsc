use std::process::Command;
use std::error::Error;
use std::io::prelude::*;
use std::fs;
use std::fs::File;

pub fn create_project(path: &str){
    initialize_git();
    create_main_cmakelists_file(path);
    create_folder(&format!("{}{}", &path, "src".to_string()));
    create_folder(&format!("{}{}", &path, "test".to_string()));
    create_folder(&format!("{}{}", &path, "include".to_string()));
    create_folder("bsc_modules");
    create_main_file(&format!("{}{}", &path, "src/".to_string()));
    println!("The project is correclty created.");
}

pub fn initialize_git(){
    if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", "git init > nul"])
                .output()
                .expect("Failed to initialize the git repository.")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg("git init > nul")
                .output()
                .expect("Failed to initialize the git repository.")
    };

    println!("The git repository is correctly initialized.");
}

pub fn create_main_cmakelists_file(path: &str){
    let final_path = "CMakeLists.txt".to_string();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(format!("{}{}", &path, final_path)) {
        Err(why) => panic!("Error: couldn't create the CMakeLists.txt main file. {}", why.description()),
        Ok(file) => file,
    };

    let cmakelists_content = 
        "cmake_minimum_required (VERSION 3.9) \
        \n\n## Include libraries ## \
        \n\tinclude_directories (\"${PROJECT_BINARY_DIR}/../include\") \
        \n## End of include libraries ## \
        \n\nset(EXECUTABLE_OUTPUT_PATH bin/${CMAKE_BUILD_TYPE}) \
        \n\nif (MSVC) \
        \n\tset(EXECUTABLE_OUTPUT_PATH bin/) \
        \nendif (MSVC) \
        \n\n## Add executables ## \
        \n\tadd_subdirectory (src) \
        \n\tadd_subdirectory (test) \
        \n## End of adding executables ##";

    match file.write_all(cmakelists_content.as_bytes()) {
        Err(why) => panic!("Error: couldn't write to {}: {}", format!("{}{}", &path, final_path), why.description()),
        Ok(_) => println!("Main CMakeLists.txt file is correclty created."),
    }
}

pub fn create_folder(folder_path: &str){
    match fs::create_dir(folder_path){
        Err(e) => println!("Error occured when creating the \"{}\" folder. {}", &folder_path, e.description()),
        Ok(_) => println!("\"{}\" folder is correclty created.", &folder_path),
    }
}

pub fn create_main_file(path: &str){
    let final_path = "main.c".to_string();

    let mut file = match File::create(format!("{}{}", &path, final_path)){
        Err(why) => panic!("Error: couldn't create the main.c source file. {}", why.description()),
        Ok(file) => file,
    };

    let main_file_content = 
        "#include <stdio.h> \
        \n\nint main(int argc, char* argv[]){ \
        \n\tprintf(\"Running the project...\"); \
        \n\treturn 0; \
        \n}";

    match file.write_all(main_file_content.as_bytes()){
        Err(why) => println!("Error: couldn't write to {}: {}", format!("{}{}", &path, final_path), why.description()),
        Ok(_) => println!("main.c source file is correclty created."),
    }
}