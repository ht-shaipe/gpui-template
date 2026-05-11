//! Left panel - VS Code style file explorer

use gpui::prelude::*;
use gpui::InteractiveElement;
use gpui::*;
use gpui_component::dock::{Panel, PanelControl};
use gpui_component::ActiveTheme as _;
use gpui_component::scroll::ScrollableElement;
use std::collections::HashSet;

/// File tree node representation
#[derive(Debug, Clone)]
pub struct FileNode {
    pub name: SharedString,
    pub is_dir: bool,
    pub children: Vec<FileNode>,
    pub path: SharedString,
}

impl FileNode {
    pub fn new(name: impl Into<SharedString>, is_dir: bool, path: impl Into<SharedString>) -> Self {
        Self {
            name: name.into(),
            is_dir,
            children: Vec::new(),
            path: path.into(),
        }
    }
}

/// Left panel with file explorer
pub struct LeftPanel {
    focus_handle: FocusHandle,
    root_nodes: Vec<FileNode>,
    collapsed_dirs: HashSet<SharedString>,
}

impl LeftPanel {
    pub fn new(_window: &mut Window, cx: &mut App) -> Self {
        // Build sample file tree
        let mut src_folder = FileNode::new("src", true, "src/");
        src_folder.children.push(FileNode::new("main.rs", false, "src/main.rs"));
        src_folder.children.push(FileNode::new("lib.rs", false, "src/lib.rs"));
        
        let mut app_folder = FileNode::new("app", true, "src/app/");
        app_folder.children.push(FileNode::new("mod.rs", false, "src/app/mod.rs"));
        src_folder.children.push(app_folder);
        
        let mut components_folder = FileNode::new("components", true, "src/components/");
        components_folder.children.push(FileNode::new("button.rs", false, "src/components/button.rs"));
        src_folder.children.push(components_folder);
        
        let mut root_nodes = Vec::new();
        root_nodes.push(src_folder);
        root_nodes.push(FileNode::new("Cargo.toml", false, "Cargo.toml"));
        root_nodes.push(FileNode::new("README.md", false, "README.md"));
        root_nodes.push(FileNode::new("LICENSE", false, "LICENSE"));

        Self {
            focus_handle: cx.focus_handle(),
            root_nodes,
            collapsed_dirs: HashSet::new(),
        }
    }
    
    fn toggle_dir(&mut self, path: SharedString, cx: &mut Context<Self>) {
        if self.collapsed_dirs.contains(&path) {
            self.collapsed_dirs.remove(&path);
        } else {
            self.collapsed_dirs.insert(path);
        }
        cx.notify();
    }
}

impl Panel for LeftPanel {
    fn panel_name(&self) -> &'static str {
        "Explorer"
    }

    fn title(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().h_px().into_any_element()
    }

    fn zoomable(&self, _cx: &App) -> Option<PanelControl> {
        None
    }
}

impl EventEmitter<gpui_component::dock::PanelEvent> for LeftPanel {}

impl Focusable for LeftPanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for LeftPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        
        div()
            .id("left-panel")
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(theme.colors.background)
            .child(self.render_header(cx))
            .child(self.render_search(cx))
            .child(self.render_file_tree(cx))
    }
}

impl LeftPanel {
    fn render_header(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .flex()
            .flex_row()
            .items_center()
            .h(px(35.))
            .px(px(12.))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.))
                    .child(
                        div()
                            .id("more-menu")
                            .flex()
                            .items_center()
                            .justify_center()
                            .w(px(22.))
                            .h(px(22.))
                            .rounded(px(4.))
                            .cursor_pointer()
                            .text_color(theme.colors.muted_foreground)
                            .text_size(px(14.))
                            .child("⋯")
                    )
                    .child(
                        div()
                            .text_size(px(11.))
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.colors.foreground)
                            .child("EXPLORER")
                    )
            )
            .child(div().flex_1())
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(2.))
                    .child(
                        div()
                            .id("collapse-all")
                            .flex()
                            .items_center()
                            .justify_center()
                            .w(px(22.))
                            .h(px(22.))
                            .rounded(px(4.))
                            .cursor_pointer()
                            .hover(|s| s.bg(theme.colors.accent))
                            .text_color(theme.colors.muted_foreground)
                            .text_size(px(14.))
                            .child("⏷")
                    )
                    .child(
                        div()
                            .id("add-file")
                            .flex()
                            .items_center()
                            .justify_center()
                            .w(px(22.))
                            .h(px(22.))
                            .rounded(px(4.))
                            .cursor_pointer()
                            .hover(|s| s.bg(theme.colors.accent))
                            .text_color(theme.colors.muted_foreground)
                            .text_size(px(14.))
                            .child("+")
                    )
            )
    }
    
    fn render_search(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        
        div()
            .flex()
            .flex_row()
            .items_center()
            .h(px(28.))
            .mx(px(8.))
            .mb(px(4.))
            .px(px(8.))
            .rounded(px(6.))
            .bg(theme.colors.input)
            .border(px(1.))
            .border_color(theme.colors.border)
            .child(
                div()
                    .text_color(theme.colors.muted_foreground)
                    .text_size(px(13.))
                    .mr(px(6.))
                    .child("🔍")
            )
            .child(
                div()
                    .text_size(px(13.))
                    .text_color(theme.colors.muted_foreground)
                    .child("Search")
            )
    }
    
    fn render_file_tree(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .flex_1()
            .overflow_y_scrollbar()
            .children(self.root_nodes.iter().map(|node| {
                self.render_node(node, 0, cx)
            }))
    }
    
    fn render_node(&self, node: &FileNode, depth: usize, cx: &mut Context<Self>) -> AnyElement {
        let theme = cx.theme();
        let indent = depth * 12;
        let is_collapsed = self.collapsed_dirs.contains(&node.path);
        
        if node.is_dir {
            div()
                .flex()
                .flex_col()
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .pl(px(indent as f32))
                        .h(px(22.))
                        .cursor_pointer()
                        .hover(|s| s.bg(theme.colors.accent))
                        .child(
                            div()
                                .text_size(px(14.))
                                .text_color(theme.colors.muted_foreground)
                                .child(if is_collapsed { "▶" } else { "▼" })
                        )
                        .child(
                            div()
                                .text_size(px(14.))
                                .ml(px(4.))
                                .text_color(theme.colors.primary)
                                .child(if is_collapsed { "📁" } else { "📂" })
                        )
                        .child(
                            div()
                                .flex_1()
                                .ml(px(4.))
                                .text_size(px(13.))
                                .text_color(theme.colors.foreground)
                                .child(node.name.clone())
                        )
                )
                .when(!is_collapsed, |el| {
                    el.children(node.children.iter().map(|child| {
                        self.render_node(child, depth + 1, cx)
                    }))
                })
                .into_any_element()
        } else {
            div()
                .flex()
                .flex_row()
                .items_center()
                .pl(px(indent as f32))
                .h(px(22.))
                .cursor_pointer()
                .hover(|s| s.bg(theme.colors.accent))
                .child(
                    div()
                        .text_size(px(14.))
                        .text_color(theme.colors.muted_foreground)
                        .mr(px(4.))
                        .child("📄")
                )
                .child(
                    div()
                        .flex_1()
                        .text_size(px(13.))
                        .text_color(theme.colors.foreground)
                        .child(node.name.clone())
                )
                .into_any_element()
        }
    }
}