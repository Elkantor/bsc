extern crate fs_extra;
use std::fs;
use std::path;
use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::io::BufRead;


/*************/
/* FUNCTIONS */
/*************/
pub fn create_folder(folder_path: &str){
    if path::Path::new(&folder_path.to_string()).exists(){
        return;
    }

    match fs::create_dir(folder_path){
        Err(why) => panic!("Error occured when creating the \"{}\" folder. {}", &folder_path, why.description()),
        Ok(_) => (),
    }
}

pub fn destroy_folder(folder_path: &str){
    if !(path::Path::new(&folder_path.to_string()).exists()){
        return;
    }

    match fs::remove_dir_all(folder_path){
        Err(why) => panic!("Error: please be sure to launch this command as administrator {}", why.description()),
        Ok(_) => (),
    }
}

pub fn copy_folder(folder_previous_path: &str, folder_destination_path: &str){
    let options = fs_extra::dir::CopyOptions {
        overwrite: false, 
        skip_exist: false, 
        buffer_size: 64000, 
        copy_inside: true, 
        depth: 0
    };
    
    match fs_extra::dir::copy(&folder_previous_path, &folder_destination_path, &options){
        Err(why) => panic!("Error: couldn't copy the {} folder inside {}. {}", &folder_previous_path, &folder_destination_path, why.description()),
        Ok(_) => (),
    }
}

pub fn move_content_folder(folder_path: &str, destination_path: &str){
    let content_folder_paths = match fs::read_dir(&folder_path){
        Err(why) => panic!("Error: couldn't get the content of the {} folder. {}", &folder_path, why.description()),
        Ok(content_folder_paths) => content_folder_paths,
    };

    if content_folder_paths.size_hint() == (0, Some(0)) {
        return;
    }

    for entry in content_folder_paths{
        let content = match entry {
            Err(why) => break,
            Ok(content) => (content),       
        };
        let path = content.path();
        let path_text = path.to_str().unwrap();
        let folder_name = path.file_name().unwrap();
        if path.is_dir() {
            copy_folder(&path_text, &destination_path);
            destroy_folder(&path_text);
        }
    }
}

pub fn rename_folder(folder_path: &str, folder_previous_name: &str, folder_new_name: &str){
   match fs::rename(format!("{}{}", &folder_path, &folder_previous_name), format!("{}{}", &folder_path, &folder_new_name)){
        Err(why) => panic!("Error: couldn't rename the {} folder. {}", format!("{}{}", &folder_path, folder_previous_name), why.description()),
        Ok(_) => (),
    }
}

pub fn create_file(file_path: &str, file_name: &str, file_content: &str){
    let mut file = match fs::File::create(format!("{}{}", &file_path, &file_name)) {
        Err(why) => panic!("Error: couldn't create the {} file. {}", format!("{}{}", &file_path, &file_name), why.description()),
        Ok(file) => file,
    };

    match file.write_all(file_content.as_bytes()) {
        Err(why) => panic!("Error: couldn't write to {}. {}", format!("{}{}", &file_path, &file_name), why.description()),
        Ok(_) => (),
    }
}

pub fn get_file_content(file_path: &str, out_file_content: &mut Vec<u8>){
    let mut file = match fs::File::open(&file_path.to_string()){
        Err(why) => panic!("Error: couldn't open the {} file. {}", &file_path, why.description()),
        Ok(file) => (file),
    };
    
    match file.read_to_end(out_file_content){
        Err(why) => panic!("{}", why.description()),
        Ok(_) => (),
    }
}

pub fn set_content_file(file_path: &str, file_content: &Vec<u8>){
    let mut file = match fs::File::create(&file_path) {
        Err(why) => panic!("Error: couldn't create the {} file. {}", &file_path, why.description()),
        Ok(file) => file,
    };

    match file.write_all(file_content) {
        Err(why) => panic!("Error: couldn't write to {}. {}", &file_path, why.description()),
        Ok(_) => (),
    }

}

pub fn get_module_name(module_path: &str, out_module_name: &mut String){
    let mut file_content = Vec::new();
    get_file_content(&format!("{}{}", &module_path, "dependencies.bsc"), &mut file_content);

    let mut name_founded: bool = false;
    for line in file_content.lines() {
        let current_line = line.unwrap();
        if !name_founded{
            if current_line.contains("BSC_PROJECT:") {
                name_founded = true;
            }
        }else{
            let index_begin = current_line.find("[").unwrap();
            let index_end = current_line.find("]").unwrap();
            *out_module_name = current_line[index_begin+1..index_end].to_string(); 
            return;
        }
    }
}