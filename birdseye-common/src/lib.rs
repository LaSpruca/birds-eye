use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
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


/// Serializable process struct to describe process used inside of crate
#[derive(Clone, Serialize, Debug)]
pub struct Process {
    pid: u32,
    name: String,
    user: User,
}

impl<'a> Process {
    pub fn pid(&'a self) -> &'a u32 {
        &self.pid
    }

    pub fn name(&'a self) -> &'a str {
        &self.name
    }

    pub fn user(&'a self) -> &'a User {
        &self.user
    }

    pub fn new(pid: u32, name: &str, user: &User) -> Self {
        Self { pid, name: name.to_string(), user: user.clone() }
    }
}