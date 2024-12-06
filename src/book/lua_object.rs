use std::{error::Error, fmt::Display, fs, io, path::Path};

use mlua::{
    AsChunk, FromLuaMulti, Function, IntoLua, IntoLuaMulti, Lua, LuaOptions, Result as LuaResult,
    StdLib,
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

pub trait FromLua: Sized {
    fn from_chunk<'a>(name: NonEmptyString, chunk: impl AsChunk<'a>) -> LuaResult<Self>;
    fn from_file<P>(path: P) -> Result<Self, IOLuaError>
    where
        P: AsRef<Path>;
}

impl<T> FromLua for T
where
    T: From<LuaObject>,
{
    fn from_chunk<'a>(name: NonEmptyString, chunk: impl AsChunk<'a>) -> LuaResult<Self> {
        let options = LuaOptions::new();
        let libs = StdLib::ALL_SAFE;
        let lua = Lua::new_with(libs, options)?;
        lua.sandbox(true)?;
        lua.load(chunk).exec()?;
        Ok(T::from(LuaObject { name, lua }))
    }

    fn from_file<P>(path: P) -> Result<Self, IOLuaError>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        let file_name = path.file_name().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Failed to extract file name from path: {:?}", path),
            )
        })?;

        let name =
            NonEmptyString::new(file_name.to_string_lossy().to_string()).map_err(|_err| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Extracted file name is empty or invalid: {:?}",
                        file_name.to_string_lossy()
                    ),
                )
            })?;

        let chunk = fs::read(path)?;
        Ok(Self::from_chunk(name, chunk)?)
    }
}

#[derive(Debug)]
pub enum IOLuaError {
    IO(io::Error),
    Lua(mlua::Error),
}

impl Error for IOLuaError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            IOLuaError::IO(error) => error.source(),
            IOLuaError::Lua(error) => error.source(),
        }
    }
}

impl Display for IOLuaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IOLuaError::IO(error) => error.fmt(f),
            IOLuaError::Lua(error) => error.fmt(f),
        }
    }
}

impl From<io::Error> for IOLuaError {
    fn from(value: io::Error) -> Self {
        IOLuaError::IO(value)
    }
}

impl From<mlua::Error> for IOLuaError {
    fn from(value: mlua::Error) -> Self {
        IOLuaError::Lua(value)
    }
}
