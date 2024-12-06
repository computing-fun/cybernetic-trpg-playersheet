use std::usize;

use non_empty_string::NonEmptyString;
use serde::{Deserialize, Serialize};

use super::lua_object;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct CharacterClass {
    pub name: String,
    pub balance: String,
    pub level: ClassLevel,
}

type ClassLevel = usize;

#[derive(Debug)]
pub struct Class(lua_object::LuaObject);
impl Class {
    pub fn name(&self) -> &NonEmptyString {
        self.0.name()
    }

    pub fn description(&self, class_level: ClassLevel) -> String {
        self.0.call("description", class_level).unwrap_or_default()
    }

    pub fn astralic_types(&self, class_level: ClassLevel) -> Vec<NonEmptyString> {
        self.0
            .call_non_empty_strings("astralic_types", class_level)
            .unwrap_or_default()
    }

    pub fn saving_throws(&self, class_level: ClassLevel) -> Vec<NonEmptyString> {
        self.0
            .call_non_empty_strings("saving_throws", class_level)
            .unwrap_or_default()
    }
}

impl From<lua_object::LuaObject> for Class {
    fn from(value: lua_object::LuaObject) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
pub struct Balance(lua_object::LuaObject);
impl Balance {
    pub fn new(lua: lua_object::LuaObject) -> Self {
        Self(lua)
    }

    pub fn name(&self) -> &NonEmptyString {
        self.0.name()
    }

    pub fn health(&self, class_level: ClassLevel) -> usize {
        self.0.call("health", class_level).unwrap_or_default()
    }

    pub fn armor_rating(&self, class_level: ClassLevel) -> usize {
        self.0.call("armor_rating", class_level).unwrap_or_default()
    }

    pub fn spell_level(&self, class_level: ClassLevel) -> usize {
        self.0.call("spell_level", class_level).unwrap_or_default()
    }
}

impl From<lua_object::LuaObject> for Balance {
    fn from(value: lua_object::LuaObject) -> Self {
        Self(value)
    }
}
