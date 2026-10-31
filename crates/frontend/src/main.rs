#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use anyhow::Context;
    use axum::Router;
    use bookeeping_frontend::app::{App, shell};
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list};

    bookeeping_core::init_logger().context("failed to initialize logging")?;

    let config = get_configuration(None).context("failed to load Leptos configuration")?;

    let addr = config.leptos_options.site_addr;
    let leptos_options = config.leptos_options;

    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();

            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .with_context(|| format!("failed to bind to {addr}"))?;

    log::info!("listening on http://{addr}");

    axum::serve(listener, app.into_make_service())
        .await
        .context("Leptos server failed")?;

    Ok(())
}

// Browser hydration starts from lib.rs, so no client-side
// main function is needed when building without SSR.
#[cfg(not(feature = "ssr"))]
fn main() {}
