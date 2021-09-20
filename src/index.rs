use askama::Template;

#[derive(Template, Clone, Copy)]
#[template(path = "index.html")]
pub struct Index {
    value: u32,
}

impl Index {
    pub fn new(value: u32) -> Self {
        Self {
            value,
        }
    }
}
