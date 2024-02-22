use core::fmt;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use json::object;
use std::io::{stdout, Write};
use std::io;

pub struct Project {
    pub name: String,
    pub version: String,
    pub description: String,
    pub output: String,
    //scripts: Vec<String>,
    pub author: String,
    pub licence: String,
}

fn read_input(msg: &str) -> String {
    print!("{}", msg);
    stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Error reading stdin");

    let buf = buf.trim();
    buf.to_string()
}

impl fmt::Debug for Project {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Project")
            .field("name", &self.name)
            .field("version", &self.version)
            .field("description", &self.description)
            .field("output", &self.output)
            .field("author", &self.author)
            .field("licence", &self.licence)
            .finish()
    }
}

impl Serialize for Project {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Project", 6)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("version", &self.version)?;
        state.serialize_field("description", &self.description)?;
        state.serialize_field("output", &self.output)?;
        state.serialize_field("author", &self.author)?;
        state.serialize_field("licence", &self.licence)?;
        state.end()
    }
}

impl Project {
    /*
    pub fn new() -> Self {
        Project {
            name: (String::new()),
            version: (String::new()),
            description: (String::new()),
            output: (String::new()),
            author: (String::new()),
            licence: (String::new()),
        }
    }
    */

    pub fn build_project(
        name: String,
        version: String,
        description: String,
        output: String,
        author: String,
        licence: String,
    ) -> Self {
        Self {
            name,
            version,
            description,
            output,
            author,
            licence,
        }
    }

    pub fn to_json(&self) -> String {
        json::stringify_pretty(object! {
            name: self.name.clone(),
            version: self.version.clone(),
            description: self.description.clone(),
            output: self.output.clone(),
        }, 4)
    }
    
    pub fn project_creation() -> Project {   
    
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
}

/* 
pub fn from_string(content: String) -> Project {
    let content = json::parse(&content).unwrap();
    
    Project {
        name: (content["name"].to_string()),
        version: (content["version"].to_string()),
        description: (content["description"].to_string()),
        output: (content["output"].to_string()),
        //scripts: (content["scripts"].to_string()),
        author: (content["author"].to_string()),
        licence: (content["licence"].to_string()),
    }
}
*/
