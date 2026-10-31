use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{FlatRoutes, Route, Router},
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta
                    name="viewport"
                    content="width=device-width, initial-scale=1"
                />

                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/bookeeping-frontend.css" />
        <Title text="Bookeeping Library" />

        <Router>
            <main class="min-h-screen bg-slate-950 p-8 text-slate-100">
                <FlatRoutes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                </FlatRoutes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <section class="mx-auto max-w-3xl rounded-lg bg-slate-900 p-6 shadow-lg">
            <h1 class="text-3xl font-bold tracking-tight">"Bookeeping Library"</h1>
            <p class="mt-4 text-slate-300">
                "This page was rendered on the server and styled with Tailwind CSS."
            </p>
        </section>
    }
}
