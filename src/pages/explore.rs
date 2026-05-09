use leptos::prelude::*;
use leptos_router::{
    components::{A},
};
use leptos_router::hooks::use_query_map;

// ---------------------------------------------------------------------------
// Home page component
// ---------------------------------------------------------------------------


#[component]
pub fn ExplorePage() -> impl IntoView {
    let query = use_query_map();

    let chart_type = move || {
        query
            .get()
            .get("chart")
            .unwrap_or_else(|| "timeline".to_string())
    };

    view! {
        <div class="page explore-page">
            <header class="topbar">
                <div class="brand">
                    <A href="/">"AOAnalysis"</A>
                </div>
                <nav class="topnav">
                    <A href="/explore?chart=timeline">"Timeline"</A>
                    <A href="/explore?chart=composition">"Composition"</A>
                    <A href="/explore?chart=engagement">"Engagement"</A>
                    <A href="/explore?chart=compare">"Compare"</A>
                </nav>
            </header>

            <div class="explore-grid">
                <section class="chart-shell">
                    <div class="chart-header">
                        <h1>"Explore"</h1>
                        <select class="chart-select">
                            <option value="timeline" selected=move || chart_type() == "timeline">"Timeline"</option>
                            <option value="composition" selected=move || chart_type() == "composition">"Composition"</option>
                            <option value="engagement" selected=move || chart_type() == "engagement">"Engagement"</option>
                            <option value="compare" selected=move || chart_type() == "compare">"Compare"</option>
                        </select>
                    </div>

                    <div class="chart-card">
                        <h2>{move || chart_type()}</h2>
                        <div class="chart-placeholder">
                            "Chart renders here"
                        </div>
                    </div>
                </section>

                <aside class="filter-panel">
                    <h3>"Filters"</h3>

                    <label>
                        <span>"Date range"</span>
                        <input type="date"/>
                    </label>

                    <label>
                        <span>"Dataset"</span>
                        <select>
                            <option>"All works"</option>
                            <option>"Created in 2024"</option>
                            <option>"Updated in 2024"</option>
                        </select>
                    </label>

                    <label>
                        <span>"Fandom"</span>
                        <input type="text" placeholder="Filter fandom"/>
                    </label>

                    <label>
                        <span>"Metric"</span>
                        <select>
                            <option>"Count"</option>
                            <option>"Percent"</option>
                            <option>"Per work"</option>
                        </select>
                    </label>

                    <button class="apply-btn">"Apply"</button>
                </aside>
            </div>
        </div>
    }
}