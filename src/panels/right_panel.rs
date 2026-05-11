//! Right panel - Outline / Properties panel

use gpui::*;
use gpui_component::dock::{Panel, PanelControl};
use gpui_component::ActiveTheme as _;

/// Right panel for outline view
pub struct RightPanel {
    focus_handle: FocusHandle,
}

impl RightPanel {
    pub fn new(_window: &mut Window, cx: &mut App) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Panel for RightPanel {
    fn panel_name(&self) -> &'static str {
        "RightPanel"
    }

    fn title(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // Hide title bar
        div().h_px().into_any_element()
    }

    fn zoomable(&self, _cx: &App) -> Option<PanelControl> {
        None
    }
}

impl EventEmitter<gpui_component::dock::PanelEvent> for RightPanel {}

impl Focusable for RightPanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for RightPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        
        div()
            .id("right-panel")
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(theme.colors.background)
            .p(px(12.))
            .child(
                div()
                    .text_color(theme.colors.foreground)
                    .text_size(px(14.))
                    .font_weight(FontWeight::BOLD)
                    .child("Outline")
            )
            .child(div().h(px(8.)))
            .child(
                div()
                    .text_color(theme.colors.muted_foreground)
                    .text_size(px(13.))
                    .child("No symbols found")
            )
    }
}
