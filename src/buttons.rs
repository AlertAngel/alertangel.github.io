#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrowKeys {
    RightArrow,
    LeftArrow,
    TopArrow,
    BottomArrow,
}

impl ArrowKeys {
    pub fn from_key_string(key: &str) -> Option<Self> {
        match key {
            "ArrowUp" => Some(ArrowKeys::TopArrow),
            "ArrowDown" => Some(ArrowKeys::BottomArrow),
            "ArrowLeft" => Some(ArrowKeys::LeftArrow),
            "ArrowRight" => Some(ArrowKeys::RightArrow),
            _ => None,
        }
    }

    pub fn as_symbol(&self) -> &str {
        match self {
            ArrowKeys::TopArrow => "↑",
            ArrowKeys::BottomArrow => "↓",
            ArrowKeys::LeftArrow => "←",
            ArrowKeys::RightArrow => "→",
        }
    }

    pub fn as_name(&self) -> &str {
        match self {
            ArrowKeys::TopArrow => "Up",
            ArrowKeys::BottomArrow => "Down",
            ArrowKeys::LeftArrow => "Left",
            ArrowKeys::RightArrow => "Right",
        }
    }
}
