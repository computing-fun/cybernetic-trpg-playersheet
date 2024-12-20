use super::character::ClassLevel;

#[derive(Debug, Clone)]
pub(crate) struct Src {
    name: Box<str>,
    lua: mlua::Lua,
}

impl Src {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn get<T>(&self, key: impl mlua::IntoLua) -> mlua::Result<T>
    where
        T: mlua::FromLua,
    {
        self.lua.globals().get::<T>(key)
    }

    pub fn set<T>(&self, key: impl mlua::IntoLua, value: impl mlua::IntoLua) -> mlua::Result<()>
    where
        T: mlua::FromLua,
    {
        self.lua.globals().set(key, value)
    }

    pub fn call<T>(&self, key: impl mlua::IntoLua, args: impl mlua::IntoLuaMulti) -> mlua::Result<T>
    where
        T: mlua::FromLuaMulti,
    {
        self.get::<mlua::Function>(key)?.call(args)
    }
}

pub trait Loader: Sized {
    fn load<'a>(
        name: impl Into<Box<str>>,
        chunk: impl mlua::AsChunk<'a>,
    ) -> mlua::Result<Result<Self, impl std::error::Error>>;
}

impl<T> Loader for T
where
    T: TryFrom<Src>,
    T::Error: std::error::Error,
{
    fn load<'a>(
        name: impl Into<Box<str>>,
        chunk: impl mlua::AsChunk<'a>,
    ) -> Result<Result<Self, impl std::error::Error>, mlua::Error> {
        let options = mlua::LuaOptions::new();
        let libs = mlua::StdLib::ALL_SAFE;
        let lua = mlua::Lua::new_with(libs, options)?;
        lua.sandbox(true)?;
        lua.load(chunk).exec()?;
        let src = Src {
            name: name.into(),
            lua,
        };
        Ok(T::try_from(src))
    }
}

#[derive(Debug, Clone)]
pub struct Class(Src);
impl From<Src> for Class {
    fn from(value: Src) -> Self {
        Self(value)
    }
}
impl Class {
    pub fn name(&self) -> &str {
        self.0.name()
    }

    pub fn description(&self) -> String {
        self.0.get("Description").unwrap_or_default()
    }

    pub fn astralic_types(&self) -> Vec<Box<str>> {
        self.0.get("AstralicTypes").unwrap_or_default()
    }

    pub fn saving_throws(&self) -> Vec<Box<str>> {
        self.0.get("SavingThrows").unwrap_or_default()
    }

    pub fn skills(&self, class_level: ClassLevel) -> Vec<Box<str>> {
        self.0.call("Skills", class_level).unwrap_or_default()
    }

    pub fn cybernetics(&self, class_level: ClassLevel) -> Vec<Box<str>> {
        self.0.call("Cybernetics", class_level).unwrap_or_default()
    }
}

#[derive(Debug, Clone)]
pub struct Balance(Src);
impl From<Src> for Balance {
    fn from(value: Src) -> Self {
        Self(value)
    }
}
impl Balance {
    pub fn name(&self) -> &str {
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

    pub fn skills(&self, class_level: ClassLevel) -> Vec<Box<str>> {
        self.0.call("Skills", class_level).unwrap_or_default()
    }
}

#[derive(Debug, Clone)]
pub struct Cybernetic(Src);
impl From<Src> for Cybernetic {
    fn from(value: Src) -> Self {
        Self(value)
    }
}
impl Cybernetic {
    pub fn name(&self) -> &str {
        self.0.name()
    }

    pub fn body_parts(&self) -> Vec<Box<str>> {
        todo!()
    }
}
