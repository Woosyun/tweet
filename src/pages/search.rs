use leptos::prelude::*;
use leptos::html::Input;
use crate::mail;
use leptos_router::{
    hooks::*,
    params::ParamsMap,
    components::A
};

#[component] 
pub fn Page() -> impl IntoView {
    view! {
        <div class="flex-col gap-4 w-4/5 m-auto">
            <TopBar />
            <TagBar />
            <MailEditor />
            <SearchResultViewer />
        </div>
    }
}


pub fn get_tags_from_query(q: &ParamsMap) -> Vec<String> {
    q.get_all("tags")
        .map(|tags| {
            tags
                .into_iter()
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default()
}

#[component] 
pub fn TopBar() -> impl IntoView {
    let query = use_query_map();
    let input_ref: NodeRef<Input> = NodeRef::new();

    view! {
        <div class="flex justify-between gap-4">
            <h1 class="btn-gray-ish">Home</h1>

            <form class="flex gap-2" on:submit=move |ev| {
                ev.prevent_default();

                let input = input_ref.get().expect("missing input_ref").value();
        
                let mut query = query.get();
                if get_tags_from_query(&query).contains(&input){
                    window().alert_with_message("input is already in query").unwrap();
                    return;
                }
        
                query.insert("tags", input);
                let query = query.to_query_string();
        
                window().location().set_search(&query).unwrap();
            }>
                <input type="search" node_ref=input_ref class="input-gray-ish"/>
                <input type="submit" value="search" class="btn-gray-ish"/>
            </form>
            
            <AuthButton />
        </div>
    }
}

#[component]
pub fn AuthButton() -> impl IntoView {
    let authenticated = Resource::new(|| (), |_| async move {
        authenticate().await
    });

    let logout = Action::new(|_| {
        async move {
            match logout().await {
                Ok(_) => window().location().reload().unwrap(),
                Err(e) => window().alert_with_message(&e.to_string()).unwrap(),
            }
        }
    });
    
    view! {
        <div 
            class="btn-gray-ish"
        >
            <Suspense fallback=move || view! {<span>"..."</span>}>
            <ErrorBoundary fallback=move |_err| view! {
                <A href="/login">login</A>
            }>
                {move || authenticated
                    .get().map(|re| {
                        re.map(|_| view! {
                            <button 
                                on:click=move |_| { logout.dispatch(()); }
                            >
                                logout
                            </button>
                        })
                    })}
            </ErrorBoundary>
            </Suspense>
        </div>
    }
}

#[server(Authenticate)]
pub async fn authenticate() -> Result<(), ServerFnError> {
    use leptos_axum::extract;
    use axum_login::AuthSession;
    use crate::auth::Backend;

    let auth_session: AuthSession<Backend> = extract().await?;
    
    match auth_session.user {
        Some(_) => {
            Ok(())
        },
        None => Err(ServerFnError::new("UNAUTHORIZED")),
    }
}

#[server(Logout)] 
pub async fn logout() -> Result<(), ServerFnError> {
    use axum_login::AuthSession;
    use leptos_axum::extract;
    use crate::auth::Backend;

    let mut auth_session: AuthSession<Backend> = extract().await?;

    let _user = auth_session
        .logout().await
        .map_err(ServerFnError::new)?;
    
    Ok(())
}

#[component] 
pub fn TagBar() -> impl IntoView {
    let query = use_query_map();
    let tags = move || get_tags_from_query(&query.get());
    let delete_tag = move |tag: String| {
        if !tags().contains(&tag) {
            return;
        }

        let mut new_tags = tags();
        new_tags.retain(|t| t != &tag);
        let mut new_query = query
            .get();

        new_query.remove("tags");
        for tag in new_tags {
            new_query.insert("tags", tag);
        }
        let new_query = new_query.to_query_string();
        
        window().location().set_search(&new_query).unwrap();
    };

    view! {
        <div class="flex gap-2 justify-center">
            <For each=tags key=|tag| tag.clone() children=move |tag: String| {
                let tag0 = tag.clone();
                view! {
                    <span class="hover:text-red-400 hover:cursor-pointer" on:click=move |ev| {
                        ev.prevent_default();
                        delete_tag(tag0.clone());
                    }>{tag}</span>
                }
            } />
        </div>
    }
}

#[component]
pub fn MailEditor() -> impl IntoView {
    let query = use_query_map();
    let (text, set_text) = signal(String::new());

    let mail_action = Action::new(move |_| {
        async move {
            let content = text.get();
            let tags = get_tags_from_query(&query.get());
            match create_mail(content, Some(tags)).await {
                Ok(_) => window().location().reload().unwrap(),
                Err(error) => {
                    window().alert_with_message(&error.to_string()).unwrap();
                }
            }
        }
    });

    view! {
        <form 
            on:submit=move |ev| {
                ev.prevent_default();
                mail_action.dispatch(());
            }
        >
            <input type="text" bind:value=(text, set_text) class="input-gray-ish"/>
            <input type="submit" value="upload" class="btn-gray-ish"/>
        </form>
    }
}

#[server]
async fn create_mail(content: String, tags: Option<Vec<String>>) -> Result<(), ServerFnError> {
    use crate::AppState;
    use leptos::prelude::use_context;
    use axum_login::AuthSession;
    use leptos_axum::extract;
    use crate::auth::Backend;

    let auth_session: AuthSession<Backend> = extract().await?;
    let user_id = match auth_session.user {
        Some(user) => user.user_id,
        None => return Err(ServerFnError::new("Unauthorized"))
    };

    let tags = tags
        .unwrap_or_default();
    
    use_context::<AppState>()
        .expect("cannot extract appstate")
        .db.mail_service
        .insert_one(mail::Mail::new(user_id, content, tags))
        .await.map_err(ServerFnError::new)
        .map(|_| ())
}

#[component] 
pub fn SearchResultViewer() -> impl IntoView {
    let query = use_query_map();
    let items = Resource::new(query, |query| async move {
        let tags = get_tags_from_query(&query);
        
        search(Some(tags)).await
    });
    
    view! {
        <Transition fallback=move || view! { <p>"searching commits..."</p>}>
        <ErrorBoundary fallback=move |_| view! {<h1>"error while searching"</h1>}>
        {move || {
            items.get().map(|re| {
                re.map(|items| {
                    items.into_iter().map(IntoRender::into_render).collect_view()
                })
            })
        }}
        </ErrorBoundary>
        </Transition>
    }
}

#[server]
pub async fn search(tags: Option<Vec<String>>) -> Result<Vec<mail::Mail>, ServerFnError> {
    use crate::AppState;
    use leptos::prelude::use_context;

    let tags = tags
        .unwrap_or_default();
    
    let search_service = use_context::<AppState>()
        .unwrap()
        .db.mail_service;
    
    search_service.find_by_tags(tags).await
        .map_err(ServerFnError::new)
}


