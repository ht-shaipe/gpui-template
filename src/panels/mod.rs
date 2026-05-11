mod bottom_panel;
mod center_panel;
mod left_panel;
mod right_panel;
mod settings;

pub use crate::app_state::AppSettings;
pub use bottom_panel::BottomPanel;
pub use center_panel::CenterPanel;
pub use left_panel::LeftPanel;
pub use right_panel::RightPanel;
pub use settings::SettingsPanel;

use gpui::*;
use gpui_component::dock::{Panel, PanelControl, PanelEvent};
use gpui_component::ActiveTheme as _;
use serde::{Deserialize, Serialize};

/// A minimal sample panel kept for reference and legacy panel registration
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
        div().h(px(0.)).into_any_element()
    }

    fn zoomable(&self, _cx: &App) -> Option<PanelControl> {
        None
    }
}

impl EventEmitter<PanelEvent> for SamplePanel {}

impl Focusable for SamplePanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for SamplePanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .id("sample-panel")
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(theme.colors.background)
            .p(px(12.))
            .child(
                div()
                    .text_color(theme.colors.muted_foreground)
                    .text_size(px(13.))
                    .child(self.name.clone())
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
