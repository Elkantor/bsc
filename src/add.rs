extern crate git2;
extern crate console;
extern crate curl;
extern crate zip;
#[path = "common.rs"] mod common;
#[path = "dependencies_handler.rs"] mod dependencies_handler;
use std::fs;
use std::error::Error;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;


/*********/
/* ENUMS */
/*********/

pub enum ModuleType {
    Local,
    Git,
    Zip
}

/*************/
/* FUNCTIONS */
/*************/
pub fn add_dependency(path: &str, module_url: &str, module_type: ModuleType){
    let mut module_name = String::new();

    common::create_folder(&format!("{}/bsc_modules", &path));
    common::destroy_folder(&format!("{}/bsc_modules/tmp", &path));
    copy_module_to_tmp(&path, &module_url, module_type);

    // Check if it's a valid bsc module
    if !common::check_dependencies_file(&format!("{}/bsc_modules/tmp/", &path)){
        common::destroy_folder(&format!("{}/bsc_modules/tmp", &path));
        panic!("Error: you are trying to add a not valid bsc module to the project.");
    }

    common::get_module_name(&format!("{}/bsc_modules/tmp/", &path), &mut module_name);

    // Check if the module to add is already in the project dependencies
    if check_already_added(&path, &format!("{}{}", "bsc_modules/", &module_name)){
        println!("{} is already added to the project", console::style(&module_name).cyan());
        common::destroy_folder(&format!("{}/bsc_modules/tmp", &path));
        return;
    }

    common::rename_folder(&format!("{}{}", &path, "bsc_modules/"), "tmp", &module_name);
    move_module_dependencies_to_parent_folder(&path, &format!("{}bsc_modules/{}/", &path, &module_name));

    let module_version = "0.1.0";
    let module_path = &format!("{}bsc_modules/{}/", &path, &module_name);
    let headers_path = &format!("{}include/", &module_path);
    let sources_path = &format!("{}src/", &module_path);
    common::copy_folder(&headers_path, &format!("{}/tmp_include/{}", &path, &module_name));
    common::destroy_folder(&headers_path);
    common::rename_folder(&path, "tmp_include", &headers_path);
    add_module_headers_to_main_cmakelists_file(&path, &format!("{}{}", "bsc_modules/", &module_name));
    add_module_sources_files_to_secondary_cmakelists_file(&format!("{}{}", &path, "src/"), &module_name);
    add_module_sources_files_to_secondary_cmakelists_file(&format!("{}{}", &path, "test/"), &module_name);

    let mut headers: Vec<String> = Vec::new();
    get_list_path_include_files(&headers_path, &mut headers, &module_name);
    change_include_path_in_sources_files(&sources_path, &module_name, &headers);
    change_include_path_in_headers_files(&headers_path, &module_name, &headers);

    dependencies_handler::add_module_to_dependencies_file(&path, &module_name, &module_version, &module_url);
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
            match git2::Repository::clone(&module_url.to_string(), format!("{}bsc_modules/tmp/", &path)){
                Err(why) => panic!("Error: failed to clone {}.", why.description()),
                Ok(_) => (),
            };
        },
        ModuleType::Local => {
            common::copy_folder(&module_url, &format!("{}bsc_modules/tmp", &path));
        },
        ModuleType::Zip => {
            common::create_folder(&format!("{}bsc_modules/tmp", &path));
            let mut easy = curl::easy::Easy::new();
            let mut dst = Vec::new();
            // Get the zip file using the curl crate
            easy.url(&module_url).unwrap();
            {
                let mut transfer = easy.transfer();
                transfer.write_function(|data| {
                    dst.extend_from_slice(data);
                    Ok(data.len())
                }).unwrap();
                transfer.perform().unwrap();
            }
            // Copy the content of the dst vector inside the tmp/test.zip file
            common::set_content_file(&format!("{}bsc_modules/tmp/test.zip", &path), &dst);          
           
            // Extract the test.zip file using the zip crate
            let file = fs::File::open(format!("{}bsc_modules/tmp/test.zip", &path)).unwrap();
            let mut archive = zip::ZipArchive::new(file).unwrap();
            let mut folder_path = String::new();
            for i in 0..archive.len() {
                let mut file = archive.by_index(i).unwrap();
                if i == 0 {
                    folder_path = file.sanitized_name().into_os_string().into_string().unwrap();
                }
                let mut outpath = PathBuf::from(format!("{}{}", &path, "bsc_modules/tmp/"));
                outpath.push(file.sanitized_name());
                {
                    let comment = file.comment();
                    if !comment.is_empty() {
                        println!("File {} comment: {}", i, comment);
                    }
                }

                if (&*file.name()).ends_with('/') {
                    println!("File {} extracted to \"{}\"", i, outpath.as_path().display());
                    fs::create_dir_all(&outpath).unwrap();
                }else{
                             println!("File {} extracted to \"{}\" ({} bytes)", i, outpath.as_path().display(), file.size());
                    if let Some(p) = outpath.parent() {
                        if !p.exists() {
                            fs::create_dir_all(&p).unwrap();
                        }
                    }
                    let mut outfile = fs::File::create(&outpath).unwrap();
                    io::copy(&mut file, &mut outfile).unwrap();
                }
            }
            common::delete_file(&format!("{}bsc_modules/tmp/test.zip", &path));
            common::move_folder_content_to_parent_folder(&format!("{}bsc_modules/tmp/{}", &path, folder_path));
            common::destroy_folder(&format!("{}bsc_modules/tmp/{}", &path, folder_path));
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

pub fn add_module_sources_files_to_secondary_cmakelists_file(path: &str, module_name: &str){
    let mut file_content_lines = Vec::new();
    common::get_file_content(&format!("{}{}", &path, "CMakeLists.txt"), &mut file_content_lines);

    let mut file_new_content_lines = String::new();
    let mut current_index_line = 0;
    let mut previous_line = String::new();
    let mut executable_line_passed: bool = false;

    for line in file_content_lines.lines(){
        let mut current_line = line.unwrap();
        if current_line.contains("## End of adding source files ##"){
            if !previous_line.contains(&format!("file (GLOB_RECURSE {}_source_files ../bsc_modules/{}/src/*)", &module_name, &module_name)){
                file_new_content_lines += &format!(
                    "\n\tfile (GLOB_RECURSE {}_source_files ../bsc_modules/{}/src/*)", 
                    &module_name,
                    &module_name
                );
            }else{
                // Module already added, return
                return;
            }
        }
        if current_line.contains("## End of removing main.c files of modules ##"){
            file_new_content_lines += &format!(
                "\n\tFOREACH(item ${{{}_source_files}}) \
                \n\t\tIF(${{item}} MATCHES \"main.c\") \
                \n\t\t\tLIST(REMOVE_ITEM {}_source_files ${{item}}) \
                \n\t\tENDIF(${{item}} MATCHES \"main.c\") \
                \n\tENDFOREACH(item)", &module_name, &module_name
            );
        }
        if executable_line_passed{
            let index_begin = match current_line.find(")"){
                Some(value) => (value),
                None => (0),
            };
            if index_begin != 0 { 
                let new_executable_line = format!("{}{}{}", &current_line[0..index_begin], &format!(" ${{{}_source_files}})", &module_name), &current_line[index_begin+1..]);
                current_line = new_executable_line;
            }
        }
        if current_line.contains("## Add executables ##"){
            executable_line_passed = true;
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
            Err(_why) => break,
            Ok(content_dependency_folder) => (content_dependency_folder),       
        };
        let path_dependency = content_dependency_folder.path();
        let path_dependency_text = path_dependency.to_str().unwrap();
        let folder_name = str::replace(&format!("{:?}", &content_dependency_folder.file_name()), "\"", "");
        let mut module_name = String::new();
        let mut module_url = String::new();

        common::get_module_name(&format!("{}/", &path_dependency_text), &mut module_name);
        common::get_module_url(&format!("{}/", &path_dependency_text), &mut module_url);

        common::copy_folder(&format!("{}/", &path_dependency_text), &format!("{}{}", &path, "bsc_modules/"));
        common::rename_folder(&format!("{}{}", &path, "bsc_modules/"), &folder_name, &module_name);
        common::destroy_folder(&format!("{}/", &path_dependency_text));
        change_headers_file_from_main_cmakelists_file(&module_path, &module_name);
        change_sources_files_from_secondary_cmakelists_file(&module_path, &module_name);
        common::copy_folder(&format!("{}bsc_modules/{}/include/", &path, &module_name), &format!("{}bsc_modules/{}/include/{}", &path, &module_name, &module_name));
        add_module_headers_to_main_cmakelists_file(&path, &format!("{}{}", "bsc_modules/", &module_name));
        add_module_sources_files_to_secondary_cmakelists_file(&format!("{}{}", &path, "src/"), &module_name);
        dependencies_handler::add_module_to_dependencies_file(&path, &module_name, "0.1.0", &module_url);
        move_module_dependencies_to_parent_folder(&path, &format!("{}bsc_modules/{}/", &path, &folder_name));
    }
}

pub fn change_headers_file_from_main_cmakelists_file(module_path: &str, module_name: &str){
    let mut file_content_lines = Vec::new();
    let file_path = format!("{}{}", &module_path, "CMakeLists.txt");
    common::get_file_content(&file_path, &mut file_content_lines);
    let mut file_new_content_lines = String::new();
    let mut current_index_line = 0;

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

pub fn change_sources_files_from_secondary_cmakelists_file(module_path: &str, module_name: &str){
    let mut file_content_lines = Vec::new();
    let file_path = format!("{}src/CMakeLists.txt", &module_path);
    common::get_file_content(&file_path, &mut file_content_lines);
    let mut file_new_content_lines = String::new();
    let mut current_index_line = 0;

    for line in file_content_lines.lines(){
        let mut current_line = line.unwrap();
        if current_line.contains(&format!("file (GLOB_RECURSE {}_source_files ../bsc_modules/{}/src/*)", &module_name, &module_name)){
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

pub fn change_include_path(file_path: &str, include_path_list: &Vec<String>, module_name: &str){
    let mut file_content_lines = Vec::new();
    common::get_file_content(&file_path, &mut file_content_lines);
    let mut file_new_content_lines = String::new();
    let mut current_index_line = 0;

    for line in file_content_lines.lines(){
        let mut current_line = line.unwrap();
        if current_index_line > 0{
            file_new_content_lines += &String::from("\n");
        }

        for header_path in include_path_list{
            let alternative_header_path: String = String::from(header_path.replace("\\", "/"));
            if current_line.contains(header_path) || current_line.contains(&alternative_header_path){
                let new_line = format!("#include \"{}/{}\"", &module_name, &alternative_header_path);
                current_line = String::from(new_line);
                current_index_line += 1;
                break;
            }
        }
        file_new_content_lines += &String::from(current_line);
        current_index_line += 1;
    }
    common::set_content_file(&file_path, &file_new_content_lines.into_bytes());
}

pub fn get_list_path_include_files(include_folder_path: &str, out_list_headers_files: &mut Vec<String>, module_name: &str){
    let headers = match fs::read_dir(&include_folder_path){
        Err(why) => panic!("Error: couldn't get the content of the {} folder. {}", &include_folder_path, why.description()),
        Ok(headers) => headers,
    };

    for entry in headers{
        let file = match entry{
            Err(_why) => break,
            Ok(file) => file,
        };
        let path_file = file.path();
        let path_file_text = path_file.to_str().unwrap();
        if path_file.is_dir(){ 
            get_list_path_include_files(path_file_text, out_list_headers_files, &module_name); 
        }else{
            let index_begin = path_file_text.find(&include_folder_path).unwrap();
            let index_end = path_file_text.find("include").unwrap();
            let final_header_path = &path_file_text[index_begin+index_end+8+module_name.len()+1..];
            out_list_headers_files.push(final_header_path.to_string());
        }
    }
}

pub fn change_include_path_in_sources_files(folder_path: &str, module_name: &str, list_headers_files: &Vec<String>){
    let sources = match fs::read_dir(&folder_path){
        Err(why) => panic!("Error: couldn't get the content of the {}/src/ folder. {}", &folder_path, why.description()),
        Ok(sources) => sources,
    };

    for entry in sources{
        let file = match entry {
            Err(_why) => break,
            Ok(file) => (file),       
        };
        let path_file = file.path();
        let path_file_text = path_file.to_str().unwrap();
        if !path_file.is_dir(){
            if path_file_text.contains("CMakeLists.txt") { continue; }
            change_include_path(&path_file_text, &list_headers_files, &module_name);
        }else{
            change_include_path_in_sources_files(&path_file_text, &module_name, &list_headers_files);
        }
    }
}

pub fn change_include_path_in_headers_files(folder_path: &str, module_name: &str, list_headers_files: &Vec<String>){
    let headers = match fs::read_dir(&folder_path){
        Err(why) => panic!("Error: couldn't get the content of the {}/include/ folder. {}", &folder_path, why.description()),
        Ok(headers) => headers,
    };

    for entry in headers{
        let file = match entry {
            Err(_why) => break,
            Ok(file) => (file),       
        };
        let path_file = file.path();
        let path_file_text = path_file.to_str().unwrap();
        if !path_file.is_dir(){
            change_include_path(&path_file_text, &list_headers_files, &module_name);
        }else{
            change_include_path_in_headers_files(&path_file_text, &module_name, &list_headers_files);
        }
    }
}
