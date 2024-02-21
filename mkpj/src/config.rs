use core::fmt;

use json::object;

pub struct Project {
    pub name: String,
    pub version: String,
    pub description: String,
    pub output: String,
    //scripts: Vec<String>,
    pub author: String,
    pub licence: String,
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

impl Project {
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
}

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
