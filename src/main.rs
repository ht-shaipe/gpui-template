#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    #[cfg(target_os = "windows")]
    std::env::set_var("GPUI_DISABLE_DIRECT_COMPOSITION", "true");

    gpui_platform::application()
        .with_assets(gpui_template::Assets)
        .run(move |cx| {
            gpui_template::init(cx);

            // System tray
            if let Err(e) = gpui_template::system_tray::init_platform() {
                log::error!("Failed to init tray platform: {}", e);
            }
            match gpui_template::system_tray::SystemTray::new() {
                Ok(tray) => {
                    gpui_template::system_tray::setup_tray_event_handler(tray, cx);
                }
                Err(e) => log::error!("Failed to init system tray: {}", e),
            }

            cx.on_action(|_: &gpui_template::Quit, cx| {
                cx.quit();
            });

            gpui_template::open_new("GPUI Template", |window, cx| gpui_template::Workspace::new(window, cx), cx);
        });
}
