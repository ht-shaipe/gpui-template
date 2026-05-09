mod settings;

pub use crate::app_state::AppSettings;
pub use settings::SettingsPanel;

use gpui::*;
use gpui_component::dock::{Panel, PanelEvent};
use serde::{Deserialize, Serialize};

/// A minimal sample panel to demonstrate dock panel integration
pub struct SamplePanel {
    focus_handle: FocusHandle,
    name: SharedString,
}

impl SamplePanel {
    pub fn new(_window: &mut Window, cx: &mut App) -> Self {
        Self::with_name("Sample Panel", cx)
    }

    pub fn with_name(name: impl Into<SharedString>, cx: &mut App) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            name: name.into(),
        }
    }
}

impl Panel for SamplePanel {
    fn panel_name(&self) -> &'static str {
        "SamplePanel"
    }

    fn title(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.name.clone()
    }
}

impl EventEmitter<PanelEvent> for SamplePanel {}

impl Focusable for SamplePanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for SamplePanel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        eprintln!("DEBUG SamplePanel render: name={}", self.name);
        
        let text_elem = div()
            .id("text-container")
            .flex()
            .items_center()
            .justify_center()
            .w_full()
            .h_full()
            .bg(gpui::rgb(0x444444))
            .text_color(gpui::red())
            .text_size(px(32.))
            .child("TEST TEXT");
        
        eprintln!("DEBUG About to return from SamplePanel render");
        text_elem
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
