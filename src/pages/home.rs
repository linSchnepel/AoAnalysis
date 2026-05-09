use leptos::prelude::*;
use leptos_meta::{Title};
use leptos_router::{
    components::{A},
};

use crate::components::{Footer, Header, NewsSection, DashboardCard, load_updates};

use crate::server_fns::charts::{HomepageChartDto, get_homepage_charts};
use crate::components::charts::{
    bar::BarChart, line::LineChart, pie::PieChart, percentile::PercentileChart,
};

#[component]
fn SocialSection() -> impl IntoView {
    view! {
        <div class="social module">
            <h3 class="heading">"Follow us"</h3>
            <p class="note">"github link or whatever"</p>
            <ul>
                <li class="bluesky">
                    <a href="https://github.com/your-org/your-repo" class="resp-sharing-button__link">
                        "Project repo"
                    </a>
                </li>
            </ul>
        </div>
    }
}

#[component]
fn BrowseList(fandoms: Vec<(&'static str, &'static str)>) -> impl IntoView {
    view! {
        <div role="navigation" aria-label="Media" class="browse module">
            <h3 class="heading">"possible fandoms"</h3>
            <ul>
                {fandoms.into_iter().map(|(name, href)| view! {
                    <li><A href=href.to_string()>{name}</A></li>
                }).collect_view()}
            </ul>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Home page component
// ---------------------------------------------------------------------------

#[component]
pub fn HomePage() -> impl IntoView {
    let charts = Resource::new(|| (), |_| get_homepage_charts());

    let fandoms = vec![
        ("fandom1", "/explore?fandom=fandom1"),
        ("fandom2", "/explore?fandom=fandom2"),
        ("fandom3", "/explore?fandom=fandom3"),
    ];

    let updates = load_updates().unwrap_or_else(|_| vec![]);

    view! {
        <Title text="Archive Analysis"/>

        <div id="outer" class="wrapper">
            <ul id="skiplinks">
                <li><a href="#main">"Main Content"</a></li>
            </ul>

            <Header />

            <div id="inner" class="wrapper">
                <div id="main" class="system docs index region" role="main">
                    <div class="flash"></div>

                    <div class="splash">
                        <div class="intro module odd">
                            <h2 class="heading">"this is data"</h2>
                            <p class="stats">
                                "dataset size "
                                <span class="count">"77,920"</span>
                                " listings | "
                                <span class="count">"10,730,000"</span>
                                " english | "
                                <span class="count">"17,470,000"</span>
                                " etc"
                            </p>
                            <p class="parent">"some other info/disclaimer."</p>

                            <div class="account module">
                                <Suspense fallback=|| view! { <p>"Loading charts..."</p> }>
                                    {move || charts.get().map(|result| match result {
                                        Err(_) => view! { <p>"Failed to load charts."</p> }.into_any(),
                                        Ok(dto) => view! {
                                            <div class="chart-grid">
                                                <div class="chart-card">
                                                    <h4>"Works Over Time"</h4>
                                                    <LineChart data=dto.line title="Works created over time".into() />
                                                </div>
                                                <div class="chart-card">
                                                    <h4>"Completion"</h4>
                                                    <PieChart data=dto.pie title="Completion breakdown".into() />
                                                </div>
                                                <div class="chart-card">
                                                    <h4>"Categories"</h4>
                                                    <BarChart data=dto.bar title="Category breakdown".into() />
                                                </div>
                                                <div class="chart-card">
                                                    <h4>"Distribution"</h4>
                                                    <PercentileChart data=dto.percentile title="Stat distribution".into() />
                                                </div>
                                            </div>
                                        }.into_any(),
                                    })}
                                </Suspense>
                            </div>
                            
                        </div>

                        <BrowseList fandoms=fandoms />

                        <NewsSection updates=updates />

                        <SocialSection />
                    </div>

                    <div class="clear"></div>
                </div>
            </div>

            <Footer />
        </div>
    }
}