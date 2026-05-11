use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::button::{Button, ButtonVariants as _};
use gpui_component::dock::{Panel, PanelEvent};
use gpui_component::h_flex;
use gpui_component::label::Label;
use gpui_component::scroll::ScrollableElement;
use gpui_component::setting::{
    NumberFieldOptions, SettingField, SettingGroup, SettingItem, SettingPage,
};
use gpui_component::v_flex;
use gpui_component::ActiveTheme;
use gpui_component::{Icon, IconName, Sizable, Theme, ThemeMode};
use rust_i18n::t;

use crate::app_menus;
use crate::app_state::AppSettings;
use crate::core::updater::{UpdateCheckResult, UpdateManager, Version};

const TAB_WORKBENCH: usize = 0;
const TAB_CONFIG: usize = 1;
const TAB_LOG: usize = 2;
const TAB_MONITOR: usize = 3;

/// Update check status
#[derive(Debug, Clone, Default)]
pub enum UpdateStatus {
    #[default]
    Idle,
    Checking,
    NoUpdate,
    Available {
        version: String,
        notes: String,
    },
    Error(String),
}

pub struct CenterPanel {
    focus_handle: FocusHandle,
    update_manager: UpdateManager,
    update_status: UpdateStatus,
    open_tabs: Vec<usize>,  // Track which tabs are open
}

impl CenterPanel {
    pub fn new(_window: &mut Window, cx: &mut App) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            update_manager: UpdateManager::default(),
            update_status: UpdateStatus::default(),
            open_tabs: vec![TAB_WORKBENCH, TAB_CONFIG, TAB_LOG, TAB_MONITOR],  // All tabs open by default
        }
    }

    /// Close a tab
    fn close_tab(&mut self, tab_idx: usize, cx: &mut Context<Self>) {
        // Don't close if it's the last tab
        if self.open_tabs.len() <= 1 {
            return;
        }
        // Remove the tab
        self.open_tabs.retain(|&t| t != tab_idx);
        // If the closed tab was selected, switch to first available tab
        let settings = AppSettings::global(cx);
        let is_selected = if tab_idx == TAB_CONFIG {
            settings.show_settings
        } else {
            false
        };
        // Check if current tab is still open
        let current_tab_open = self.open_tabs.contains(&tab_idx);
        if is_selected || !current_tab_open {
            // Switch to workbench (always available)
            AppSettings::global_mut(cx).show_settings = false;
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
        let show_left = AppSettings::global(cx).show_left_panel;
        let show_right = AppSettings::global(cx).show_right_panel;
        let show_bottom = AppSettings::global(cx).show_bottom_panel;
        let theme = cx.theme();
        let open_tabs = &self.open_tabs;
        let entity = cx.entity().clone();

        // Tab definitions with icons
        let tab_defs = [
            (TAB_WORKBENCH, t!("tab.workbench").to_string(), IconName::LayoutDashboard),
            (TAB_CONFIG, t!("tab.config").to_string(), IconName::Settings),
            (TAB_LOG, t!("tab.log").to_string(), IconName::SquareTerminal),
            (TAB_MONITOR, t!("tab.monitor").to_string(), IconName::ChartPie),
        ];

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
                    .icon(if show_left {
                        IconName::PanelLeftClose
                    } else {
                        IconName::PanelLeftOpen
                    })
                    .on_click(|_ev, _window: &mut Window, cx: &mut App| {
                        AppSettings::global_mut(cx).show_left_panel =
                            !AppSettings::global(cx).show_left_panel;
                    }),
            )
            // Center: tabs
            .children(tab_defs.iter().filter_map(|(idx, label, icon)| {
                // Only show tabs that are open
                if !open_tabs.contains(idx) {
                    return None;
                }
                let is_selected = selected_tab == *idx;
                let tab_idx = *idx;
                let tab_label = label.clone();
                let tab_bg = theme.colors.tab_active;
                let tab_bar_bg = theme.colors.tab_bar;
                let primary = theme.colors.primary;
                let foreground = theme.colors.foreground;
                let muted_fg = theme.colors.muted_foreground;
                let entity_clone = entity.clone();

                Some(
                    div()
                        .id(*idx)
                        .px(px(12.))
                        .h_full()
                        .flex()
                        .items_center()
                        .gap(px(6.))
                        .cursor_pointer()
                        .when(is_selected, |this| {
                            this.bg(tab_bg).border_b(px(2.0)).border_color(primary)
                        })
                        .when(!is_selected, |this| {
                            this.border_b(px(2.0)).border_color(tab_bar_bg)
                        })
                        .child(
                            Icon::new(icon.clone())
                                .text_size(px(14.))
                                .text_color(if is_selected { foreground } else { muted_fg }),
                        )
                        .child(
                            div()
                                .text_color(if is_selected { foreground } else { muted_fg })
                                .text_size(px(13.))
                                .font_weight(if is_selected {
                                    FontWeight::BOLD
                                } else {
                                    FontWeight::NORMAL
                                })
                                .child(tab_label),
                        )
                        // Close button (only show if more than one tab)
                        .when(self.open_tabs.len() > 1, |this| {
                            let tab_to_close = *idx;
                            let entity = entity_clone.clone();
                            this.child(
                                Button::new(("close-tab", *idx))
                                    .ghost()
                                    .icon(IconName::Close)
                                    .on_click(move |_ev, window: &mut Window, cx: &mut App| {
                                        entity.update(cx, |this, cx| {
                                            this.close_tab(tab_to_close, cx);
                                        });
                                        window.refresh();
                                    })
                            )
                        })
                        .on_click(move |_ev, window: &mut Window, _cx: &mut App| {
                            if tab_idx == TAB_CONFIG {
                                AppSettings::global_mut(_cx).show_settings = true;
                            } else {
                                AppSettings::global_mut(_cx).show_settings = false;
                            }
                            window.refresh();
                        }),
                )
            }))
            // Spacer
            .child(div().flex_1())
            // Right: toggle right panel button
            .child(
                Button::new("toggle-right")
                    .ghost()
                    .icon(if show_right {
                        IconName::PanelRightClose
                    } else {
                        IconName::PanelRightOpen
                    })
                    .on_click(|_ev, _window: &mut Window, cx: &mut App| {
                        AppSettings::global_mut(cx).show_right_panel =
                            !AppSettings::global(cx).show_right_panel;
                    }),
            )
            // Bottom panel toggle button
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
                    }),
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
                    .child(t!("tab.workbench").to_string()),
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
                                    let mode = if val {
                                        ThemeMode::Dark
                                    } else {
                                        ThemeMode::Light
                                    };
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
                    .items(vec![SettingItem::new(
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
                    .description(
                        t!("config.auto_switch_theme.description").to_string(),
                    )]),
                // Software Update
                SettingGroup::new()
                    .title(t!("config.group.update").to_string())
                    .items(vec![
                        // Current version & update status
                        SettingItem::render({
                            let current_version = Version::current().to_string();
                            let update_status = self.update_status.clone();
                            move |_options, _window, cx| {
                                v_flex()
                                    .gap_2()
                                    .w_full()
                                    .child(
                                        h_flex()
                                            .gap_2()
                                            .items_center()
                                            .child(
                                                Label::new(
                                                    t!("config.update.current_version.label")
                                                        .to_string(),
                                                )
                                                .text_sm(),
                                            )
                                            .child(
                                                Label::new(&current_version)
                                                    .text_sm()
                                                    .text_color(cx.theme().colors.muted_foreground),
                                            ),
                                    )
                                    .child(match &update_status {
                                        UpdateStatus::Idle | UpdateStatus::NoUpdate => h_flex()
                                            .gap_2()
                                            .items_center()
                                            .child(Icon::new(IconName::Check))
                                            .child(
                                                Label::new(
                                                    t!("config.update.status.up_to_date")
                                                        .to_string(),
                                                )
                                                .text_xs()
                                                .text_color(cx.theme().colors.success_foreground),
                                            ),
                                        UpdateStatus::Checking => h_flex()
                                            .gap_2()
                                            .items_center()
                                            .child(Icon::new(IconName::LoaderCircle))
                                            .child(
                                                Label::new(
                                                    t!("config.update.status.checking")
                                                        .to_string(),
                                                )
                                                .text_xs()
                                                .text_color(cx.theme().colors.muted_foreground),
                                            ),
                                        UpdateStatus::Available { version, notes } => {
                                            let notes_elem = if notes.is_empty() {
                                                None
                                            } else {
                                                Some(
                                                    Label::new(notes)
                                                        .text_xs()
                                                        .text_color(cx.theme().colors.muted_foreground),
                                                )
                                            };
                                            v_flex()
                                                .gap_2()
                                                .w_full()
                                                .child(
                                                    h_flex()
                                                        .gap_2()
                                                        .items_center()
                                                        .child(Icon::new(IconName::ArrowDown))
                                                        .child(
                                                            Label::new(
                                                                t!(
                                                                    "config.update.status.available",
                                                                    version = version
                                                                )
                                                                .to_string(),
                                                            )
                                                            .text_xs()
                                                            .text_color(cx.theme().colors.accent_foreground),
                                                        ),
                                                )
                                                .children(notes_elem)
                                        }
                                        UpdateStatus::Error(err) => div()
                                            .mt_2()
                                            .p(px(8.))
                                            .rounded(px(6.))
                                            .bg(cx.theme().colors.danger.alpha(0.1))
                                            .border(px(1.))
                                            .border_color(cx.theme().colors.danger.alpha(0.3))
                                            .child(
                                                h_flex()
                                                    .gap_2()
                                                    .items_center()
                                                    .child(Icon::new(IconName::CircleX).text_color(cx.theme().colors.danger))
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .text_color(cx.theme().colors.danger)
                                                            .child(err.clone())
                                                    )
                                            ),
                                    })
                            }
                        }),
                        // Check for updates button
                        SettingItem::new(
                            t!("config.update.check.label").to_string(),
                            SettingField::render({
                                let view = cx.entity().clone();
                                move |options, _window, _cx| {
                                    Button::new("check-updates")
                                        .icon(IconName::LoaderCircle)
                                        .label(t!("config.update.check.button").to_string())
                                        .outline()
                                        .with_size(options.size)
                                        .on_click({
                                            let view = view.clone();
                                            move |_, window, cx| {
                                                view.update(cx, |this, cx| {
                                                    this.check_for_updates(window, cx);
                                                });
                                            }
                                        })
                                }
                            }),
                        )
                        .description(t!("config.update.check.description").to_string()),
                        // Auto check on startup
                        SettingItem::new(
                            t!("config.update.auto_check.label").to_string(),
                            SettingField::switch(
                                |cx: &App| AppSettings::global(cx).auto_check_on_startup,
                                |val: bool, cx: &mut App| {
                                    AppSettings::global_mut(cx).auto_check_on_startup = val;
                                    crate::app::themes::save_state(cx);
                                },
                            )
                            .default_value(default_settings.auto_check_on_startup),
                        )
                        .description(t!("config.update.auto_check.description").to_string()),
                        // Notifications
                        SettingItem::new(
                            t!("config.update.notifications.label").to_string(),
                            SettingField::switch(
                                |cx: &App| AppSettings::global(cx).notifications_enabled,
                                |val: bool, cx: &mut App| {
                                    AppSettings::global_mut(cx).notifications_enabled = val;
                                    crate::app::themes::save_state(cx);
                                },
                            )
                            .default_value(default_settings.notifications_enabled),
                        )
                        .description(t!("config.update.notifications.description").to_string()),
                        // Auto update
                        SettingItem::new(
                            t!("config.update.auto_update.label").to_string(),
                            SettingField::switch(
                                |cx: &App| AppSettings::global(cx).auto_update,
                                |val: bool, cx: &mut App| {
                                    AppSettings::global_mut(cx).auto_update = val;
                                    crate::app::themes::save_state(cx);
                                },
                            )
                            .default_value(default_settings.auto_update),
                        )
                        .description(t!("config.update.auto_update.description").to_string()),
                        // Check frequency
                        SettingItem::new(
                            t!("config.update.frequency.label").to_string(),
                            SettingField::number_input(
                                NumberFieldOptions {
                                    min: 1.0,
                                    max: 30.0,
                                    step: 1.0,
                                    ..Default::default()
                                },
                                |cx: &App| AppSettings::global(cx).check_frequency_days,
                                |val: f64, cx: &mut App| {
                                    AppSettings::global_mut(cx).check_frequency_days = val;
                                    crate::app::themes::save_state(cx);
                                },
                            )
                            .default_value(default_settings.check_frequency_days),
                        )
                        .description(t!("config.update.frequency.description").to_string()),
                    ]),
            ]);

        let theme = cx.theme();
        div()
            .id("config-content")
            .flex()
            .flex_1()
            .overflow_scroll()
            // .p(px(16.))
            .bg(theme.colors.background)
            .child(gpui_component::setting::Settings::new("settings").page(settings_page))
    }

    /// Check for software updates
    pub fn check_for_updates(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.update_status = UpdateStatus::Checking;
        cx.notify();

        let update_manager = self.update_manager.clone();
        let entity = cx.entity().downgrade();

        cx.spawn(async move |_this, cx| {
            let result = update_manager.check_for_updates().await;

            let _ = cx.update(|cx| {
                let _ = entity.update(cx, |this, cx| {
                    this.update_status = match result {
                        UpdateCheckResult::NoUpdate => UpdateStatus::NoUpdate,
                        UpdateCheckResult::UpdateAvailable(info) => UpdateStatus::Available {
                            version: info.version,
                            notes: info.release_notes,
                        },
                        UpdateCheckResult::Error(err) => UpdateStatus::Error(err),
                    };cx.notify();
                });
            });
        })
        .detach();
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
                    .child(t!("tab.log").to_string()),
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
                    .child(t!("tab.monitor").to_string()),
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
            .child(if selected_tab == TAB_CONFIG {
                self.render_config_content(cx).into_any_element()
            } else if selected_tab == TAB_LOG {
                self.render_log_content(cx).into_any_element()
            } else if selected_tab == TAB_MONITOR {
                self.render_monitor_content(cx).into_any_element()
            } else {
                self.render_workbench_content(cx).into_any_element()
            })
    }
}
