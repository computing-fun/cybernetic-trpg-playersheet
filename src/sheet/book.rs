use super::character::ClassLevel;

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct Src(mlua::Lua);
impl Src {
    pub fn load<'a>(chunk: impl mlua::AsChunk<'a>) -> mlua::Result<Self> {
        let options = mlua::LuaOptions::new();
        let libs = mlua::StdLib::ALL_SAFE;
        let lua = mlua::Lua::new_with(libs, options)?;
        lua.sandbox(true)?;
        lua.load(chunk).exec()?;
        Ok(Src(lua))
    }

    pub fn src(&self) -> &mlua::Lua {
        &self.0
    }

    pub fn get<T>(&self, key: impl mlua::IntoLua) -> mlua::Result<T>
    where
        T: mlua::FromLua,
    {
        self.src().globals().get::<T>(key)
    }

    pub fn set<T>(&self, key: impl mlua::IntoLua, value: impl mlua::IntoLua) -> mlua::Result<()>
    where
        T: mlua::FromLua,
    {
        self.src().globals().set(key, value)
    }

    pub fn func(&self, key: impl mlua::IntoLua) -> mlua::Result<mlua::Function> {
        self.get::<mlua::Function>(key)
    }

    pub fn call<T>(&self, key: impl mlua::IntoLua, args: impl mlua::IntoLuaMulti) -> mlua::Result<T>
    where
        T: mlua::FromLuaMulti,
    {
        self.func(key)?.call(args)
    }
}

pub trait Page<'a>: std::fmt::Debug + Clone {
    fn new(name: &'a str, src: &'a Src) -> Self
    where
        Self: Sized;
    fn name(&self) -> &str;
    fn src(&self) -> &Src;

    fn description(&self) -> Box<str> {
        self.src().get("Description").unwrap_or_default()
    }

    fn astralic_types(&self) -> Vec<Box<str>> {
        self.src().get("AstralicTypes").unwrap_or_default()
    }

    fn saving_throws(&self) -> Vec<Box<str>> {
        self.src().get("SavingThrows").unwrap_or_default()
    }

    fn skills(&self, class_level: ClassLevel) -> Vec<Box<str>> {
        self.src().call("Skills", class_level).unwrap_or_default()
    }

    fn cybernetics(&self, class_level: ClassLevel) -> Vec<Box<str>> {
        self.src()
            .call("Cybernetics", class_level)
            .unwrap_or_default()
    }

    fn health(&self, class_level: ClassLevel) -> usize {
        self.src().call("Health", class_level).unwrap_or_default()
    }

    fn armor_rating(&self, class_level: ClassLevel) -> usize {
        self.src()
            .call("ArmorRating", class_level)
            .unwrap_or_default()
    }

    fn spell_level(&self, class_level: ClassLevel) -> usize {
        self.src()
            .call("SpellLevel", class_level)
            .unwrap_or_default()
    }
}

macro_rules! simple_sheets {
    ($($name:ident),*) => {
        $(
            #[derive(Debug, Clone)]
            pub struct $name<'a> {
                name: &'a str,
                src: &'a Src,
            }

            impl<'a> Page<'a> for $name<'a> {
                fn new(name: &'a str, src: &'a Src) -> Self {
                    Self { name, src }
                }

                fn name(&self) -> &str {
                    self.name
                }

                fn src(&self) -> &Src {
                    self.src
                }
            }

        )*
    };
}
simple_sheets!(RaceSheet, ClassSheet, BalanceSheet, CyberneticSheet);

#[derive(Debug)]
pub struct Section<'a, P>
where
    P: Page<'a>,
{
    sections: std::collections::HashMap<Box<str>, Src>,
    _page: std::marker::PhantomData<&'a P>,
}

impl<'a, P> Section<'a, P>
where
    P: Page<'a>,
{
    pub fn new() -> Self {
        Self {
            sections: std::collections::HashMap::new(),
            _page: std::marker::PhantomData::default(),
        }
    }

    pub fn read(&'a self, name: &'a str) -> Option<P> {
        Some(P::new(name, self.sections.get(name)?))
    }

    pub fn write(&mut self, name: Box<str>, src: Src) {
        self.sections.insert(name, src);
    }

    pub fn iter(&'a self) -> impl Iterator<Item = P> + 'a
    where
        P: 'a,
    {
        self.sections.iter().map(|(name, src)| P::new(name, src))
    }
}

#[derive(Debug)]
pub struct Book<'a> {
    errors: Vec<String>,
    pub race: Section<'a, RaceSheet<'a>>,
    pub class: Section<'a, ClassSheet<'a>>,
    pub balance: Section<'a, BalanceSheet<'a>>,
    pub cybernetics: Section<'a, CyberneticSheet<'a>>,
}

impl Book<'_> {
    pub fn new() -> Self {
        Self {
            errors: vec![],
            race: Section::new(),
            class: Section::new(),
            balance: Section::new(),
            cybernetics: Section::new(),
        }
    }
}
