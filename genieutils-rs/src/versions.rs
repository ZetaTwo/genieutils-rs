use binrw::{BinRead, BinWrite};

#[derive(Clone, Copy, PartialEq, PartialOrd, BinRead, BinWrite)]
#[cfg_attr(test, derive(Debug))]
#[brw(little)]
pub enum Version {
    #[brw(magic = b"VER 7.1\x00")]
    Ver71 = 71,
    #[brw(magic = b"VER 7.2\x00")]
    Ver72 = 72,
    #[brw(magic = b"VER 7.3\x00")]
    Ver73 = 73,
    #[brw(magic = b"VER 7.4\x00")]
    Ver74 = 74,
    #[brw(magic = b"VER 7.5\x00")]
    Ver75 = 75,
    #[brw(magic = b"VER 7.6\x00")]
    Ver76 = 76,
    #[brw(magic = b"VER 7.7\x00")]
    Ver77 = 77,
    #[brw(magic = b"VER 7.8\x00")]
    Ver78 = 78,
    Undefined = 0,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_version() {
        let version = Version::read(&mut std::io::Cursor::new(b"VER 7.8\x00")).unwrap();
        assert_eq!(Version::Ver78, version);
    }

    #[test]
    fn parse_version_invalid() {
        let version = Version::read(&mut std::io::Cursor::new(b"VER 1.1\x00")).unwrap();
        assert_eq!(Version::Undefined, version);
    }
}
