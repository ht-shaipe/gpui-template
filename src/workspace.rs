use gpui::*;
use gpui_component::dock::{Dock, DockItem, PanelRegistry, TabPlacement};
use gpui_component::v_flex;

use crate::panels::SamplePanel;

pub struct Workspace {
    dock: Entity<Dock>,
}

impl Workspace {
    pub fn new(window: &mut Window, cx: &mut App) -> Entity<Self> {
        let panel_registry = cx.new(|_| PanelRegistry::new());
        let sample = cx.new(|cx| SamplePanel::new(window, cx));
        let dock = cx.new(|cx| {
            let mut dock = Dock::new(TabPlacement::Top, panel_registry, window, cx);
            dock.add_item(
                DockItem::panel(sample.into(), window, cx),
                gpui_component::dock::DockPlacement::Center,
                true,
                window,
                cx,
            );
            dock
        });

        cx.new(|_| Self { dock })
    }
}

impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().size_full().child(self.dock.clone())
    }
}
