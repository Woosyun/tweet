pub mod app;
pub mod mail;
pub mod user;
pub mod pages;

use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature="ssr")] {
        pub mod db;
        pub mod auth;

        use axum_macros::FromRef;
        #[derive(Clone, FromRef)]
        pub struct AppState {
            pub db: crate::db::DB,
            pub leptos_options: leptos::config::LeptosOptions,
        }
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
