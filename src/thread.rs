pub trait Thread {
    fn rgb<'a>(&'a self) -> &'a [u8; 3];
    fn brand<'a>(&'a self) -> &'a str;
    fn code<'a>(&'a self) -> &'a str;
    fn name<'a>(&'a self) -> &'a str;
    fn to_ref<'a>(&'a self) -> ThreadRef<'a>;
    fn to_owned(&self) -> OwnedThread;
}

pub struct ThreadRef<'a> {
    brand: &'a str,
    name: &'a str,
    code: &'a str,
    color: &'a [u8; 3],
}

impl<'a> ThreadRef<'a> {
    pub const fn new(brand: &'a str, name: &'a str, code: &'a str, color: &'a [u8; 3]) -> Self {
        Self {
            brand,
            name,
            code,
            color,
        }
    }
}

impl<'l> Thread for ThreadRef<'l> {
    fn rgb<'a>(&'a self) -> &'a [u8; 3] {
        self.color
    }
    fn brand<'a>(&'a self) -> &'a str {
        self.brand
    }
    fn code<'a>(&'a self) -> &'a str {
        self.code
    }
    fn name<'a>(&'a self) -> &'a str {
        self.name
    }
    fn to_ref<'a>(&'a self) -> ThreadRef<'a> {
        ThreadRef {
            brand: self.brand,
            name: self.name,
            code: self.code,
            color: self.color,
        }
    }
    fn to_owned(&self) -> OwnedThread {
        OwnedThread {
            brand: self.brand.to_string(),
            name: self.name.to_string(),
            code: self.code.to_string(),
            color: *self.color,
        }
    }
}

pub struct OwnedThread {
    brand: String,
    name: String,
    code: String,
    color: [u8; 3],
}

impl OwnedThread {
    pub const fn new(brand: String, name: String, code: String, color: [u8; 3]) -> Self {
        Self {
            brand,
            name,
            code,
            color,
        }
    }
}

impl Thread for OwnedThread {
    fn rgb<'a>(&'a self) -> &'a [u8; 3] {
        &self.color
    }
    fn brand<'a>(&'a self) -> &'a str {
        &self.brand
    }
    fn code<'a>(&'a self) -> &'a str {
        &self.code
    }
    fn name<'a>(&'a self) -> &'a str {
        &self.name
    }
    fn to_ref<'a>(&'a self) -> ThreadRef<'a> {
        ThreadRef {
            brand: &self.brand,
            name: &self.name,
            code: &self.code,
            color: &self.color,
        }
    }
    fn to_owned(&self) -> OwnedThread {
        OwnedThread {
            brand: self.brand.clone(),
            name: self.name.clone(),
            code: self.code.clone(),
            color: self.color,
        }
    }
}
