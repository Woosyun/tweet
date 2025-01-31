use leptos::prelude::*;

#[component]
pub fn NodeEditor() -> impl IntoView {
    let input_ref = NodeRef::new();

    view! {
        <form on:submit=move |ev| {
            ev.prevent_default();
            let input = input_ref
                .get().expect("node_ref cannot be missing");
            leptos::logging::log!("input: {}", input);
        }>
            <input type="text" />
            <input type="submit" value"submit" />
        </form>
    }
}
