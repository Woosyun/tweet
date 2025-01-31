use leptos::prelude::*;
use leptos::ev::SubmitEvent;
use leptos::html::Input;

#[component]
pub fn Page() -> impl IntoView {
    let id_ref: NodeRef<Input> = NodeRef::new();
    let pw_ref: NodeRef<Input> = NodeRef::new();

    let login = Action::new(move |_| {
        let id = id_ref.get().expect("id_ref missed!").value();
        let pw = pw_ref.get().expect("pw_ref missed!").value();
        // let id = input.0.clone();
        // let pw = input.1.clone();

        async move {
            login(id, pw).await
                .map_err(|e| {
                    let _ = window().alert_with_message(&e.to_string());
                    e
                })
        }
    });
    let login = move |ev: SubmitEvent| {
        ev.prevent_default();
        login.dispatch(());
    };
    
    view! {
        <form on:submit=login>
            <label>
                "user id"
                <input type="text" node_ref=id_ref />
            </label>
            <label>
                "password"
                <input type="text" node_ref=pw_ref />
            </label>
            <input type="submit" />
        </form>
        <a href="/register">register</a>
    }
}

#[server(Login)]
async fn login(id: String, password: String) -> Result<(), ServerFnError> {
    use leptos_axum::{extract, redirect};
    use axum_login::AuthSession;
    use crate::auth::{Backend, Credentials};

    let mut auth_session: AuthSession<Backend> = extract().await?;

    // make sure user logged out
    let user = &auth_session.user;
    if !user.is_none() {
        return Err(ServerFnError::ServerError("you have to logout to login!".to_string()));
    }

    // find user from db
    let credentials= Credentials::new(id, password)
        .map_err(ServerFnError::new)?;
    let user = match auth_session.authenticate(credentials).await {
        Ok(Some(user)) =>   Ok(user),
        // Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Ok(None) => Err(ServerFnError::new("UNAUTHORIZED".to_string())),
        Err(e) => Err(ServerFnError::new(e)),
    }?;

    auth_session.login(&user).await.map_err(ServerFnError::new)?;

    redirect("/");
    Ok(())
}
