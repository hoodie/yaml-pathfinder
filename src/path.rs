pub struct Paths(pub String);

pub struct YPaths<'a>(pub &'a str);

impl Paths {
    pub fn as_ref<'a>(&'a self) -> YPaths<'a> {
        YPaths(self.0.as_ref())
    }
}

impl YPaths<'_> {
    fn str(&self) -> &str {
        self.0
    }

    pub fn alternatives<'a>(&'a self) -> impl Iterator<Item = YPath<'a>> {
        self.str().split('|').map(YPath)
    }
}

impl<'a> From<&'a str> for YPaths<'a> {
    fn from(s: &'a str) -> YPaths<'a> {
        YPaths(s)
    }
}

impl From<&str> for Paths {
    fn from(s: &str) -> Paths {
        Paths(s.into())
    }
}

#[derive(Clone, Copy)]
pub struct YPath<'a>(pub &'a str);

impl YPath<'_> {
    fn str(&self) -> &str {
        self.0
    }

    pub fn elements(&self) -> impl Iterator<Item = &str> {
        self.str()
            .split(|p| p == '/' || p == '.')
            .filter(|k| !k.is_empty())
    }
}

impl<'a> AsRef<str> for YPaths<'a> {
    fn as_ref(&self) -> &str {
        self.0
    }
}
impl<'a> AsRef<str> for YPath<'a> {
    fn as_ref(&self) -> &str {
        self.0
    }
}
