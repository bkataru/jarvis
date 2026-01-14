use crate::components::{Button, JarvisRing};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

/// Home page with JARVIS voice interface
#[component]
pub fn HomePage() -> impl IntoView {
    let (is_listening, set_listening) = signal(false);
    let navigate = use_navigate();
    let navigate_clone = navigate.clone();

    let toggle_listening = move || {
        set_listening.update(|listening| *listening = !*listening);
    };

    let go_to_chat = move || {
        navigate("/chat", Default::default());
    };

    let go_to_mcp = move || {
        navigate_clone("/mcp", Default::default());
    };

    view! {
        <div class="min-h-screen flex flex-col items-center justify-center p-8">
            <h1 class="text-6xl font-bold text-white mb-12">JARVIS</h1>

            <div
                class="cursor-pointer"
                on:click=move |_| toggle_listening()
            >
                <JarvisRing active=is_listening.get()/>
            </div>

            <p class="text-white text-xl mt-8">
                {move || if is_listening.get() {
                    "Listening..."
                } else {
                    "Click to activate"
                }}
            </p>

            <div class="flex gap-4 mt-12">
                <Button on_click=Box::new(go_to_chat)>
                    "Chat Mode"
                </Button>
                <Button on_click=Box::new(go_to_mcp) variant=crate::components::button::ButtonVariant::Secondary>
                    "MCP Settings"
                </Button>
            </div>
        </div>
    }
}
