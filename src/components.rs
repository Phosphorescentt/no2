// use ratatui::style::Color;
#[derive(Clone, PartialEq, Eq)]
pub struct Button {
    pub id: String,
    pub text: String,
    // pub fg_color: Option<Color>,
    // pub bg_color: Option<Color>,
}

impl Button {
    pub fn new(id: String, text: String) -> Self {
        Button { id, text }
    }
}
