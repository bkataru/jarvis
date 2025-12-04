use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use wasm_bindgen::prelude::*;

mod components;
mod pages;
mod state;
mod utils;

use pages::{chat::ChatPage, home::HomePage, mcp::McpPage};

/// Main application component
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main class="min-h-screen bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900">
                <Routes fallback=|| "Page not found.">
                    <Route path=path!("/") view=HomePage/>
                    <Route path=path!("/chat") view=ChatPage/>
                    <Route path=path!("/mcp") view=McpPage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Entry point for the WASM module
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main() {
    // Set up panic hook for better error messages
    console_error_panic_hook::set_once();

    // Initialize logging
    console_log::init_with_level(log::Level::Debug).expect("Failed to initialize logger");

    log::info!("JARVIS initializing...");

    // Mount the application
    leptos::mount::mount_to_body(App);
}
