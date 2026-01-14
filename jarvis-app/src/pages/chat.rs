use crate::components::{Button, MessageView};
use jarvis_ai::Message;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

/// Chat page with text-based interface
#[component]
pub fn ChatPage() -> impl IntoView {
    let (messages, set_messages) = signal(Vec::<Message>::new());
    let (input, set_input) = signal(String::new());
    let (is_loading, set_loading) = signal(false);
    let navigate = use_navigate();

    let send_message = move || {
        let text = input.get();
        if text.trim().is_empty() {
            return;
        }

        set_loading.set(true);
        let user_msg = Message::user(text.clone());

        set_messages.update(|msgs| msgs.push(user_msg));
        set_input.set(String::new());

        // TODO: Actually process the message with AI
        leptos::task::spawn_local(async move {
            // Simulate processing
            let response = Message::assistant("I'm sorry, but I'm still being implemented. Please check back soon!".to_string());
            set_messages.update(|msgs| msgs.push(response));
            set_loading.set(false);
        });
    };

    let go_home = move || {
        navigate("/", Default::default());
    };

    view! {
        <div class="min-h-screen flex flex-col p-8">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold text-white">"Chat with JARVIS"</h1>
                <Button on_click=Box::new(go_home) variant=crate::components::button::ButtonVariant::Secondary>
                    "Back to Home"
                </Button>
            </div>

            <div class="flex-1 overflow-y-auto space-y-4 mb-4">
                {move || messages.get().into_iter().map(|msg| {
                    view! { <MessageView message=msg/> }
                }).collect::<Vec<_>>()}
            </div>

            <div class="flex gap-4">
                <input
                    type="text"
                    class="flex-1 bg-gray-800 text-white px-4 py-3 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                    placeholder="Type your message..."
                    prop:value=move || input.get()
                    on:input=move |ev| set_input.set(event_target_value(&ev))
                    on:keypress=move |ev| {
                        if ev.key() == "Enter" {
                            send_message();
                        }
                    }
                />
                <Button
                    on_click=Box::new(send_message)
                    disabled=is_loading.get()
                >
                    {move || if is_loading.get() { "Sending..." } else { "Send" }}
                </Button>
            </div>
        </div>
    }
}
