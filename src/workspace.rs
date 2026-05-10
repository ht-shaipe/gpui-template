use gpui::*;
use gpui_component::dock::{DockArea, DockItem, DockPlacement};
use std::sync::Arc;

use crate::panels::{CenterPanel, SamplePanel};
use crate::app_state::AppSettings;

pub struct Workspace {
    dock_area: Entity<DockArea>,
    docks_initialized: bool,
}

impl Workspace {
    pub fn new(window: &mut Window, cx: &mut App) -> Entity<Self> {
        let dock_area = cx.new(|cx| DockArea::new("main-dock", None, window, cx));

        // Create center panel
        let center_panel = cx.new(|cx| CenterPanel::new(window, cx));
        let center_item = DockItem::panel(Arc::new(center_panel.clone()));

        dock_area.update(cx, |view, cx| {
            view.set_center(center_item, window, cx);
        });

        cx.new(|_cx| Self {
            dock_area,
            docks_initialized: false,
        })
    }

    /// Initialize side/bottom docks once on first render
    fn init_docks(&mut self, window: &mut Window, cx: &mut App) {
        if self.docks_initialized {
            return;
        }
        self.docks_initialized = true;

        // Copy settings flags before mutable borrows
        let show_left = AppSettings::global(cx).show_left_panel;
        let show_right = AppSettings::global(cx).show_right_panel;

        let left_panel = cx.new(|cx| SamplePanel::with_name("Left Panel", cx));
        let left_item = DockItem::panel(Arc::new(left_panel));

        let right_panel = cx.new(|cx| SamplePanel::with_name("Right Panel", cx));
        let right_item = DockItem::panel(Arc::new(right_panel));

        let bottom_panel = cx.new(|cx| SamplePanel::with_name("Bottom Panel", cx));
        let bottom_item = DockItem::panel(Arc::new(bottom_panel));

        self.dock_area.update(cx, |view, cx| {
            view.set_left_dock(left_item, Some(px(250.)), show_left, window, cx);
            view.set_right_dock(right_item, Some(px(250.)), show_right, window, cx);
            view.set_bottom_dock(bottom_item, Some(px(150.)), true, window, cx);
        });
    }

    /// Sync dock open/close state with AppSettings flags
    fn sync_dock_visibility(&self, window: &mut Window, cx: &mut App) {
        let show_left = AppSettings::global(cx).show_left_panel;
        let show_right = AppSettings::global(cx).show_right_panel;

        self.dock_area.update(cx, |view, cx| {
            if view.is_dock_open(DockPlacement::Left, cx) != show_left {
                view.toggle_dock(DockPlacement::Left, window, cx);
            }
            if view.is_dock_open(DockPlacement::Right, cx) != show_right {
                view.toggle_dock(DockPlacement::Right, window, cx);
            }
        });
    }
}

impl Render for Workspace {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.init_docks(window, cx);
        self.sync_dock_visibility(window, cx);
        self.dock_area.clone()
    }
}
