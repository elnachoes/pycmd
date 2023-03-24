use std::{
    env,
    fs,
    io::prelude::*,
    io::Write,
    process::Command
};

fn install_python_program(args : &Vec<String>) {
    // read in the current programs file
    let mut installed_programs_file = fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open("installed_programs.json")
        .unwrap();

    // read the file contents into a string that can be parsed
    let mut installed_programs_file_contents = String::new();
    installed_programs_file.read_to_string(&mut installed_programs_file_contents).unwrap();

    // if the file is new and or empty go ahead and set the programs json to an empty json object else parse in from the file contents
    let mut programs_json = if installed_programs_file_contents.len() == 0 {
        json::object!{}
    }
    else {
        json::parse(&installed_programs_file_contents).unwrap()
    };

    // add in the new program to the json
    programs_json[args[2].as_str()] = json::object!{
        program_path : args[3].as_str(),
        program_dir : args[4].as_str()
    };

    // reopen the file and wipe it.
    installed_programs_file = fs::OpenOptions::new().write(true).truncate(true).open("installed_programs.json").unwrap();

    // write all the contents out to the installed programs file
    installed_programs_file.write_all(json::stringify_pretty(programs_json, 4).as_bytes()).unwrap();
}

//TODO : fix this to run from the dir
fn launch_program(program : &String) {
    // read in the current programs file
    let mut installed_programs_file = fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open("installed_programs.json")
        .unwrap();

    // read the file contents into a string that can be parsed
    let mut installed_programs_file_contents = String::new();
    installed_programs_file.read_to_string(&mut installed_programs_file_contents).unwrap();

    let programs_json = json::parse(&installed_programs_file_contents).unwrap();

    let program = &programs_json[program.as_str()];

    Command::new("python")
        .arg(program["program_path"].as_str().unwrap())
        .current_dir(program["program_dir"].as_str().unwrap())
        .spawn()
        .unwrap();
}

fn main() {
    let args : Vec<String> = env::args().map(|x| String::from(x)).collect();
    if args[1] == "-i".to_string() {
        install_python_program(&args);
    }
    else {
        launch_program(&args[1])
    }
}
