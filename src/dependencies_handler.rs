#[path = "common.rs"] mod common;


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