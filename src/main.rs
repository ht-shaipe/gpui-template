#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gpui::{Application, App};

fn main() {
    #[cfg(target_os = "windows")]
    std::env::set_var("GPUI_DISABLE_DIRECT_COMPOSITION", "true");

    Application::new()
        .with_assets({{crate_name}}::Assets)
        .run(move |cx: &mut App| {
            {{crate_name}}::init(cx);

            // System tray
            if let Err(e) = {{crate_name}}::system_tray::init_platform() {
                log::error!("Failed to init tray platform: {}", e);
            }
            match {{crate_name}}::system_tray::SystemTray::new() {
                Ok(tray) => {
                    {{crate_name}}::system_tray::setup_tray_event_handler(tray, cx);
                }
                Err(e) => log::error!("Failed to init system tray: {}", e),
            }

            cx.on_action(|_: &{{crate_name}}::Quit, cx| {
                cx.quit();
            });

            {{crate_name}}::open_new("{{project-name}}", |window, cx| {{crate_name}}::Workspace::new(window, cx), cx);
        });
}
