#[cfg(not(target_family = "wasm"))]
use anyhow::{Context, Result};
#[cfg(not(target_family = "wasm"))]
use tray_icon::{
    menu::{Menu, MenuEvent, MenuId, MenuItem, PredefinedMenuItem},
    MouseButton, TrayIcon, TrayIconBuilder, TrayIconEvent, TrayIconId,
};

#[cfg(not(target_family = "wasm"))]
const MENU_SHOW_ID: &str = "show_window";
#[cfg(not(target_family = "wasm"))]
const MENU_QUIT_ID: &str = "quit_app";
#[cfg(not(target_family = "wasm"))]
const TRAY_ICON_ID: &str = "com.example.{{crate_name}}.tray";

/// Initialize GTK on Linux (required before creating tray icon)
#[cfg(all(target_os = "linux", not(target_family = "wasm")))]
pub fn init_platform() -> Result<()> {
    gtk::init().context("Failed to initialize GTK")?;
    Ok(())
}

#[cfg(all(not(target_os = "linux"), not(target_family = "wasm")))]
pub fn init_platform() -> Result<()> {
    Ok(())
}

#[cfg(target_family = "wasm")]
pub fn init_platform() -> anyhow::Result<()> {
    Ok(())
}

/// System tray manager (native only)
#[cfg(not(target_family = "wasm"))]
pub struct SystemTray {
    #[allow(dead_code)]
    tray_icon: TrayIcon,
    show_menu_id: MenuId,
    quit_menu_id: MenuId,
}

#[cfg(not(target_family = "wasm"))]
impl SystemTray {
    pub fn new() -> Result<Self> {
        let tray_menu = Menu::new();
        let show_menu_id = MenuId::new(MENU_SHOW_ID);
        let quit_menu_id = MenuId::new(MENU_QUIT_ID);

        let show_item = MenuItem::with_id(show_menu_id.clone(), "Show Window", true, None);
        let separator = PredefinedMenuItem::separator();
        let quit_item = MenuItem::with_id(quit_menu_id.clone(), "Quit", true, None);

        tray_menu
            .append(&show_item)
            .context("Failed to append show item")?;
        tray_menu
            .append(&separator)
            .context("Failed to append separator")?;
        tray_menu
            .append(&quit_item)
            .context("Failed to append quit item")?;

        let icon = load_icon()?;
        let tray_icon = TrayIconBuilder::new()
            .with_id(TrayIconId::new(TRAY_ICON_ID))
            .with_menu(Box::new(tray_menu))
            .with_menu_on_left_click(false)
            .with_tooltip("{{project-name}}")
            .with_icon(icon)
            .build()
            .context("Failed to build tray icon")?;

        Ok(Self {
            tray_icon,
            show_menu_id,
            quit_menu_id,
        })
    }
}

#[cfg(not(target_family = "wasm"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayEvent {
    Show,
    Quit,
}

#[cfg(not(target_family = "wasm"))]
fn load_icon() -> Result<tray_icon::Icon> {
    let icon_bytes = include_bytes!("../../assets/icon.png");
    let image = image::load_from_memory(icon_bytes).context("Failed to load icon image")?;
    let rgba_image = image.to_rgba8();
    let (width, height) = rgba_image.dimensions();
    let rgba_data = rgba_image.into_raw();
    tray_icon::Icon::from_rgba(rgba_data, width, height).context("Failed to create tray icon")
}

/// Setup tray event handler
#[cfg(not(target_family = "wasm"))]
pub fn setup_tray_event_handler(tray: SystemTray, cx: &mut gpui::App) {
    let menu_event_receiver = MenuEvent::receiver().clone();
    let tray_icon_event_receiver = TrayIconEvent::receiver().clone();

    let show_menu_id = tray.show_menu_id.clone();
    let quit_menu_id = tray.quit_menu_id.clone();

    // Keep tray alive for the app lifetime
    let _tray = Box::leak(Box::new(tray));

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<TrayEvent>();

    std::thread::spawn(move || loop {
        let mut has_event = false;

        if let Ok(event) = tray_icon_event_receiver.try_recv() {
            has_event = true;
            if let TrayIconEvent::Click { button, .. } = event {
                if button == MouseButton::Left {
                    let _ = tx.send(TrayEvent::Show);
                }
            }
        }

        if let Ok(event) = menu_event_receiver.try_recv() {
            has_event = true;
            let menu_id = event.id();
            let tray_event = if menu_id == &show_menu_id {
                Some(TrayEvent::Show)
            } else if menu_id == &quit_menu_id {
                Some(TrayEvent::Quit)
            } else {
                None
            };

            if let Some(tray_event) = tray_event {
                if tx.send(tray_event).is_err() {
                    break;
                }
                if tray_event == TrayEvent::Quit {
                    break;
                }
            }
        }

        if !has_event {
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    });

    cx.spawn(async move |cx| {
        while let Some(event) = rx.recv().await {
            match event {
                TrayEvent::Show => {
                    let _ = cx.update(|cx| {
                        if let Some(window) = cx.windows().first() {
                            let _ = window.update(cx, |_, window, _| {
                                window.activate_window();
                            });
                        }
                    });
                }
                TrayEvent::Quit => {
                    let _ = cx.update(|cx| {
                        cx.quit();
                    });
                    break;
                }
            }
        }
        Ok::<(), anyhow::Error>(())
    })
    .detach();
}
