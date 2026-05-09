use gpui::{App, SharedString};
#[cfg(not(target_family = "wasm"))]
use gpui::{Menu, MenuItem};
#[cfg(not(target_family = "wasm"))]
use gpui_component::{ActiveTheme as _, ThemeMode, ThemeRegistry};
#[cfg(not(target_family = "wasm"))]
use rust_i18n::t;

#[cfg(not(target_family = "wasm"))]
use crate::app::actions::{About, CloseWindow, Open, Quit, SelectLocale, SwitchTheme, SwitchThemeMode};

/// Initialize app menus
pub fn init(title: impl Into<SharedString>, cx: &mut App) {
    #[cfg(target_family = "wasm")]
    {
        let _: SharedString = title.into();
        let _ = cx;
        return;
    }

    #[cfg(not(target_family = "wasm"))]
    cx.set_menus(vec![
        Menu {
            name: title.into(),
            items: vec![
                MenuItem::action(t!("menu.app.about").to_string(), About),
                MenuItem::Separator,
                MenuItem::action(t!("menu.app.open").to_string(), Open),
                MenuItem::Separator,
                MenuItem::Submenu(Menu {
                    name: t!("menu.app.appearance").to_string().into(),
                    items: vec![
                        MenuItem::action(
                            t!("menu.app.appearance.light").to_string(),
                            SwitchThemeMode(ThemeMode::Light),
                        ),
                        MenuItem::action(
                            t!("menu.app.appearance.dark").to_string(),
                            SwitchThemeMode(ThemeMode::Dark),
                        ),
                    ],
                }),
                theme_menu(cx),
                language_menu(cx),
                MenuItem::Separator,
                MenuItem::action(t!("menu.app.quit").to_string(), Quit),
            ],
        },
        Menu {
            name: t!("menu.edit.title").to_string().into(),
            items: vec![
                MenuItem::action(t!("menu.edit.undo").to_string(), gpui_component::input::Undo),
                MenuItem::action(t!("menu.edit.redo").to_string(), gpui_component::input::Redo),
                MenuItem::separator(),
                MenuItem::action(t!("menu.edit.cut").to_string(), gpui_component::input::Cut),
                MenuItem::action(t!("menu.edit.copy").to_string(), gpui_component::input::Copy),
                MenuItem::action(t!("menu.edit.paste").to_string(), gpui_component::input::Paste),
                MenuItem::separator(),
                MenuItem::action(t!("menu.edit.select_all").to_string(), gpui_component::input::SelectAll),
            ],
        },
        Menu {
            name: t!("menu.window.title").to_string().into(),
            items: vec![
                MenuItem::action(t!("menu.window.close").to_string(), CloseWindow),
            ],
        },
        Menu {
            name: t!("menu.help.title").to_string().into(),
            items: vec![MenuItem::action(
                t!("menu.help.open_website").to_string(),
                Open,
            )],
        },
    ]);
}

#[cfg(not(target_family = "wasm"))]
fn language_menu(_cx: &App) -> MenuItem {
    MenuItem::Submenu(Menu {
        name: t!("menu.app.language").to_string().into(),
        items: vec![
            MenuItem::action(
                t!("menu.app.language.english").to_string(),
                SelectLocale("en".into()),
            ),
            MenuItem::action(
                t!("menu.app.language.zh_cn").to_string(),
                SelectLocale("zh-CN".into()),
            ),
        ],
    })
}

#[cfg(not(target_family = "wasm"))]
fn theme_menu(cx: &App) -> MenuItem {
    let themes = ThemeRegistry::global(cx).sorted_themes();
    let current_theme = cx.theme().theme_name();
    MenuItem::Submenu(Menu {
        name: t!("menu.app.theme").to_string().into(),
        items: themes
            .iter()
            .map(|theme| {
                let name = theme.name.clone();
                let label = if name.as_ref() == current_theme.as_ref() {
                    format!("✓ {}", name)
                } else {
                    name.to_string()
                };
                MenuItem::action(label, SwitchTheme(name))
            })
            .collect(),
    })
}