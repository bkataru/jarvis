use crate::components::Button;
use jarvis_mcp::McpServerConfig;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

/// MCP settings page
#[component]
pub fn McpPage() -> impl IntoView {
    let (servers, set_servers) = signal(Vec::<McpServerConfig>::new());
    let (new_server_name, set_new_server_name) = signal(String::new());
    let (new_server_url, set_new_server_url) = signal(String::new());
    let navigate = use_navigate();

    let add_server = move || {
        let name = new_server_name.get();
        let url = new_server_url.get();

        if name.trim().is_empty() || url.trim().is_empty() {
            return;
        }

        let config = McpServerConfig {
            name: name.clone(),
            url: Some(url.clone()),
            server_type: None,
            active: true,
            active_tools: vec![],
            active_prompts: vec![],
        };

        set_servers.update(|s| s.push(config));
        set_new_server_name.set(String::new());
        set_new_server_url.set(String::new());
    };

    let go_home = move || {
        navigate("/", Default::default());
    };

    view! {
        <div class="min-h-screen p-8">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold text-white">"MCP Servers"</h1>
                <Button on_click=Box::new(go_home) variant=crate::components::button::ButtonVariant::Secondary>
                    "Back to Home"
                </Button>
            </div>

            <div class="max-w-2xl mx-auto">
                <div class="bg-gray-800 rounded-lg p-6 mb-8">
                    <h2 class="text-2xl font-bold text-white mb-4">"Add Server"</h2>
                    <div class="space-y-4">
                        <input
                            type="text"
                            class="w-full bg-gray-700 text-white px-4 py-2 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                            placeholder="Server name"
                            prop:value=move || new_server_name.get()
                            on:input=move |ev| set_new_server_name.set(event_target_value(&ev))
                        />
                        <input
                            type="text"
                            class="w-full bg-gray-700 text-white px-4 py-2 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                            placeholder="Server URL"
                            prop:value=move || new_server_url.get()
                            on:input=move |ev| set_new_server_url.set(event_target_value(&ev))
                        />
                        <Button on_click=Box::new(add_server)>
                            "Add Server"
                        </Button>
                    </div>
                </div>

                <div class="space-y-4">
                    {move || servers.get().into_iter().map(|server| {
                        let server_name = server.name.clone();
                        let server_url = server.url.clone().unwrap_or_default();
                        view! {
                            <div class="bg-gray-800 rounded-lg p-4">
                                <div class="flex items-center justify-between">
                                    <div>
                                        <h3 class="text-xl font-bold text-white">{server_name}</h3>
                                        <p class="text-gray-400">{server_url}</p>
                                    </div>
                                    <div class={if server.active {
                                        "w-3 h-3 bg-green-500 rounded-full"
                                    } else {
                                        "w-3 h-3 bg-red-500 rounded-full"
                                    }}></div>
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}
