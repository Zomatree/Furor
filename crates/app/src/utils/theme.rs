use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename="kebab")]
pub struct Theme {
    pub accent: String,
    pub background: String,
    pub foreground: String,
    pub primary_background: String,
    pub primary_header: String,
    pub secondary_background: String,
    pub secondary_foreground: String,
    pub secondary_header: String,
    pub tertiary_background: String,
    pub tertiary_foreground: String,
    pub block: String,
    pub message_box: String,
    pub mention: String,
    pub scrollbar_thumb: String,
    pub scrollbar_track: String,
    pub status_online: String,
    pub status_away: String,
    pub status_busy: String,
    pub status_streaming: String,
    pub status_invisible: String,
    pub success: String,
    pub warning: String,
    pub error: String,
    pub hover: String,

    pub light: bool,
    pub border_color: String,
    pub sidebar_active: String,
    pub css: String,
    pub font: String,

    #[serde(rename="monospaceFont")]
    pub monospace_font: String
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            accent: "#FD6671".to_string(),
            background: "#191919".to_string(),
            foreground: "#F6F6F6".to_string(),
            primary_background: "#242424".to_string(),
            primary_header: "#363636".to_string(),
            secondary_background: "#1E1E1E".to_string(),
            secondary_foreground: "#C8C8C8".to_string(),
            secondary_header: "#2D2D2D".to_string(),
            tertiary_background: "#4D4D4D".to_string(),
            tertiary_foreground: "#848484".to_string(),
            block: "#2D2D2D".to_string(),
            message_box: "#363636".to_string(),
            mention: "#rgba(251, 255, 0, 0.06)".to_string(),
            scrollbar_thumb: "#CA525A".to_string(),
            scrollbar_track: "transparent".to_string(),
            status_online: "#3ABF7E".to_string(),
            status_away: "#F39F00".to_string(),
            status_busy: "#F84848".to_string(),
            status_streaming: "#977EFF".to_string(),
            status_invisible: "#A5A5A5".to_string(),
            success: "#65E572".to_string(),
            warning: "#FAA352".to_string(),
            error: "#ED4245".to_string(),
            hover: "#rgba(0, 0, 0, 0.1)".to_string(),

            light: false,
            border_color: "#20252C".to_string(),
            sidebar_active: "#2f3136".to_string(),
            css: "".to_string(),
            font: "Inter".to_string(),

            monospace_font: "Fira Code".to_string()
        }
    }

    pub fn load_custom(json: &str) -> Option<Self> {
        serde_json::from_str(json).ok()
    }
}
