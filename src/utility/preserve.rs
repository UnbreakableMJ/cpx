#[derive(Debug, Clone, Copy)]
pub struct PreserveAttr {
    pub mode: bool,
    pub ownership: bool,
    pub timestamps: bool,
    pub links: bool,
    pub context: bool,
    pub xattr: bool,
}

impl Default for PreserveAttr {
    fn default() -> Self {
        Self {
            mode: true,
            ownership: true,
            timestamps: true,
            links: false,
            context: false,
            xattr: false,
        }
    }
}

impl PreserveAttr {
    pub fn none() -> Self {
        Self {
            mode: false,
            ownership: false,
            timestamps: false,
            links: false,
            context: false,
            xattr: false,
        }
    }

    pub fn all() -> Self {
        Self {
            mode: true,
            ownership: true,
            timestamps: true,
            links: true,
            context: true,
            xattr: true,
        }
    }

    pub fn from_string(s: &str) -> Result<Self, String> {
        if s.is_empty() {
            return Ok(Self::default());
        }

        if s == "all" {
            return Ok(Self::all());
        }

        let mut attr = Self::none();

        for cur in s.split(',') {
            match cur.trim() {
                "" => continue,
                "mode" => attr.mode = true,
                "ownership" => attr.ownership = true,
                "timestamps" => attr.timestamps = true,
                "xattr" => attr.xattr = true,
                "context" => attr.context = true,
                "links" => attr.links = true,
                "all" => return Ok(Self::all()),
                other => return Err(format!("Unknown attribute: {}", other)),
            }
        }

        Ok(attr)
    }
}
