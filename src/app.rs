use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::pages::{ExplorePage, HomePage};

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
                    <Route path=StaticSegment("explore") view=ExplorePage/>
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