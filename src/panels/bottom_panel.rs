//! Bottom panel - Multi-tab terminal

use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::scroll::ScrollableElement;
use gpui_component::dock::{Panel, PanelControl};
use gpui_component::ActiveTheme as _;
use gpui_component::IconName;
use crate::app_state::AppSettings;

pub struct BottomPanel {
    focus_handle: FocusHandle,
}

impl BottomPanel {
    pub fn new(_window: &mut Window, cx: &mut App) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Panel for BottomPanel {
    fn panel_name(&self) -> &'static str {
        "BottomPanel"
    }

    fn title(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().h_px().into_any_element()
    }

    fn zoomable(&self, _cx: &App) -> Option<PanelControl> {
        None
    }
}

impl EventEmitter<gpui_component::dock::PanelEvent> for BottomPanel {}

impl Focusable for BottomPanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for BottomPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        
        div()
            .id("bottom-panel")
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(theme.colors.background)
            .child(self.render_tab_bar(window, cx))
            .child(self.render_terminal_content(cx))
    }
}

impl BottomPanel {
    fn render_tab_bar(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let settings = AppSettings::global(cx);
        let tabs: Vec<_> = settings.get_all_terminal_tabs();
        let active_id = settings.active_terminal_tab_id;
        let tab_count = tabs.len();
        
        let show_bottom = settings.show_bottom_panel;
        
        div()
            .flex()
            .flex_row()
            .h(px(32.))
            .bg(theme.colors.tab_bar)
            .border_b(px(1.))
            .border_color(theme.colors.border)
            // Left: toggle bottom panel button
            .child(
                Button::new("toggle-bottom")
                    .ghost()
                    .icon(if show_bottom {
                        IconName::ChevronDown
                    } else {
                        IconName::ChevronUp
                    })
                    .on_click(|_ev, _window: &mut Window, cx: &mut App| {
                        AppSettings::global_mut(cx).show_bottom_panel =
                            !AppSettings::global(cx).show_bottom_panel;
                    })
            )
            .children(tabs.iter().map(|tab| {
                let is_active = tab.id == active_id;
                let tab_id = tab.id;
                let tab_bg = theme.colors.tab_active;
                let tab_bar_bg = theme.colors.tab_bar;
                let primary = theme.colors.primary;
                let foreground = theme.colors.foreground;
                let muted_fg = theme.colors.muted_foreground;
                
                // Use a clickable div wrapper with child content
                let tab_btn_id = SharedString::from(format!("tab-{}", tab_id));
                
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .px(px(12.))
                    .py(px(4.))
                    .cursor_pointer()
                    .when(is_active, |this| {
                        this.bg(tab_bg).border_b(px(2.0)).border_color(primary)
                    })
                    .when(!is_active, |this| {
                        this.border_b(px(2.0)).border_color(tab_bar_bg)
                    })
                    .child(
                        Button::new(tab_btn_id)
                            .ghost()
                            .on_click(move |_ev, _window: &mut Window, cx: &mut App| {
                                AppSettings::global_mut(cx).set_active_terminal_tab(tab_id);
                            })
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .text_color(if is_active { foreground } else { muted_fg })
                                    .text_size(px(12.))
                                    .child(tab.title.clone())
                            )
                    )
                    .when(tab_count > 1, |el| {
                        let close_btn_id = SharedString::from(format!("close-{}", tab_id));
                        el.child(
                            Button::new(close_btn_id)
                                .ghost()
                                .on_click(move |_ev, _window: &mut Window, cx: &mut App| {
                                    AppSettings::global_mut(cx).close_terminal_tab(tab_id);
                                })
                                .child(
                                    div()
                                        .ml(px(6.))
                                        .text_color(muted_fg)
                                        .text_size(px(10.))
                                        .child("×")
                                )
                        )
                    })
            }))
            .child(
                Button::new("add-tab")
                    .ghost()
                    .on_click(|_ev, _window: &mut Window, cx: &mut App| {
                        AppSettings::global_mut(cx).add_terminal_tab();
                    })
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .w(px(32.))
                            .h(px(32.))
                            .text_color(theme.colors.muted_foreground)
                            .text_size(px(16.))
                            .child("+")
                    )
            )
    }
    
    fn render_terminal_content(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let _theme = cx.theme();
        let settings = AppSettings::global(cx);
        let Some(tab) = settings.get_active_terminal_tab() else {
            return div().flex_1().into_any_element();
        };
        
        let output_text = tab.output.join("\n");
        
        div()
            .flex()
            .flex_col()
            .flex_1()
            .p(px(8.))
            .child(
                div()
                    .flex_1()
                    .bg(rgb(0x1E1E1E))
                    .rounded(px(4.))
                    .p(px(8.))
                    .font_family("Menlo, monospace")
                    .text_size(px(12.))
                    .text_color(rgb(0xD4D4D4))
                    .overflow_scrollbar()
                    .child(
                        div()
                            .id("terminal-output")
                            .child(output_text)
                    )
            )
            .child(div().h(px(8.)))
            .into_any_element()
    }
}
