mod app;
mod assets;
mod i18n;
mod panels;
mod workspace;

rust_i18n::i18n!("locales", fallback = "en");

pub use assets::Assets;

pub use app::{
    actions::{
        About, CloseWindow, Open, Quit, SelectFont, SelectLocale, SelectRadius, SwitchTheme,
        SwitchThemeMode, ToggleSearch,
    },
    app_menus, app_state, key_binding, system_tray, themes, title_bar,
};
pub use panels::{AppSettings, SettingsPanel};

use gpui::{
    AnyView, App, Bounds, Context, Entity, IntoElement, ParentElement, Pixels, Render,
    SharedString, Size, Styled, Window, WindowBounds, WindowKind, WindowOptions, div, px, size,
};
use gpui_component::{
    Root, TitleBar,
    dock::{PanelInfo, register_panel},
    v_flex,
};

const PANEL_NAME: &str = "DockPanelContainer";

/// Root component
struct DockRoot {
    title_bar: Entity<title_bar::AppTitleBar>,
    view: AnyView,
}

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

impl Render for DockRoot {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sheet_layer = Root::render_sheet_layer(window, cx);
        let dialog_layer = Root::render_dialog_layer(window, cx);
        let notification_layer = Root::render_notification_layer(window, cx);

        div()
            .size_full()
            .child(
                v_flex()
                    .size_full()
                    .child(self.title_bar.clone())
                    .child(div().flex_1().overflow_hidden().child(self.view.clone())),
            )
            .children(sheet_layer)
            .children(dialog_layer)
            .children(notification_layer)
    }
}

/// Create a new window
pub fn open_new<F, E>(title: &str, crate_view_fn: F, cx: &mut App)
where
    E: Into<AnyView>,
    F: FnOnce(&mut Window, &mut App) -> E + Send + 'static,
{
    let mut window_size = size(px(1200.0), px(800.0));
    if let Some(display) = cx.primary_display() {
        let display_size = display.bounds().size;
        window_size.width = window_size.width.min(display_size.width * 0.85);
        window_size.height = window_size.height.min(display_size.height * 0.85);
    }
    let window_bounds = Bounds::centered(None, window_size, cx);
    let title = SharedString::from(title.to_string());

    cx.spawn(async move |cx| {
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(window_bounds)),
            titlebar: Some(TitleBar::title_bar_options()),
            window_min_size: Some(gpui::Size {
                width: px(480.),
                height: px(320.),
            }),
            kind: WindowKind::Normal,
            ..Default::default()
        };

        let window = cx
            .open_window(options, |window, cx| {
                let view = crate_view_fn(window, cx);
                let root = cx.new(|cx| DockRoot::new(title.clone(), view, window, cx));
                cx.new(|cx| Root::new(root, window, cx))
            })
            .expect("failed to open window");

        window
            .update(cx, |_, window, _| {
                window.activate_window();
                window.set_window_title(&title);
            })
            .expect("failed to update window");

        Ok::<_, anyhow::Error>(())
    })
    .detach();
}

/// Initialize the app
pub fn init(cx: &mut App) {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("gpui_component=trace".parse().unwrap()),
        )
        .init();

    gpui_component::init(cx);
    app_state::AppState::init(cx);
    themes::init(cx);
    i18n::init(cx);
    app_menus::init("GPUI Template", cx);
    key_binding::init(cx);

    register_panel(cx, PANEL_NAME, |_, _, info, window, cx| {
        let state = match info {
            PanelInfo::Panel(value) => panels::DockPanelState::from_value(value.clone()),
            _ => panels::DockPanelState::default(),
        };
        Box::new(panels::SamplePanel::new(window, cx))
    });

    cx.activate(true);
}
