use gpui::*;
use gpui::prelude::FluentBuilder;
use gpui_component::dock::{Panel, PanelEvent};
use gpui_component::setting::{SettingGroup, SettingItem, SettingPage, SettingField};
use gpui_component::h_flex;
use gpui_component::ActiveTheme;

use crate::app_state::AppSettings;

const TAB_WORKBENCH: usize = 0;
const TAB_CONFIG: usize = 1;
const TAB_LOG: usize = 2;
const TAB_MONITOR: usize = 3;

pub struct CenterPanel {
    focus_handle: FocusHandle,
}

impl CenterPanel {
    pub fn new(_window: &mut Window, cx: &mut App) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }

    fn get_selected_tab(&self, cx: &App) -> usize {
        // Check if settings should be shown
        if AppSettings::global(cx).show_settings {
            TAB_CONFIG
        } else {
            TAB_WORKBENCH
        }
    }

    fn render_tab_bar(&self, selected_tab: usize, cx: &mut Context<Self>) -> impl IntoElement {
        let tabs = [
            ("工作台", TAB_WORKBENCH),
            ("配置", TAB_CONFIG),
            ("日志", TAB_LOG),
            ("监控", TAB_MONITOR),
        ];

        h_flex()
            .id("tab-bar")
            .h(px(40.))
            .w_full()
            .bg(rgb(0x2a2a2a))
            .border_b(px(1.0))
            .border_color(rgb(0x3a3a3a))
            .items_center()
            .children(tabs.iter().map(|(label, idx)| {
                let is_selected = selected_tab == *idx;
                let tab_idx = *idx;
                
                div()
                    .id(*label)
                    .px(px(16.))
                    .h_full()
                    .flex()
                    .items_center()
                    .cursor_pointer()
                    .when(is_selected, |this| {
                        this.bg(rgb(0x1a1a1a))
                            .border_b(px(2.0))
                            .border_color(rgb(0x4a9eff))
                    })
                    .when(!is_selected, |this| {
                        this.border_b(px(2.0))
                            .border_color(rgb(0x2a2a2a))
                    })
                    .child(
                        div()
                            .text_color(if is_selected { rgb(0xffffff) } else { rgb(0x888888) })
                            .text_size(px(14.))
                            .font_weight(if is_selected { FontWeight::BOLD } else { FontWeight::NORMAL })
                            .child(*label),
                    )
                    .on_click(move |_ev, window: &mut Window, _cx: &mut App| {
                        // Update global settings to show the selected tab
                        if tab_idx == TAB_CONFIG {
                            AppSettings::global_mut(_cx).show_settings = true;
                        } else {
                            AppSettings::global_mut(_cx).show_settings = false;
                        }
                        window.refresh();
                    })
            }))
            .child(
                div()
                    .id("tab-add")
                    .ml_auto()
                    .px(px(12.))
                    .h_full()
                    .flex()
                    .items_center()
                    .cursor_pointer()
                    .child(
                        div()
                            .text_color(rgb(0x888888))
                            .text_size(px(18.))
                            .child("+"),
                    )
            )
    }

    fn render_workbench_content(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("workbench-content")
            .flex()
            .flex_1()
            .items_center()
            .justify_center()
            .bg(rgb(0x1a1a1a))
            .child(
                div()
                    .text_color(rgb(0x666666))
                    .text_size(px(24.))
                    .child("工作台")
            )
    }

    fn render_config_content(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let default_settings = AppSettings::default();
        
        // Create a simple settings page
        let settings_page = SettingPage::new("系统配置".to_string())
            .resettable(true)
            .default_open(true)
            .groups(vec![
                SettingGroup::new()
                    .title("外观".to_string())
                    .items(vec![
                        SettingItem::new(
                            "深色模式".to_string(),
                            SettingField::switch(
                                |cx: &App| cx.theme().mode.is_dark(),
                                |val: bool, cx: &mut App| {
                                    log::info!("Dark mode toggled: {}", val);
                                },
                            )
                            .default_value(false),
                        )
                        .description("启用深色主题".to_string()),
                        SettingItem::new(
                            "语言".to_string(),
                            SettingField::dropdown(
                                vec![
                                    ("zh-CN".into(), "简体中文".into()),
                                    ("en".into(), "English".into()),
                                ],
                                |cx: &App| "zh-CN".into(),
                                |val: SharedString, cx: &mut App| {
                                    log::info!("Language changed to: {}", val);
                                    rust_i18n::set_locale(val.as_ref());
                                },
                            )
                            .default_value("zh-CN"),
                        )
                        .description("选择界面语言".to_string()),
                    ]),
                SettingGroup::new()
                    .title("其他".to_string())
                    .items(vec![
                        SettingItem::new(
                            "自动保存".to_string(),
                            SettingField::checkbox(
                                |cx: &App| AppSettings::global(cx).auto_switch_theme,
                                |val: bool, cx: &mut App| {
                                    log::info!("Auto save toggled: {}", val);
                                },
                            )
                            .default_value(default_settings.auto_switch_theme),
                        )
                        .description("自动保存设置".to_string()),
                    ]),
            ]);

        div()
            .id("config-content")
            .flex()
            .flex_1()
            .overflow_scroll()
            .p(px(16.))
            .bg(rgb(0x1a1a1a))
            .child(gpui_component::setting::Settings::new("settings").page(settings_page))
    }

    fn render_log_content(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("log-content")
            .flex()
            .flex_1()
            .items_center()
            .justify_center()
            .bg(rgb(0x1a1a1a))
            .child(
                div()
                    .text_color(rgb(0x666666))
                    .text_size(px(24.))
                    .child("日志")
            )
    }

    fn render_monitor_content(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("monitor-content")
            .flex()
            .flex_1()
            .items_center()
            .justify_center()
            .bg(rgb(0x1a1a1a))
            .child(
                div()
                    .text_color(rgb(0x666666))
                    .text_size(px(24.))
                    .child("监控")
            )
    }
}

impl Panel for CenterPanel {
    fn panel_name(&self) -> &'static str {
        "CenterPanel"
    }

    fn title(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        "工作区"
    }
}

impl EventEmitter<PanelEvent> for CenterPanel {}

impl Focusable for CenterPanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for CenterPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected_tab = self.get_selected_tab(cx);
        
        div()
            .id("center-panel")
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .child(self.render_tab_bar(selected_tab, cx))
            .child(
                if selected_tab == TAB_CONFIG {
                    div().child(self.render_config_content(cx))
                } else if selected_tab == TAB_LOG {
                    div().child(self.render_log_content(cx))
                } else if selected_tab == TAB_MONITOR {
                    div().child(self.render_monitor_content(cx))
                } else {
                    div().child(self.render_workbench_content(cx))
                }
            )
    }
}
