use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    pub name: Box<str>,
    pub description: Box<str>,
    pub race: Race,
    pub classes: Vec<Class>,
    pub cybernetics: Vec<Cybernetic>,
}

pub type CharacterLevel = usize;

impl Character {
    pub fn level(&self) -> CharacterLevel {
        self.classes.iter().map(|class| class.level).sum()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Race {
    pub name: Box<str>,
    pub info: Box<str>,
    pub age: Box<str>,
    pub size: Box<str>,
    pub speed: usize,
    pub languages: Vec<Box<str>>,
    // sub race
}

pub type ClassLevel = usize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Class {
    pub name: Box<str>,
    pub balance: Box<str>,
    pub level: ClassLevel,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cybernetic {
    pub name: Box<str>,
}
