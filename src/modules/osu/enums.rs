use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

// 定义关于OSU 4种模式的枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
    Osu,
    Taiko,
    Ctb,
    Mania,
}

impl Mode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Mode::Osu => "osu",
            Mode::Taiko => "taiko",
            Mode::Ctb => "ctb",
            Mode::Mania => "mania",
        }
    }
}

impl FromStr for Mode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "osu" => Ok(Mode::Osu),
            "taiko" => Ok(Mode::Taiko),
            "ctb" => Ok(Mode::Ctb),
            "mania" => Ok(Mode::Mania),
            _ => Err(anyhow::anyhow!("Invalid mode")),
        }
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
