use non_empty_string::NonEmptyString;

use super::lua_object;

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct CharacterClass {
    pub name: String,
    pub balance: String,
    pub level: ClassLevel,
}

type ClassLevel = usize;

#[derive(Debug, Clone)]
pub struct Class(lua_object::LuaObject);
impl Class {
    pub fn name(&self) -> &NonEmptyString {
        self.0.name()
    }

    pub fn description(&self) -> String {
        self.0.get("Description").unwrap_or_default()
    }

    pub fn astralic_types(&self) -> Vec<NonEmptyString> {
        self.0
            .get_non_empty_strings("AstralicTypes")
            .unwrap_or_default()
    }

    pub fn saving_throws(&self) -> Vec<NonEmptyString> {
        self.0
            .get_non_empty_strings("SavingThrows")
            .unwrap_or_default()
    }

    pub fn skills(&self, class_level: ClassLevel) -> Vec<NonEmptyString> {
        self.0
            .call_non_empty_strings("Skills", class_level)
            .unwrap_or_default()
    }

    pub fn cybernetics(&self, class_level: ClassLevel) -> Vec<NonEmptyString> {
        self.0
            .call_non_empty_strings("Cybernetics", class_level)
            .unwrap_or_default()
    }
}

impl From<lua_object::LuaObject> for Class {
    fn from(value: lua_object::LuaObject) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone)]
pub struct Balance(lua_object::LuaObject);
impl Balance {
    pub fn name(&self) -> &NonEmptyString {
        self.0.name()
    }

    pub fn description(&self) -> String {
        self.0.get("Description").unwrap_or_default()
    }

    pub fn health(&self, class_level: ClassLevel) -> usize {
        self.0.call("Health", class_level).unwrap_or_default()
    }

    pub fn armor_rating(&self, class_level: ClassLevel) -> usize {
        self.0.call("ArmorRating", class_level).unwrap_or_default()
    }

    pub fn spell_level(&self, class_level: ClassLevel) -> usize {
        self.0.call("SpellLevel", class_level).unwrap_or_default()
    }

    pub fn skills(&self, class_level: ClassLevel) -> Vec<NonEmptyString> {
        self.0
            .call_non_empty_strings("Skills", class_level)
            .unwrap_or_default()
    }
}

impl From<lua_object::LuaObject> for Balance {
    fn from(value: lua_object::LuaObject) -> Self {
        Self(value)
    }
}
