use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/ao-analysis.css"/>

        <Title text="AOAnalysis"/>

        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

// ---------------------------------------------------------------------------
// Server function — fetch dataset summary
// ---------------------------------------------------------------------------
 
/// Summary data sent from server to client.
/// Small and serializable. No raw Listing data crosses the wire.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DatasetSummary {
    pub fandom_counts: Vec<(String, usize)>, // (fandom_name, listing_count)
    pub total: usize,
}
 
#[server]
pub async fn get_dataset_summary() -> Result<DatasetSummary, ServerFnError> {
    use crate::state::SharedState;
 
    let state = expect_context::<SharedState>();
 
    let mut fandom_counts: Vec<(String, usize)> = state
        .fandoms
        .iter()
        .map(|(name, data)| (name.clone(), data.listing_count()))
        .collect();

    fandom_counts.sort_by(|a, b| a.0.cmp(&b.0));
 
    let total = fandom_counts.iter().map(|(_, c)| c).sum();
 
    Ok(DatasetSummary {
        fandom_counts,
        total,
    })
}

// ---------------------------------------------------------------------------
// Home page component
// ---------------------------------------------------------------------------
 
#[component]
fn HomePage() -> impl IntoView {
    let summary = Resource::new(|| (), |_| get_dataset_summary());
 
    view! {
        <div class="home">
            <h1>"AOAnalysis"</h1>
            <Suspense fallback=|| view! { <p class="loading">"Loading data..."</p> }>
                {move || {
                    summary.get().map(|result| match result {
                        Err(e) => view! {
                            <p class="error">"Error loading data: " {e.to_string()}</p>
                        }.into_any(),
                        Ok(data) => view! {
                            <section class="summary">
                                <h2>"Dataset Summary"</h2>
                                <p class="total">
                                    "Total listings loaded: "
                                    <strong>{data.total}</strong>
                                </p>
                                <ul class="fandom-list">
                                    {data.fandom_counts.into_iter().map(|(name, count)| view! {
                                        <li>
                                            <span class="fandom-name">{name}</span>
                                            " — "
                                            <span class="fandom-count">{count}</span>
                                            " listings"
                                        </li>
                                    }).collect::<Vec<_>>()}
                                </ul>
                            </section>
                        }.into_any(),
                    })
                }}
            </Suspense>
        </div>
    }
}
