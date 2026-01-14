use crate::components::{Button, MessageView};
use crate::state::AiService;
use jarvis_ai::{Message, ModelType};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use std::rc::Rc;

/// Chat page with text-based interface
#[component]
pub fn ChatPage() -> impl IntoView {
    let (messages, set_messages) = signal(Vec::<Message>::new());
    let (input, set_input) = signal(String::new());
    let (is_loading, set_loading) = signal(false);
    let (model_status, set_model_status) = signal("Not loaded".to_string());
    let (error_msg, set_error) = signal(Option::<String>::None);
    let navigate = use_navigate();

    // Create AI service
    let ai_service = Rc::new(AiService::new());
    
    // Initialize model on mount
    let ai_service_init = ai_service.clone();
    let _ = {
        let service = ai_service_init.clone();
        leptos::task::spawn_local(async move {
            set_model_status.set("Loading model...".to_string());
            match service.load_model(ModelType::TinyLlama).await {
                Ok(_) => {
                    set_model_status.set("Model ready".to_string());
                    log::info!("Model loaded successfully");
                }
                Err(e) => {
                    set_model_status.set(format!("Load failed: {}", e));
                    log::error!("Failed to load model: {}", e);
                }
            }
        });
    };

    let ai_service_send = ai_service.clone();
    let do_send = Rc::new(move || {
        let text = input.get();
        if text.trim().is_empty() {
            return;
        }

        set_loading.set(true);
        set_error.set(None);
        let user_msg = Message::user(text.clone());

        set_messages.update(|msgs| msgs.push(user_msg.clone()));
        set_input.set(String::new());

        let service = ai_service_send.clone();
        let msgs = messages.get();
        
        leptos::task::spawn_local(async move {
            // Try to generate response using AI
            match service.generate(&msgs).await {
                Ok(response_text) => {
                    let response = Message::assistant(response_text);
                    set_messages.update(|msgs| msgs.push(response));
                }
                Err(e) => {
                    // For now, provide a helpful response explaining the limitation
                    let fallback = if e.contains("not yet implemented") {
                        Message::assistant(
                            "I'm JARVIS, your AI assistant. While my neural networks are still \
                            being configured, I can tell you that this Rust/WebAssembly version \
                            is designed to run entirely in your browser - no server required! \
                            The AI inference implementation using Burn ML framework is in progress. \
                            Once complete, I'll be able to help you with various tasks using \
                            local language models like TinyLlama.".to_string()
                        )
                    } else {
                        log::warn!("AI generation error: {}", e);
                        set_error.set(Some(e.clone()));
                        Message::assistant(format!(
                            "I encountered an issue: {}. Please try again.", e
                        ))
                    };
                    set_messages.update(|msgs| msgs.push(fallback));
                }
            }
            set_loading.set(false);
        });
    });

    let send_for_keypress = do_send.clone();
    let send_for_button = do_send.clone();

    let clear_chat = move || {
        set_messages.set(Vec::new());
        set_error.set(None);
    };

    let go_home = move || {
        navigate("/", Default::default());
    };

    view! {
        <div class="min-h-screen flex flex-col p-8 bg-gray-900">
            // Header
            <div class="flex items-center justify-between mb-6">
                <div class="flex items-center gap-4">
                    <h1 class="text-4xl font-bold text-white">"Chat with JARVIS"</h1>
                    <span class="text-sm px-3 py-1 rounded-full bg-gray-700 text-gray-300">
                        {move || model_status.get()}
                    </span>
                </div>
                <div class="flex gap-2">
                    <Button on_click=Box::new(clear_chat) variant=crate::components::button::ButtonVariant::Secondary>
                        "Clear"
                    </Button>
                    <Button on_click=Box::new(go_home) variant=crate::components::button::ButtonVariant::Secondary>
                        "Home"
                    </Button>
                </div>
            </div>

            // Error banner
            {move || error_msg.get().map(|e| view! {
                <div class="mb-4 p-4 bg-red-900/50 border border-red-500 rounded-lg text-red-200">
                    <span class="font-semibold">"Error: "</span>
                    {e}
                </div>
            })}

            // Messages area
            <div class="flex-1 overflow-y-auto space-y-4 mb-4 p-4 bg-gray-800/50 rounded-lg">
                {move || {
                    let msgs = messages.get();
                    if msgs.is_empty() {
                        view! {
                            <div class="text-center text-gray-400 py-8">
                                <p class="text-xl mb-2">"Welcome to JARVIS"</p>
                                <p class="text-sm">"Type a message below to start a conversation."</p>
                            </div>
                        }.into_any()
                    } else {
                        msgs.into_iter().map(|msg| {
                            view! { <MessageView message=msg/> }
                        }).collect::<Vec<_>>().into_any()
                    }
                }}
            </div>

            // Input area
            <div class="flex gap-4">
                <input
                    type="text"
                    class="flex-1 bg-gray-800 text-white px-4 py-3 rounded-lg border border-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                    placeholder="Type your message..."
                    prop:value=move || input.get()
                    on:input=move |ev| set_input.set(event_target_value(&ev))
                    on:keypress=move |ev| {
                        if ev.key() == "Enter" && !is_loading.get() {
                            send_for_keypress();
                        }
                    }
                    disabled=is_loading.get()
                />
                <Button
                    on_click=Box::new(move || send_for_button())
                    disabled=is_loading.get()
                >
                    {move || if is_loading.get() { "Processing..." } else { "Send" }}
                </Button>
            </div>

            // Status bar
            <div class="mt-4 text-xs text-gray-500 text-center">
                "JARVIS v0.1.0 - Running locally in your browser via WebAssembly"
            </div>
        </div>
    }
}
