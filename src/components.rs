use ratatui::style::Color;

pub struct Button {
    pub text: String,
    pub fg_color: Option<Color>,
    pub bg_color: Option<Color>,
}

impl From<String> for Button {
    fn from(text: String) -> Self {
        Button {
            text,
            fg_color: None,
            bg_color: None,
        }
    }
}
