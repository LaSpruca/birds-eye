use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct User {
    name: String,
}

impl User {
    pub fn name<'a>(&'a self) -> &'a str {
        self.name.as_str()
    }

    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}
