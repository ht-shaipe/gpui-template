use gpui::{
    div, prelude::FluentBuilder, App, AppContext, Context, Entity, FocusHandle,
    InteractiveElement as _, IntoElement, MouseButton, ParentElement as _, Render,
    SharedString, Styled as _, Window, px,
};
use gpui_component::{
    IconName, Sizable as _, Side,
    button::{Button, ButtonVariants as _},
    menu::{AppMenuBar, DropdownMenu as _},
    TitleBar,
};
use rust_i18n::t;

use crate::app_state::AppState;
use crate::{SelectFont, app_menus};

/// App title bar
pub struct AppTitleBar {
    app_menu_bar: Entity<AppMenuBar>,
    font_size_selector: Entity<FontSizeSelector>,
}

impl AppTitleBar {
    pub fn new(
        title: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let title = title.into();
        app_menus::init(title.clone(), cx);
        AppState::global_mut(cx).set_app_title(title);

        let font_size_selector = cx.new(|cx| FontSizeSelector::new(window, cx));
        let app_menu_bar = AppMenuBar::new(window, cx);

        Self {
            app_menu_bar,
            font_size_selector,
        }
    }
}

impl Render for AppTitleBar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        TitleBar::new()
            .child(
                div()
                    .flex()
                    .items_center()
                    .when(
                        !cfg!(any(target_os = "macos", target_family = "wasm")),
                        |this| this.child(self.app_menu_bar.clone()),
                    ),
            )
            .child(t!("app.title").to_string())
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_end()
                    .px_2()
                    .gap_2()
                    .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                    .child(self.font_size_selector.clone())
                    .child(
                        Button::new("title-settings-btn")
                            .small()
                            .ghost()
                            .icon(IconName::Settings)
                            .on_click(move |_ev, window: &mut Window, cx: &mut App| {
                                crate::panels::AppSettings::global_mut(cx).show_settings = true;
                                window.refresh();
                            }),
                    ),
            )
    }
}

/// Font size selector in title bar
struct FontSizeSelector {
    focus_handle: FocusHandle,
}

impl FontSizeSelector {
    pub fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }

    fn on_select_font(&mut self, font_size: &SelectFont, window: &mut Window, _cx: &mut Context<Self>) {
        crate::panels::AppSettings::global_mut(_cx).font_size = font_size.0 as f64;
        window.refresh();
    }
}

impl Render for FontSizeSelector {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle.clone();
        let font_size = crate::panels::AppSettings::global(cx).font_size as i32;

        div()
            .id("font-size-selector")
            .track_focus(&focus_handle)
            .on_action(cx.listener(Self::on_select_font))
            .child(
                Button::new("btn")
                    .small()
                    .ghost()
                    .icon(IconName::Settings2)
                    .dropdown_menu(move |this, _window, _cx| {
                        this.scrollable(true)
                            .check_side(Side::Right)
                            .max_h(px(480.))
                            .label(t!("title_bar.font_size.label").to_string())
                            .menu_with_check(
                                t!("title_bar.font_size.large").to_string(),
                                font_size == 18,
                                Box::new(SelectFont(18)),
                            )
                            .menu_with_check(
                                t!("title_bar.font_size.medium_default").to_string(),
                                font_size == 16,
                                Box::new(SelectFont(16)),
                            )
                            .menu_with_check(
                                t!("title_bar.font_size.small").to_string(),
                                font_size == 14,
                                Box::new(SelectFont(14)),
                            )
                    }),
            )
    }
}
