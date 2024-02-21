mod config;
use std::io::{stdout, Write};
use std::io;
use std::fs;

use config::Project;

fn read_input(msg: &str) -> String {
    print!("{}", msg);
    stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Error reading stdin");

    let buf = buf.trim();
    buf.to_string()
}

fn project_creation() -> Project {   

    let questions = [
        "Project name: ",
        "Project version: ",
        "Project decription: ",
        "Output file name: ",
        "Author: ",
        "Licence: ",
    ];

    let mut responses: Vec<String> = Vec::new();
    for question in questions {
        responses.push(
            read_input(question)
        )
    }

    if responses.len() != 6 {
        panic!("Error: Expected 6 responses, got {}", responses.len());
    }

    Project::build_project(
        responses[0].clone(),
        responses[1].clone(),
        responses[2].clone(),
        responses[3].clone(),
        responses[4].clone(),
        responses[5].clone(),
    )
}

fn main() {
    let project = project_creation();

    let file_name = format!("{}.json", project.name);
    fs::write(file_name, project.to_json()).expect("Error writing file");
}
