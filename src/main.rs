extern crate clap;
mod create;
mod add;

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
    
    match matches.subcommand_name() {
        Some("create")  => create::create_project(),
        Some("add")     => println!("'myapp add' was used"),
        Some("update")  => println!("The modules (dependencies) have been correclty updated."),
        None            => println!("No subcommand was used"),
        _               => println!("Some other subcommand was used"),
    }

}
