#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tagmail::{
        app::*,
        AppState,
        db::DB,
        auth::Backend,
    };
    use tower_sessions::{
        //SessionStore,
        session_store::ExpiredDeletion,
        Expiry,
        SessionManagerLayer,
    };
    use tower_sessions_mongodb_store::{
        mongodb::Client, 
        MongoDBStore
    };
    use axum_login::AuthManagerLayerBuilder;
    use time::Duration;
    use dotenv::dotenv;
    dotenv().ok();

    //app setting
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    //db setting
    let conn = DB::connect().await
        .expect("db connection gone wrong");
    let appstate = AppState {
        db: DB::new(&conn).await.expect("cannot create db service"),
        leptos_options: leptos_options.clone()
    };
    
    //session setting
    let mongodb_uri = std::env::var("MONGODB_URI")
        .expect("MONGODB_URI must be set");
    let client = Client::with_uri_str(mongodb_uri).await.expect("cannot create mongodb client");
    let session_store = MongoDBStore::new(client, "tower-sessions".to_string());
    let deletion_task = tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
    );
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::weeks(1)));
    
    //authentication setting
    let backend = Backend::new(&conn).await;
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            {
                //let appstate_tmp = appstate.clone();
                move || provide_context(appstate.clone())
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            }
        )
        .layer(auth_layer)
        .fallback(leptos_axum::file_and_error_handler::<LeptosOptions, _>(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal(deletion_task.abort_handle()))
        .await
        .unwrap();
}

#[cfg(feature = "ssr")]
use tokio::{signal, task::AbortHandle};

#[cfg(feature = "ssr")]
async fn shutdown_signal(deletion_task_abort_handle: AbortHandle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c => deletion_task_abort_handle.abort(),
        _ = terminate => deletion_task_abort_handle.abort(),
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
