use leptos::prelude::*;

/// JARVIS ring animation component
#[component]
pub fn JarvisRing(
    /// Whether the ring is active/pulsing
    #[prop(default = false)] active: bool,
) -> impl IntoView {
    let ring_class = if active {
        "animate-pulse ring-4 ring-blue-500"
    } else {
        "ring-2 ring-gray-500"
    };

    view! {
        <div class="relative w-64 h-64 flex items-center justify-center">
            <div class={format!(
                "absolute inset-0 rounded-full {} transition-all duration-300",
                ring_class
            )}></div>
            <div class="absolute inset-4 rounded-full bg-gradient-to-br from-blue-900 to-blue-700"></div>
            <div class="absolute inset-8 rounded-full bg-slate-900 flex items-center justify-center">
                <svg
                    class="w-24 h-24 text-blue-400"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z"
                    />
                </svg>
            </div>
        </div>
    }
}
