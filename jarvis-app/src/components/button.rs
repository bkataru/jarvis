use leptos::prelude::*;

/// Button component with Tailwind styling
#[component]
pub fn Button(
    /// Button text
    children: Children,
    /// Click handler
    #[prop(optional)]
    on_click: Option<Box<dyn Fn() + 'static>>,
    /// Whether button is disabled
    #[prop(default = false)]
    disabled: bool,
    /// Button variant
    #[prop(default = ButtonVariant::Primary)]
    variant: ButtonVariant,
) -> impl IntoView {
    let class = match variant {
        ButtonVariant::Primary => "bg-blue-600 hover:bg-blue-700 text-white",
        ButtonVariant::Secondary => "bg-gray-600 hover:bg-gray-700 text-white",
        ButtonVariant::Danger => "bg-red-600 hover:bg-red-700 text-white",
    };

    view! {
        <button
            class={format!(
                "px-4 py-2 rounded-lg font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed {}",
                class
            )}
            disabled=disabled
            on:click=move |_| {
                if let Some(handler) = &on_click {
                    handler();
                }
            }
        >
            {children()}
        </button>
    }
}

#[derive(Clone, Copy)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    #[allow(dead_code)]
    Danger,
}
