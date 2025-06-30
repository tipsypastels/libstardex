use crate::RefStr;
use enum_assoc::Assoc;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Builtin(BuiltinType),
    Custom(CustomType),
}

impl Type {
    pub fn new(ty: impl Into<Self>) -> Self {
        ty.into()
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Builtin(ty) => ty.name(),
            Self::Custom(ty) => &ty.name,
        }
    }

    pub fn color(&self) -> &str {
        match self {
            Self::Builtin(ty) => ty.color(),
            Self::Custom(ty) => &ty.color,
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            Self::Builtin(ty) => ty.icon(),
            Self::Custom(ty) => &ty.icon,
        }
    }
}

impl From<BuiltinType> for Type {
    fn from(ty: BuiltinType) -> Self {
        Self::Builtin(ty)
    }
}

impl From<CustomType> for Type {
    fn from(ty: CustomType) -> Self {
        Self::Custom(ty)
    }
}

#[derive(Assoc, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[func(pub const fn name(self) -> &'static str)]
#[func(pub const fn color(self) -> &'static str)]
#[func(pub const fn icon(self) -> &'static str)]
pub enum BuiltinType {
    #[assoc(name = "Normal", color = "rgb(168, 168, 120)", icon = "balance-scale")]
    Normal,
    #[assoc(name = "Fire", color = "rgb(240, 128, 48)", icon = "fire")]
    Fire,
    #[assoc(name = "Fighting", color = "rgb(192, 48, 40)", icon = "fist-raised")]
    Fighting,
    #[assoc(name = "Water", color = "rgb(104, 144, 240)", icon = "water")]
    Water,
    #[assoc(name = "Flying", color = "rgb(168, 144, 240)", icon = "feather-alt")]
    Flying,
    #[assoc(name = "Grass", color = "rgb(120, 200, 80)", icon = "leaf")]
    Grass,
    #[assoc(name = "Poison", color = "rgb(160, 64, 160)", icon = "flask")]
    Poison,
    #[assoc(name = "Electric", color = "rgb(248, 208, 48)", icon = "bolt")]
    Electric,
    #[assoc(name = "Ground", color = "rgb(224, 192, 104)", icon = "globe")]
    Ground,
    #[assoc(name = "Psychic", color = "rgb(248, 88, 136)", icon = "eye")]
    Psychic,
    #[assoc(name = "Rock", color = "rgb(184, 160, 56)", icon = "gem")]
    Rock,
    #[assoc(name = "Ice", color = "rgb(152, 216, 216)", icon = "icicles")]
    Ice,
    #[assoc(name = "Bug", color = "rgb(168, 184, 32)", icon = "bug")]
    Bug,
    #[assoc(name = "Dragon", color = "rgb(112, 56, 248)", icon = "dragon")]
    Dragon,
    #[assoc(name = "Ghost", color = "rgb(112, 88, 152)", icon = "ghost")]
    Ghost,
    #[assoc(name = "Dark", color = "rgb(112, 88, 72)", icon = "moon")]
    Dark,
    #[assoc(name = "Steel", color = "rgb(184, 184, 208)", icon = "robot")]
    Steel,
    #[assoc(name = "Fairy", color = "rgb(238, 153, 172)", icon = "sparkles")]
    Fairy,
}

impl fmt::Display for BuiltinType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CustomType {
    pub name: RefStr,
    pub color: RefStr,
    pub icon: RefStr,
}

impl fmt::Display for CustomType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}
