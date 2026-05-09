use gpui::{actions, Action, SharedString};
use gpui_component::ThemeMode;
use serde::Deserialize;

// General application actions
actions!(
    app,
    [
        About,
        Open,
        Quit,
        CloseWindow,
        ToggleSearch,
        OpenSettings,
    ]
);

// Menu editing actions
actions!(menu, [Copy, Paste, Cut,]);

/// Select language
#[derive(Action, Clone, PartialEq, Eq, Deserialize)]
#[action(namespace = app, no_json)]
pub struct SelectLocale(pub SharedString);

/// Select font size
#[derive(Action, Clone, PartialEq, Eq, Deserialize)]
#[action(namespace = app, no_json)]
pub struct SelectFont(pub usize);

/// Select border radius
#[derive(Action, Clone, PartialEq, Eq, Deserialize)]
#[action(namespace = app, no_json)]
pub struct SelectRadius(pub usize);

/// Switch theme
#[derive(Action, Clone, PartialEq)]
#[action(namespace = themes, no_json)]
pub struct SwitchTheme(pub SharedString);

/// Switch theme mode (light/dark)
#[derive(Action, Clone, PartialEq)]
#[action(namespace = themes, no_json)]
pub struct SwitchThemeMode(pub ThemeMode);
