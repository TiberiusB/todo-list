#[derive(Debug)]

pub struct Personne {
    // "pub" is for making the content here accessible from another module, or the main. struct is a keyword that builds a structure, a custom type, allowing one to bundle various data types, key-value pairs.
    pub name: String,
    pub age: u8,
    pub job: Option<String>,
}

impl Personne {
    pub fn new(name: &str, age: u8) -> Self {
        Self {
            name: name.to_string(),
            age: (age),
            job: None,
        }
    }

    pub fn birthday(&mut self) {
        self.age += 1;
    }
}
