
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use ao_analysis::state::{load_app_state, SharedState};
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use ao_analysis::app::shell;

    // Set up logging.
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ao_analysis=info,warn".into()),
        )
        .init();
 
    // Load all data once before accepting any requests.
    let app_state: SharedState = load_app_state().await;
 
    // Build Leptos config from environment / Cargo.toml metadata.
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options.clone();
 
    let value = leptos_options.clone();
    let routes = generate_route_list(move || shell(value.clone()));
 
    let router = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            // Inject AppState into every Leptos server-fn context.
            {
                let app_state = app_state.clone();
                move || {
                    provide_context(app_state.clone());
                }
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler({
            let leptos_options = leptos_options.clone();
            move |_| shell(leptos_options.clone())
        }))
        .with_state(leptos_options);
 
    tracing::info!("Listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
