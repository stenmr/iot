use askama::Template;

#[derive(Template, Clone, Copy)]
#[template(path = "index.html")]
pub struct Index {
    current_temperature: f32,
}

impl Index {
    pub fn new(temp: f32) -> Self {
        Self {
            current_temperature: temp,
        }
    }
}
