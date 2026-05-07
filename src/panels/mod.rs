mod settings;

pub use settings::AppSettings;
pub use settings::SettingsPanel;

use gpui::*;
use gpui_component::dock::{Panel, PanelEvent, PanelInfo};
use serde::{Deserialize, Serialize};

/// A minimal sample panel to demonstrate dock panel integration
pub struct SamplePanel {
    focus_handle: FocusHandle,
}

impl SamplePanel {
    pub fn new(_window: &mut Window, cx: &mut App) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Panel for SamplePanel {
    fn persistent_name() -> &'static str {
        "SamplePanel"
    }

    fn panel_info(&self) -> PanelInfo {
        PanelInfo::default()
    }

    fn title(&self, _window: &Window, _cx: &App) -> AnyElement {
        "Sample Panel".into_any_element()
    }

    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for SamplePanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        use gpui_component::ActiveTheme;

        div()
            .size_full()
            .p_4()
            .child(
                div()
                    .text_sm()
                    .text_color(cx.theme().foreground)
                    .child("Welcome to GPUI Template! This is a sample panel."),
            )
    }
}

/// Dock panel state for persistence
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DockPanelState {
    #[serde(default)]
    pub panel_name: String,
}

impl DockPanelState {
    pub fn from_value(value: serde_json::Value) -> Self {
        serde_json::from_value(value).unwrap_or_default()
    }
}
