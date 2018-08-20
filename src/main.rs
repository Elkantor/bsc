extern crate clap;
mod create;
mod add;
mod dependencies_handler;

fn main() {
    let matches = clap::App::new("bsc")
        .version("0.1.0")
        .author("Victor Gallet <victor.gallet@hotmail.com>")
        .about("clone of bscxx (a package manager for C++), for C language, made in Rust")
        .subcommand(clap::SubCommand::with_name("create")
            .about("create a new project / module")
            .arg(clap::Arg::with_name("PROJECT_NAME")
                .help("the name of the project / module")
                .required(true)
                .takes_value(true)
                .index(1)))  
        .subcommand(clap::SubCommand::with_name("add")
            .about("add a module to the project")
            .arg(clap::Arg::with_name("MODULE_URL")
                .help("the url of the module")
                .required(true)
                .takes_value(true)
                .index(1)))
        .subcommand(clap::SubCommand::with_name("update")
            .about("update the modules (dependencies) of the project"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("create") {
        // Safe to use unwrap() because of the required() option
        println!("Create file: {}", matches.value_of("PROJECT_NAME").unwrap());
    }
    
    // match matches.subcommand_name() {
    //     Some("create")  => create::create_project("./", matches.value_of("PROJECT_NAME").unwrap()),
    //     Some("add")     => println!("'myapp add' was used"),
    //     Some("update")  => println!("The modules (dependencies) have been correclty updated."),
    //     None            => println!("No subcommand was used"),
    //     _               => println!("Some other subcommand was used"),
    // }

    match matches.subcommand() {
        ("create", Some(create_matches)) => create_new_project("./", create_matches.value_of("PROJECT_NAME").unwrap()),
        ("", None) => println!("No subcommand was used"), // If no subcommand was used it'll match the tuple ("", None)
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }
}

fn create_new_project(path: &str, project_name: &str){
    create::create_project(&path, &project_name);
    dependencies_handler::update_dependencies_file("", &path, &project_name);
}
