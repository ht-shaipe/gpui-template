use gpui::{App, Context, Entity, IntoElement, Render, Window};
use gpui_component::{
    ActiveTheme as _, Theme, ThemeMode,
    setting::{NumberFieldOptions, SettingField, SettingGroup, SettingItem, SettingPage},
};
use rust_i18n::t;

use super::AppSettings;

/// Settings panel
pub struct SettingsPanel;

impl SettingsPanel {
    pub fn new(_window: &mut Window, _cx: &mut App) -> Self {
        Self
    }

    pub fn general_page(&self, _view: &Entity<Self>, resettable: bool) -> SettingPage {
        let default_settings = AppSettings::default();

        SettingPage::new(t!("settings.general.title").to_string())
            .resettable(resettable)
            .default_open(true)
            .groups(vec![
                SettingGroup::new()
                    .title(t!("settings.general.group.appearance").to_string())
                    .items(vec![
                        SettingItem::new(
                            t!("settings.general.appearance.dark_mode.label").to_string(),
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
                        .description(t!("settings.general.appearance.dark_mode.description").to_string()),
                        SettingItem::new(
                            t!("settings.general.appearance.auto_switch.label").to_string(),
                            SettingField::checkbox(
                                |cx: &App| AppSettings::global(cx).auto_switch_theme,
                                |val: bool, cx: &mut App| {
                                    AppSettings::global_mut(cx).auto_switch_theme = val;
                                },
                            )
                            .default_value(default_settings.auto_switch_theme),
                        )
                        .description(t!("settings.general.appearance.auto_switch.description").to_string()),
                        SettingItem::new(
                            t!("settings.general.appearance.resettable.label").to_string(),
                            SettingField::switch(
                                |cx: &App| AppSettings::global(cx).resettable,
                                |checked: bool, cx: &mut App| {
                                    AppSettings::global_mut(cx).resettable = checked
                                },
                            ),
                        )
                        .description(t!("settings.general.appearance.resettable.description").to_string()),
                    ]),
                SettingGroup::new()
                    .title(t!("settings.general.group.font").to_string())
                    .item(
                        SettingItem::new(
                            t!("settings.general.font.family.label").to_string(),
                            SettingField::dropdown(
                                vec![
                                    ("Arial".into(), "Arial".into()),
                                    ("Helvetica".into(), "Helvetica".into()),
                                    ("Times New Roman".into(), "Times New Roman".into()),
                                    ("Courier New".into(), "Courier New".into()),
                                ],
                                |cx: &App| AppSettings::global(cx).font_family.clone(),
                                |val, cx: &mut App| {
                                    AppSettings::global_mut(cx).font_family = val;
                                },
                            )
                            .default_value(default_settings.font_family),
                        )
                        .description(t!("settings.general.font.family.description").to_string()),
                    )
                    .item(
                        SettingItem::new(
                            t!("settings.general.font.size.label").to_string(),
                            SettingField::number_input(
                                NumberFieldOptions {
                                    min: 8.0,
                                    max: 72.0,
                                    ..Default::default()
                                },
                                |cx: &App| AppSettings::global(cx).font_size,
                                |val: f64, cx: &mut App| {
                                    AppSettings::global_mut(cx).font_size = val;
                                },
                            )
                            .default_value(default_settings.font_size),
                        )
                        .description(t!("settings.general.font.size.description").to_string()),
                    )
                    .item(
                        SettingItem::new(
                            t!("settings.general.font.line_height.label").to_string(),
                            SettingField::number_input(
                                NumberFieldOptions {
                                    min: 8.0,
                                    max: 32.0,
                                    ..Default::default()
                                },
                                |cx: &App| AppSettings::global(cx).line_height,
                                |val: f64, cx: &mut App| {
                                    AppSettings::global_mut(cx).line_height = val;
                                },
                            )
                            .default_value(default_settings.line_height),
                        )
                        .description(t!("settings.general.font.line_height.description").to_string()),
                    ),
            ])
    }
}

impl Render for SettingsPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let page = self.general_page(&cx.entity(), AppSettings::global(cx).resettable);
        gpui_component::setting::Settings::new("settings").page(page)
    }
}
