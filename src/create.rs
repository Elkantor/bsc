use std::process::Command;
use std::error::Error;
use std::io::prelude::*;
use std::fs;
use std::fs::File;

pub fn create_project(path: &str, project_name: &str){
    println!("Project name: {}", &project_name);
    initialize_git();
    create_main_cmakelists_file(path);
    create_folder(&format!("{}{}", &path, "src".to_string()));
    create_folder(&format!("{}{}", &path, "test".to_string()));
    create_folder(&format!("{}{}", &path, "include".to_string()));
    create_folder("bsc_modules");
    create_main_file(&format!("{}{}", &path, "src/".to_string()), &"main.c".to_string());
    create_main_file(&format!("{}{}", &path, "test/".to_string()), &"test.c".to_string());
    create_secondary_cmakelists_file(&format!("{}{}", &path, "src/".to_string()), &project_name);
    create_secondary_cmakelists_file(&format!("{}{}", &path, "test/".to_string()), &project_name);
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

pub fn create_main_file(path: &str, file_name: &str){
    let mut file = match File::create(format!("{}{}", &path, &file_name)){
        Err(why) => panic!("Error: couldn't create the {} source file. {}", &file_name, why.description()),
        Ok(file) => file,
    };

    let main_file_content = 
        "#include <stdio.h> \
        \n\nint main(int argc, char* argv[]){ \
        \n\tprintf(\"Hello World !\"); \
        \n\treturn 0; \
        \n}";

    match file.write_all(main_file_content.as_bytes()){
        Err(why) => println!("Error: couldn't write to {}: {}", format!("{}{}", &path, &file_name), why.description()),
        Ok(_) => println!("{} source file is correclty created.", &file_name),
    }
}

pub fn create_secondary_cmakelists_file(path: &str, project_name: &str){
    let final_path = "CMakeLists.txt".to_string();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(format!("{}{}", &path, final_path)) {
        Err(why) => panic!("Error: couldn't create the secondary CMakeLists.txt file inside {}. {}", &path, why.description()),
        Ok(file) => file,
    };

    let final_project_name = project_name.to_string();
    let mut secondary_cmakelists_content = format!(
        "project({}) \
        \n\nset(EXECUTABLE_OUTPUT_PATH bin/${{CMAKE_BUILD_TYPE}}) \
        \n\n## Add source files ## \
        \n\tfile (GLOB_RECURSE source_files ./*) \
        \n## End of adding source files ## \
        \n\n## Add executables ## \
        \n\tadd_executable ({} ${{source_files}}) \
        \n## End of adding executables ##", final_project_name, final_project_name);

    if path.to_string().contains("test"){
        secondary_cmakelists_content = format!(
            "project(test_{}) \
            \n\nset(EXECUTABLE_OUTPUT_PATH bin/${{CMAKE_BUILD_TYPE}}) \
            \n\n## Add source files ## \
            \n\tfile (GLOB_RECURSE testing_files ./*) \
            \n\tfile (GLOB_RECURSE testing_source_files ../src/*) \
            \n## End of adding source files ## \
            \n\n## Remove main.c files of modules ## \
            \n\tFOREACH(item ${{testing_source_files}}) \
            \n\t\tIF(${{item}} MATCHES \"main.c\") \
            \n\t\t\tLIST(REMOVE_ITEM testing_source_files ${{item}}) \
            \n\t\tENDIF(${{item}} MATCHES \"main.c\") \
            \n\tENDFOREACH(item) \
            \n## End of removing main.c files of modules ## \
            \n\n## Add executables ## \
            \n\tadd_executable (test_{} ${{testing_files}} ${{testing_source_files}}) \
            \n## End of adding executables ##", final_project_name, final_project_name);
    }
    
    match file.write_all(secondary_cmakelists_content.as_bytes()) {
        Err(why) => panic!("Error: couldn't write to {}: {}", format!("{}{}", &path, final_path), why.description()),
        Ok(_) => println!("Secondary CMakeLists.txt file is correclty created inside {}.", &path),
    }
}