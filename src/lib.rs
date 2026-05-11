mod app;
mod assets;
mod core;
mod i18n;
mod panels;
mod workspace;

rust_i18n::i18n!("locales", fallback = "en");

pub use assets::Assets;

pub use app::{
    actions::{
        About, CloseWindow, Open, OpenSettings, Quit, SelectFont, SelectLocale, SelectRadius,
        SwitchTheme, SwitchThemeMode, ToggleSearch,
    },
    app_menus, app_state, key_binding, system_tray, themes, title_bar,
};
pub use panels::{AppSettings, SettingsPanel};
pub use workspace::Workspace;

use gpui::{
    div, AnyView, App, AppContext as _, Context, Entity, IntoElement, ParentElement, Render,
    SharedString, Styled, Window, WindowOptions,
};
#[cfg(not(target_family = "wasm"))]
use gpui::{px, size, Bounds, WindowBounds, WindowKind};
#[cfg(not(target_family = "wasm"))]
use gpui_component::TitleBar;
use gpui_component::{
    dock::{register_panel, PanelInfo},
    Root,
};

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

const PANEL_NAME: &str = "DockPanelContainer";

#[cfg(target_family = "wasm")]
const GPUI_COMPONENT_ASSETS_BASE: &str = "/gpui-component/gallery/";

// ---- WASM entry ----

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub fn init_web() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info).expect("Failed to initialize logger");
    tracing_wasm::set_as_global_default();

    let app = gpui_platform::single_threaded_web();

    app.with_assets(gpui_component_assets::Assets::new(SharedString::from(
        GPUI_COMPONENT_ASSETS_BASE,
    )))
    .run(move |cx: &mut App| {
        let http_client = unsafe {
            gpui_web::FetchHttpClient::with_user_agent("gpui-template/0.1.0")
                .expect("failed to create FetchHttpClient")
        };
        cx.set_http_client(std::sync::Arc::new(http_client));

        gpui_component::init(cx);
        app_state::AppState::init(cx);
        themes::init(cx);
        i18n::init(cx);
        key_binding::init(cx);

        register_panel(cx, PANEL_NAME, |_dock_area, _state, info, window, cx| {
            let _state = match info {
                PanelInfo::Panel(value) => panels::DockPanelState::from_value(value.clone()),
                _ => panels::DockPanelState::default(),
            };
            let panel: gpui::Entity<panels::SamplePanel> =
                cx.new(|cx| panels::SamplePanel::new(window, cx));
            Box::new(panel) as Box<dyn gpui_component::dock::PanelView>
        });

        cx.on_action(|_: &Quit, cx| {
            cx.quit();
        });

        cx.open_window(WindowOptions::default(), |window, cx| {
            let workspace = Workspace::new(window, cx);
            cx.new(|cx| Root::new(workspace, window, cx))
        })
        .expect("failed to open window");

        cx.activate(true);
    });

    Ok(())
}

// ---- Desktop-only components ----

#[cfg(not(target_family = "wasm"))]
struct DockRoot {
    title_bar: Entity<title_bar::AppTitleBar>,
    view: AnyView,
}

#[cfg(not(target_family = "wasm"))]
impl DockRoot {
    pub fn new(
        title: impl Into<SharedString>,
        view: impl Into<AnyView>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let title_bar = cx.new(|cx| title_bar::AppTitleBar::new(title, window, cx));
        Self {
            title_bar,
            view: view.into(),
        }
    }
}

#[cfg(not(target_family = "wasm"))]
impl Render for DockRoot {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sheet_layer = Root::render_sheet_layer(window, cx);
        let dialog_layer = Root::render_dialog_layer(window, cx);
        let notification_layer = Root::render_notification_layer(window, cx);

        div()
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .child(self.title_bar.clone())
            .child(
                div()
                    .flex_1()
                    .w_full()
                    .overflow_hidden()
                    .child(self.view.clone()),
            )
            .children(sheet_layer)
            .children(dialog_layer)
            .children(notification_layer)
    }
}

#[cfg(not(target_family = "wasm"))]
pub fn open_new<F, E>(title: &str, crate_view_fn: F, cx: &mut App)
where
    E: Into<AnyView>,
    F: FnOnce(&mut Window, &mut App) -> E + 'static,
{
    let title = SharedString::from(title.to_string());

    let mut window_size = size(px(1200.0), px(800.0));
    if let Some(display) = cx.primary_display() {
        let display_size = display.bounds().size;
        window_size.width = window_size.width.min(display_size.width * 0.85);
        window_size.height = window_size.height.min(display_size.height * 0.85);
    }
    let window_bounds = Bounds::centered(None, window_size, cx);

    cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(window_bounds)),
            titlebar: Some(TitleBar::title_bar_options()),
            window_min_size: Some(gpui::Size {
                width: px(480.),
                height: px(320.),
            }),
            kind: WindowKind::Normal,
            ..Default::default()
        },
        |window, cx| {
            let view = crate_view_fn(window, cx);
            let root = cx.new(|cx| DockRoot::new(title.clone(), view, window, cx));
            cx.new(|cx| Root::new(root, window, cx))
        },
    )
    .expect("failed to open window")
    .update(cx, |_, window, _| {
        window.activate_window();
        window.set_window_title(&title);
    })
    .expect("failed to update window");
}

#[cfg(not(target_family = "wasm"))]
pub fn init(cx: &mut App) {
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    let _ = tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("gpui_component=trace".parse().unwrap()),
        )
        .try_init();

    gpui_component::init(cx);
    app_state::AppState::init(cx);
    themes::init(cx);
    i18n::init(cx);
    app_menus::init("GPUI Template", cx);
    key_binding::init(cx);

    register_panel(cx, PANEL_NAME, |_dock_area, _state, info, window, cx| {
        let _state = match info {
            PanelInfo::Panel(value) => panels::DockPanelState::from_value(value.clone()),
            _ => panels::DockPanelState::default(),
        };
        let panel: gpui::Entity<panels::SamplePanel> =
            cx.new(|cx| panels::SamplePanel::new(window, cx));
        Box::new(panel) as Box<dyn gpui_component::dock::PanelView>
    });

    cx.activate(true);
}
