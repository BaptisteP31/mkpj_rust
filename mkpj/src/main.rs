extern crate mustache;

mod config;
use std::fs;

use config::Project;


fn main() {
    let project = Project::project_creation();

    let file_name = format!("{}.json", project.name);
    fs::write(file_name, project.to_json()).expect("Error writing file");

    let readme_template = mustache::compile_str(
    &fs::read_to_string("README.md.mustache")
                .expect("Error reading file")
    ).unwrap();

    let mut rendered = Vec::new();
    readme_template.render(&mut rendered, &project).unwrap();
    let rendered = String::from_utf8(rendered).unwrap();

    fs::write("README.md", rendered).expect("Error writing file");
}
