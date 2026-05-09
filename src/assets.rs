//! Re-export of `gpui-component-assets` [`Assets`].
//! On native, icons are embedded; on WASM, construct with [`gpui_component_assets::Assets::new`](...) in `init_web`.

pub use gpui_component_assets::Assets;
