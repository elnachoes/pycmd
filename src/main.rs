use std::{
    env::{ args, current_exe },
    io::{ prelude::*, Write },
    process::Command, 
    path::PathBuf,
    fs::OpenOptions,
};

// this will load the PathBuf of the installed programs json which should be stored in the pycmd.exe dir
fn load_exe_path_buf() -> PathBuf {
    let mut exe_path_buf = current_exe().unwrap();
    exe_path_buf.pop();
    exe_path_buf.push("installed_programs.json");
    exe_path_buf
}

fn install_python_program(program : &str, program_path : &str, program_dir : &str) {
    let exe_path_buf = load_exe_path_buf();

    // read in the current programs file
    let mut installed_programs_file = OpenOptions::new().create(true).read(true).write(true).open(exe_path_buf.clone()).unwrap();

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
    programs_json[program] = json::object!{
        program_path : program_path,
        program_dir : program_dir
    };

    // reopen the file and wipe it.
    installed_programs_file = OpenOptions::new().write(true).truncate(true).open(exe_path_buf.clone()).unwrap();

    // write all the contents out to the installed programs file
    installed_programs_file.write_all(json::stringify_pretty(programs_json, 4).as_bytes()).unwrap();
}

// this will launch a program stored within the installed programs json
fn launch_program(program : &String) -> Result<(), &str> {
    let exe_path_buf = load_exe_path_buf();

    // try opening up the installed programs file
    let mut installed_programs_file = match OpenOptions::new().create(true).read(true).write(true).open(exe_path_buf.clone()) 
    {
        Err(_) => return Err("error - no programs installed"),
        Ok(file) => file
    };

    // read the file contents into a string that can be parsed
    let mut installed_programs_file_contents = String::new();
    installed_programs_file.read_to_string(&mut installed_programs_file_contents).unwrap();

    // parse the file contents into a json
    let programs_json = json::parse(&installed_programs_file_contents).unwrap();

    let program = &programs_json[program.as_str()];

    // run the program as a spawned process
    Command::new("python").arg(program["program_path"].as_str().unwrap()).current_dir(program["program_dir"].as_str().unwrap()).spawn().unwrap();

    Ok(())
}

fn main() {
    let args : Vec<String> = args().map(|x| String::from(x)).collect();

    if args.len() == 5 && args[1] == "-i".to_string() {
        install_python_program(&args[2], &args[3],&args[4]);
    }
    else if args.len() == 2 {
        match launch_program(&args[1]) {
            Err(error) => println!("{}", error),
            _ => {}           
        }
    }
    else {
        println!("error - you must install or launch a program");
    }
}
