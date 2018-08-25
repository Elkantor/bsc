extern crate git2;
#[path = "common.rs"] mod common;
use std::error::Error;


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
    common::create_folder(&format!("{}/bsc_modules", &path));
    common::destroy_folder(&format!("{}/bsc_modules/tmp", &path));
    copy_module_to_tmp(&path, &module_url, module_type);
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

pub fn get_module_name(){
    
}

pub fn add_module_header_to_main_cmakelists_file(){

}

