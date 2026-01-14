use jarvis_ai::{Message, MessagePart, MessageRole};
use leptos::prelude::*;

/// Message view component
#[component]
pub fn MessageView(message: Message) -> impl IntoView {
    let is_user = matches!(message.role, MessageRole::User);
    let bg_class = if is_user {
        "bg-blue-600"
    } else {
        "bg-gray-700"
    };
    let align_class = if is_user {
        "ml-auto"
    } else {
        "mr-auto"
    };

    let content = message
        .message_parts
        .iter()
        .filter_map(|part| match part {
            MessagePart::Text(text_part) => Some(text_part.text.clone()),
            MessagePart::ToolCall(tool_part) => Some(format!(
                "[Tool: {}] {}",
                tool_part.function_name, tool_part.response
            )),
        })
        .collect::<Vec<_>>()
        .join("\n");

    view! {
        <div class={format!("max-w-3xl p-4 rounded-lg {} {}", bg_class, align_class)}>
            <div class="text-white whitespace-pre-wrap">{content}</div>
        </div>
    }
}
