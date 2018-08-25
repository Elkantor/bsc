extern crate fs_extra;
use std::fs;
use std::path;
use std::error::Error;
use std::io::prelude::*;

/*************/
/* FUNCTIONS */
/*************/
pub fn create_folder(folder_path: &str){
    if path::Path::new(&folder_path.to_string()).exists(){
        return;
    }

    match fs::create_dir(folder_path){
        Err(why) => println!("Error occured when creating the \"{}\" folder. {}", &folder_path, why.description()),
        Ok(_) => (),
    }
}

pub fn destroy_folder(folder_path: &str){
    if !(path::Path::new(&folder_path.to_string()).exists()){
        return;
    }

    match fs::remove_dir_all(folder_path){
        Err(why) => println!("Error: please be sure to launch this command as administrator {}", why.description()),
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
    fs_extra::dir::copy(&folder_previous_path, &folder_destination_path, &options);
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

// pub fn get_file_content(file_path: &str, ){}