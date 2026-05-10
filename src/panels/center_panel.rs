use gpui::*;
use gpui::prelude::FluentBuilder;
use gpui_component::dock::{Panel, PanelEvent};
use gpui_component::setting::{NumberFieldOptions, SettingGroup, SettingItem, SettingPage, SettingField};
use gpui_component::h_flex;
use gpui_component::ActiveTheme;
use gpui_component::{IconName, Theme, ThemeMode};
use gpui_component::button::{Button, ButtonVariants as _};
use rust_i18n::t;

use crate::app_menus;
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
            (t!("tab.workbench").to_string(), TAB_WORKBENCH),
            (t!("tab.config").to_string(), TAB_CONFIG),
            (t!("tab.log").to_string(), TAB_LOG),
            (t!("tab.monitor").to_string(), TAB_MONITOR),
        ];

        let show_left = AppSettings::global(cx).show_left_panel;
        let show_right = AppSettings::global(cx).show_right_panel;
        let theme = cx.theme();

        h_flex()
            .id("tab-bar")
            .h(px(40.))
            .w_full()
            .bg(theme.colors.tab_bar)
            .border_b(px(1.0))
            .border_color(theme.colors.border)
            .items_center()
            // Left: toggle left panel button
            .child(
                Button::new("toggle-left")
                    .ghost()
                    .icon(if show_left { IconName::PanelLeftClose } else { IconName::PanelLeftOpen })
                    .on_click(|_ev, _window: &mut Window, cx: &mut App| {
                        AppSettings::global_mut(cx).show_left_panel = !AppSettings::global(cx).show_left_panel;
                    })
            )
            // Center: tabs
            .children(tabs.iter().map(|(label, idx)| {
                let is_selected = selected_tab == *idx;
                let tab_idx = *idx;
                let tab_bg = theme.colors.tab_active;
                let tab_bar_bg = theme.colors.tab_bar;
                let primary = theme.colors.primary;
                let foreground = theme.colors.foreground;
                let muted_fg = theme.colors.muted_foreground;

                div()
                    .id(*idx)
                    .px(px(16.))
                    .h_full()
                    .flex()
                    .items_center()
                    .cursor_pointer()
                    .when(is_selected, |this| {
                        this.bg(tab_bg)
                            .border_b(px(2.0))
                            .border_color(primary)
                    })
                    .when(!is_selected, |this| {
                        this.border_b(px(2.0))
                            .border_color(tab_bar_bg)
                    })
                    .child(
                        div()
                            .text_color(if is_selected { foreground } else { muted_fg })
                            .text_size(px(14.))
                            .font_weight(if is_selected { FontWeight::BOLD } else { FontWeight::NORMAL })
                            .child(label.clone()),
                    )
                    .on_click(move |_ev, window: &mut Window, _cx: &mut App| {
                        if tab_idx == TAB_CONFIG {
                            AppSettings::global_mut(_cx).show_settings = true;
                        } else {
                            AppSettings::global_mut(_cx).show_settings = false;
                        }
                        window.refresh();
                    })
            }))
            // Spacer
            .child(div().flex_1())
            // Right: toggle right panel button
            .child(
                Button::new("toggle-right")
                    .ghost()
                    .icon(if show_right { IconName::PanelRightClose } else { IconName::PanelRightOpen })
                    .on_click(|_ev, _window: &mut Window, cx: &mut App| {
                        AppSettings::global_mut(cx).show_right_panel = !AppSettings::global(cx).show_right_panel;
                    })
            )
    }

    fn render_workbench_content(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .id("workbench-content")
            .flex()
            .flex_1()
            .items_center()
            .justify_center()
            .bg(theme.colors.background)
            .child(
                div()
                    .text_color(theme.colors.muted_foreground)
                    .text_size(px(24.))
                    .child(t!("tab.workbench").to_string())
            )
    }

    fn render_config_content(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let default_settings = AppSettings::default();

        let settings_page = SettingPage::new(t!("config.page.title").to_string())
            .resettable(true)
            .default_open(true)
            .groups(vec![
                SettingGroup::new()
                    .title(t!("config.group.appearance").to_string())
                    .items(vec![
                        SettingItem::new(
                            t!("config.appearance.dark_mode.label").to_string(),
                            SettingField::switch(
                                |cx: &App| cx.theme().mode.is_dark(),
                                |val: bool, cx: &mut App| {
                                    let mode = if val { ThemeMode::Dark } else { ThemeMode::Light };
                                    Theme::global_mut(cx).mode = mode;
                                    Theme::change(mode, None, cx);
                                    crate::app::themes::save_state(cx);
                                },
                            )
                            .default_value(false),
                        )
                        .description(t!("config.appearance.dark_mode.description").to_string()),
                        SettingItem::new(
                            t!("config.appearance.language.label").to_string(),
                            SettingField::dropdown(
                                vec![
                                    ("zh-CN".into(), t!("lang.zh_cn").into()),
                                    ("en".into(), t!("lang.en").into()),
                                ],
                                |cx: &App| AppSettings::global(cx).locale.clone(),
                                |val: SharedString, cx: &mut App| {
                                    AppSettings::global_mut(cx).locale = val.clone();
                                    rust_i18n::set_locale(val.as_ref());
                                    app_menus::refresh(cx);
                                    cx.refresh_windows();
                                    crate::app::themes::save_state(cx);
                                },
                            )
                            .default_value(default_settings.locale),
                        )
                        .description(t!("config.appearance.language.description").to_string()),
                    ]),
                SettingGroup::new()
                    .title(t!("config.group.font").to_string())
                    .items(vec![
                        SettingItem::new(
                            t!("config.font.label").to_string(),
                            SettingField::dropdown(
                                vec![
                                    ("Arial".into(), "Arial".into()),
                                    ("Helvetica".into(), "Helvetica".into()),
                                    ("Times New Roman".into(), "Times New Roman".into()),
                                    ("Courier New".into(), "Courier New".into()),
                                ],
                                |cx: &App| AppSettings::global(cx).font_family.clone(),
                                |val: SharedString, cx: &mut App| {
                                    AppSettings::global_mut(cx).font_family = val;
                                    crate::app::themes::save_state(cx);
                                },
                            )
                            .default_value(default_settings.font_family),
                        )
                        .description(t!("config.font.description").to_string()),
                        SettingItem::new(
                            t!("config.font.size.label").to_string(),
                            SettingField::number_input(
                                NumberFieldOptions {
                                    min: 10.0,
                                    max: 32.0,
                                    step: 1.0,
                                    ..Default::default()
                                },
                                |cx: &App| AppSettings::global(cx).font_size,
                                |val: f64, cx: &mut App| {
                                    AppSettings::global_mut(cx).font_size = val;
                                    crate::app::themes::save_state(cx);
                                },
                            )
                            .default_value(default_settings.font_size),
                        )
                        .description(t!("config.font.size.description").to_string()),
                        SettingItem::new(
                            t!("config.font.line_height.label").to_string(),
                            SettingField::number_input(
                                NumberFieldOptions {
                                    min: 8.0,
                                    max: 32.0,
                                    step: 1.0,
                                    ..Default::default()
                                },
                                |cx: &App| AppSettings::global(cx).line_height,
                                |val: f64, cx: &mut App| {
                                    AppSettings::global_mut(cx).line_height = val;
                                    crate::app::themes::save_state(cx);
                                },
                            )
                            .default_value(default_settings.line_height),
                        )
                        .description(t!("config.font.line_height.description").to_string()),
                    ]),
                SettingGroup::new()
                    .title(t!("config.group.other").to_string())
                    .items(vec![
                        SettingItem::new(
                            t!("config.auto_switch_theme.label").to_string(),
                            SettingField::checkbox(
                                |cx: &App| AppSettings::global(cx).auto_switch_theme,
                                |val: bool, cx: &mut App| {
                                    AppSettings::global_mut(cx).auto_switch_theme = val;
                                    crate::app::themes::save_state(cx);
                                },
                            )
                            .default_value(default_settings.auto_switch_theme),
                        )
                        .description(t!("config.auto_switch_theme.description").to_string()),
                    ]),
            ]);

        let theme = cx.theme();
        div()
            .id("config-content")
            .flex()
            .flex_1()
            .overflow_scroll()
            .p(px(16.))
            .bg(theme.colors.background)
            .child(gpui_component::setting::Settings::new("settings").page(settings_page))
    }

    fn render_log_content(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .id("log-content")
            .flex()
            .flex_1()
            .items_center()
            .justify_center()
            .bg(theme.colors.background)
            .child(
                div()
                    .text_color(theme.colors.muted_foreground)
                    .text_size(px(24.))
                    .child(t!("tab.log").to_string())
            )
    }

    fn render_monitor_content(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .id("monitor-content")
            .flex()
            .flex_1()
            .items_center()
            .justify_center()
            .bg(theme.colors.background)
            .child(
                div()
                    .text_color(theme.colors.muted_foreground)
                    .text_size(px(24.))
                    .child(t!("tab.monitor").to_string())
            )
    }
}

impl Panel for CenterPanel {
    fn panel_name(&self) -> &'static str {
        "CenterPanel"
    }

    fn title(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        t!("panel.workspace").to_string()
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
                    self.render_config_content(cx).into_any_element()
                } else if selected_tab == TAB_LOG {
                    self.render_log_content(cx).into_any_element()
                } else if selected_tab == TAB_MONITOR {
                    self.render_monitor_content(cx).into_any_element()
                } else {
                    self.render_workbench_content(cx).into_any_element()
                }
            )
    }
}
