enum Version {
    Undefined,
    Ver71,
    Ver72,
    Ver73,
    Ver74,
    Ver75,
    Ver76,
    Ver77,
    Ver78,
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
