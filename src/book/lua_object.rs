use mlua::{
    AsChunk, FromLua, FromLuaMulti, Function, IntoLua, IntoLuaMulti, Lua, LuaOptions,
    Result as LuaResult, StdLib,
};
use non_empty_string::NonEmptyString;

#[derive(Debug, Clone)]
pub struct LuaObject {
    name: NonEmptyString,
    lua: Lua,
}

impl LuaObject {
    pub fn name(&self) -> &NonEmptyString {
        &self.name
    }

    pub fn get<T>(&self, key: impl IntoLua) -> LuaResult<T>
    where
        T: FromLua,
    {
        self.lua.globals().get::<T>(key)
    }

    pub fn get_non_empty_strings(&self, key: impl IntoLua) -> LuaResult<Vec<NonEmptyString>> {
        Ok(self
            .get::<Vec<String>>(key)?
            .into_iter()
            .flat_map(NonEmptyString::new)
            .collect())
    }

    pub fn call<T>(&self, key: impl IntoLua, args: impl IntoLuaMulti) -> LuaResult<T>
    where
        T: FromLuaMulti,
    {
        self.lua.globals().get::<Function>(key)?.call(args)
    }

    pub fn call_non_empty_strings(
        &self,
        key: impl IntoLua,
        args: impl IntoLuaMulti,
    ) -> LuaResult<Vec<NonEmptyString>> {
        Ok(self
            .call::<Vec<String>>(key, args)?
            .into_iter()
            .flat_map(NonEmptyString::new)
            .collect())
    }
}

pub trait FromLuaChunk: Sized {
    fn from_lua_chunk<'a>(name: NonEmptyString, chunk: impl AsChunk<'a>) -> LuaResult<Self>;
}

impl<T> FromLuaChunk for T
where
    T: From<LuaObject>,
{
    fn from_lua_chunk<'a>(name: NonEmptyString, chunk: impl AsChunk<'a>) -> LuaResult<Self> {
        let options = LuaOptions::new();
        let libs = StdLib::ALL_SAFE;
        let lua = Lua::new_with(libs, options)?;
        lua.sandbox(true)?;
        lua.load(chunk).exec()?;
        Ok(T::from(LuaObject { name, lua }))
    }
}

#[derive(Debug)]
pub enum IOLuaError {
    IO(std::io::Error),
    Lua(mlua::Error),
}

impl std::error::Error for IOLuaError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            IOLuaError::IO(error) => error.source(),
            IOLuaError::Lua(error) => error.source(),
        }
    }
}

impl std::fmt::Display for IOLuaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IOLuaError::IO(error) => error.fmt(f),
            IOLuaError::Lua(error) => error.fmt(f),
        }
    }
}

impl From<std::io::Error> for IOLuaError {
    fn from(value: std::io::Error) -> Self {
        IOLuaError::IO(value)
    }
}

impl From<mlua::Error> for IOLuaError {
    fn from(value: mlua::Error) -> Self {
        IOLuaError::Lua(value)
    }
}
