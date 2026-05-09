use gpui::*;
use gpui_component::dock::{DockArea, DockItem};
use std::sync::Arc;

use crate::panels::{CenterPanel, SamplePanel};

pub struct Workspace {
    dock_area: Entity<DockArea>,
}

impl Workspace {
    pub fn new(window: &mut Window, cx: &mut App) -> Entity<Self> {
        let dock_area =
            cx.new(|cx| DockArea::new("main-dock", None, window, cx));
        let weak_dock_area = dock_area.downgrade();

        // Create left panel (no title bar, no zoom)
        let left_panel = cx.new(|cx| SamplePanel::with_name("Left Panel", cx));
        let left_item = DockItem::panel(Arc::new(left_panel.clone()));

        // Create center panel with tabs
        let center_panel = cx.new(|cx| CenterPanel::new(window, cx));
        let center_item = DockItem::tab(center_panel, &weak_dock_area, window, cx);

        // Create right panel
        let right_panel = cx.new(|cx| SamplePanel::with_name("Right Panel", cx));
        let right_item = DockItem::tab(right_panel, &weak_dock_area, window, cx);

        // Create bottom panel
        let bottom_panel = cx.new(|cx| SamplePanel::with_name("Bottom Panel", cx));
        let bottom_item = DockItem::tab(bottom_panel, &weak_dock_area, window, cx);

        dock_area.update(cx, |view, cx| {
            view.set_left_dock(left_item.clone(), Some(px(250.)), true, window, cx);
            view.set_center(center_item.clone(), window, cx);
            view.set_right_dock(right_item.clone(), Some(px(250.)), true, window, cx);
            view.set_bottom_dock(bottom_item.clone(), Some(px(150.)), true, window, cx);
        });

        cx.new(|_| Self { dock_area })
    }
}

impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.dock_area.clone()
    }
}
