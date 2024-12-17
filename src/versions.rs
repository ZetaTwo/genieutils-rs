#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Version {
    Undefined = 0,
    Ver71 = 71,
    Ver72 = 72,
    Ver73 = 73,
    Ver74 = 74,
    Ver75 = 75,
    Ver76 = 76,
    Ver77 = 77,
    Ver78 = 78,
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::Undefined => write!(f, "UNDEFINED"),
            Self::Ver71 => write!(f, "VER 7.1"),
            Self::Ver72 => write!(f, "VER 7.2"),
            Self::Ver73 => write!(f, "VER 7.3"),
            Self::Ver74 => write!(f, "VER 7.4"),
            Self::Ver75 => write!(f, "VER 7.5"),
            Self::Ver76 => write!(f, "VER 7.6"),
            Self::Ver77 => write!(f, "VER 7.7"),
            Self::Ver78 => write!(f, "VER 7.8"),
        }
    }
}

impl TryFrom<String> for Version {
    type Error = std::io::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "VER 7.1" => Ok(Self::Ver71),
            "VER 7.2" => Ok(Self::Ver72),
            "VER 7.3" => Ok(Self::Ver73),
            "VER 7.4" => Ok(Self::Ver74),
            "VER 7.5" => Ok(Self::Ver75),
            "VER 7.6" => Ok(Self::Ver76),
            "VER 7.7" => Ok(Self::Ver77),
            "VER 7.8" => Ok(Self::Ver78),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "unknown version",
            )),
        }
    }
}
