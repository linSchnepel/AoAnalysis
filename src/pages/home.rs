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
            <p class="note">"Submit errors and request features with GitHub or by asking on Tumblr"</p>
            <ul>
            // TODO: css
                <li class="bluesky">
                    <a href="https://github.com/linSchnepel/AoAnalysis" class="resp-sharing-button__link" aria-label="AoAnalysis on GitHub">
                        <div class="resp-sharing-button resp-sharing-button--bluesky resp-sharing-button--medium">
                            <div class="resp-sharing-button__icon resp-sharing-button__icon--solid" aria-hidden="true">
                                // <!--!Font Awesome Free v7.0.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2025 Fonticons, Inc.-->
                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 640">
                                    <path d="M439.8 358.7C436.5 358.3 433.1 357.9 429.8 357.4C433.2 357.8 436.5 358.3 439.8 358.7zM320 291.1C293.9 240.4 222.9 145.9 156.9 99.3C93.6 54.6 69.5 62.3 53.6 69.5C35.3 77.8 32 105.9 32 122.4C32 138.9 41.1 258 47 277.9C66.5 343.6 136.1 365.8 200.2 358.6C203.5 358.1 206.8 357.7 210.2 357.2C206.9 357.7 203.6 358.2 200.2 358.6C106.3 372.6 22.9 406.8 132.3 528.5C252.6 653.1 297.1 501.8 320 425.1C342.9 501.8 369.2 647.6 505.6 528.5C608 425.1 533.7 372.5 439.8 358.6C436.5 358.2 433.1 357.8 429.8 357.3C433.2 357.7 436.5 358.2 439.8 358.6C503.9 365.7 573.4 343.5 593 277.9C598.9 258 608 139 608 122.4C608 105.8 604.7 77.7 586.4 69.5C570.6 62.4 546.4 54.6 483.2 99.3C417.1 145.9 346.1 240.4 320 291.1z">
                                    </path>
                                </svg>
                            </div>"AoAnalysis on GitHub"
                        </div>
                    </a>
                </li>

                <li class="tumblr">
                    <a href="https://ao3org.tumblr.com" class="resp-sharing-button__link" aria-label="ao3org on Tumblr">
                        <div class="resp-sharing-button resp-sharing-button--tumblr resp-sharing-button--medium">
                            <div class="resp-sharing-button__icon resp-sharing-button__icon--solid" aria-hidden="true">
                                // <!--!Font Awesome Free v7.0.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2025 Fonticons, Inc.-->
                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 640">
                                    <path d="M469.9 544.3C456.3 558.8 419.9 576 372.5 576C251.7 576 225.5 487.2 225.5 435.4L225.5 291.4L178 291.4C172.5 291.4 168 286.9 168 281.4L168 213.4C168 206.2 172.5 199.8 179.3 197.4C241.3 175.6 260.8 121.4 263.6 80.3C264.4 69.3 270.1 64 279.7 64L350.6 64C356.1 64 360.6 68.5 360.6 74L360.6 189.2L443.6 189.2C449.1 189.2 453.6 193.6 453.6 199.1L453.6 280.8C453.6 286.3 449.1 290.8 443.6 290.8L360.2 290.8L360.2 424C360.2 458.2 383.9 477.6 428.2 459.8C433 457.9 437.2 456.6 440.9 457.6C444.4 458.5 446.7 461 448.3 465.5L470.3 529.8C472.1 534.8 473.6 540.4 469.9 544.3z">
                                    </path>
                                </svg>
                            </div>ao3org on Tumblr
                        </div>
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
            <h3 class="heading">"Available Fandoms"</h3>
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
fn format_us_number(n: u64) -> String {
    let s = n.to_string();
    let mut chars: Vec<char> = s.chars().collect();
    let mut out = String::new();
    let mut count = 0;

    while let Some(ch) = chars.pop() {
        if count > 0 && count % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
        count += 1;
    }

    out.chars().rev().collect()
}

#[component]
pub fn HomePage() -> impl IntoView {
    use crate::state::SharedState;
    let charts = Resource::new(|| (), |_| get_homepage_charts());

    // TODO: load from state
    let fandoms = vec![
        ("Superman (2025)", "/explore?fandom=fandom1"),
        ("Clair Obscur", "/explore?fandom=fandom2"),
        ("Stranger Things", "/explore?fandom=fandom3"),
    ];

    let updates = load_updates().unwrap_or_else(|_| vec![]);

    let state = use_context::<SharedState>()
        .expect("Missing app state");

    let title = state.featured_fandom.clone().replace('-', " ");

    let total_listings = state.fandoms
        .get(&state.featured_fandom)
        .map(|fandom| fandom.stats.total_listings)
        .unwrap_or(0);
    let unique_authors = state.fandoms
        .get(&state.featured_fandom)
        .map(|fandom| fandom.stats.unique_authors)
        .unwrap_or(0);
    let total_words = state.fandoms
        .get(&state.featured_fandom)
        .map(|fandom| fandom.stats.total_words)
        .unwrap_or(0);


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
                            <h2 class="heading">{title} " - fandom overview"</h2>
                            <p class="stats">
                                <span class="count">{total_listings}</span> " works | "
                                <span class="count">{unique_authors}</span> " authors | "
                                <span class="count">{format_us_number(total_words)}</span> " words"
                            </p>
                            <p class="parent">"Go to Available Fandoms to use customizable graphs."</p>

                            <div class="account module">
                                <Suspense fallback=|| view! { <p>"Loading charts..."</p> }>
                                    {move || charts.get().map(|result| match result {
                                        Err(_) => view! { <p>"Failed to load charts."</p> }.into_any(),
                                        Ok(dto) => view! {
                                            <div class="chart-grid">
                                                <div class="chart-card">
                                                    //<h4>"Works created over time"</h4>
                                                    <LineChart data=dto.line title="Works Creation History".into() />
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