extern crate git2;
#[path = "common.rs"] mod common;
use std::error::Error;


/*************/
/* FUNCTIONS */
/*************/
pub fn create_project(path: &str, project_name: &str){
    println!("Project name: {}", &project_name);
    initialize_git(&path);
    create_main_cmakelists_file(path);
    common::create_folder(&format!("{}{}", &path, "src"));
    common::create_folder(&format!("{}{}", &path, "test"));
    common::create_folder(&format!("{}{}", &path, "include"));
    common::create_folder("bsc_modules");
    create_main_file(&format!("{}{}", &path, "src/")), &"main.c");
    create_main_file(&format!("{}{}", &path, "test/"), &"test.c");
    create_secondary_cmakelists_file(&format!("{}{}", &path, "src/"), &project_name);
    create_secondary_cmakelists_file(&format!("{}{}", &path, "test/"), &project_name);
    println!("The project is correclty created.");
}

pub fn initialize_git(path_repository: &str){
    match git2::Repository::init(&path_repository){
        Err(why) => panic!("Error: failed to create the git repository. {}", why.description()),
        Ok(_) => (),
    };

    println!("The git repository is correctly initialized.");
}

pub fn create_main_cmakelists_file(path: &str){
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

    common::create_file(&path, "CMakeLists.txt", &cmakelists_content);
}

pub fn create_main_file(path: &str, file_name: &str){
    let main_file_content = 
        "#include <stdio.h> \
        \n\nint main(int argc, char* argv[]){ \
        \n\tprintf(\"Hello World !\"); \
        \n\treturn 0; \
        \n}";

    common::create_file(&path, &file_name, &main_file_content);
}

pub fn create_secondary_cmakelists_file(path: &str, project_name: &str){
    let final_project_name = project_name.to_string();
    let mut secondary_cmakelists_content = format!(
        "project({}) \
        \n\nset(EXECUTABLE_OUTPUT_PATH bin/${{CMAKE_BUILD_TYPE}}) \
        \n\n## Add source files ## \
        \n\tfile (GLOB_RECURSE source_files ./*) \
        \n## End of adding source files ## \
        \n\n## Remove main.c files of modules ## \
        \n## End of removing main.c files of modules ## \
        \n\n## Add executables ## \
        \n\tadd_executable ({} ${{source_files}}) \
        \n## End of adding executables ##", 
        final_project_name, final_project_name);

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

    common::create_file(&path, "CMakeLists.txt", &secondary_cmakelists_content);
}