use leptos::prelude::*;
use leptos::html::Input;
use leptos::ev::SubmitEvent;

#[component]
pub fn Page() -> impl IntoView {
    let id_ref: NodeRef<Input> = NodeRef::new();
    let pw_ref: NodeRef<Input> = NodeRef::new();
    let name_ref: NodeRef<Input> = NodeRef::new();

    let register = Action::new(move |_| {
        let id = id_ref.get().expect("missing id_ref").value();
        let pw = pw_ref.get().expect("missing pw_ref").value();
        let name = name_ref.get().expect("missing name_ref").value();

        async move {
            register(id, pw, name).await
                .map_err(|e| {
                    let _ = window().alert_with_message(&e.to_string());
                    e
                })
        }
    });
    let register = move |ev: SubmitEvent| {
        ev.prevent_default();
        register.dispatch(());
    };
    
    view! {
        <div class="w-svw h-svh flex flex-col gap-4 justify-center items-center">
            <form class="flex flex-col gap-4" on:submit=register>
                <label class="flex flex-col gap-2">
                    "user id"
                    <input type="text" node_ref=id_ref class="input-gray-ish"/>
                </label>
                <label class="flex flex-col gap-2">
                    "password"
                    <input type="text" node_ref=pw_ref class="input-gray-ish" />
                </label>
                <label class="flex flex-col gap-2">
                    "user name"
                    <input type="text" node_ref=name_ref class="input-gray-ish" />
                </label>
                <input type="submit" value="register" class="btn-gray-ish" />
            </form>
        </div>
    }
}

#[server(Register)]
async fn register(user_id: String, password: String, user_name: String) -> Result<(), ServerFnError> {
    use axum_login::AuthSession;
    use leptos_axum::{extract, redirect};
    use crate::auth::Backend;
    use crate::user::User;

    let user = User::new(user_id, password, user_name);
    
    let auth_session: AuthSession<Backend> = extract().await?;
    let backend = auth_session.backend;
    backend.register(user).await
        .map_err(ServerFnError::new)?;
    
    redirect("/");
    Ok(())
}
