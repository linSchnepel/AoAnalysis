// TODO: Recheck privacy
use leptos::prelude::*;
use leptos_router::{
    components::{A},
};

#[derive(serde::Deserialize, Clone)]
pub struct NewsItemData {
    title: String,
    href: String,
    date: String,
    comments: Option<String>,
    text: String,
}

// TODO: load_updates should become a #[server] fn and NewsSection should use a Resource, same as charts.
pub fn load_updates() -> Result<Vec<NewsItemData>, String> {
    let raw = std::fs::read_to_string("assets/files/updates.json")
        .map_err(|e| e.to_string())?;
    serde_json::from_str(&raw).map_err(|e| e.to_string())
}

#[component]
pub fn NewsSection(updates: Vec<NewsItemData>) -> impl IntoView {
    view! {
        <div class="latest news module odd">
            <h3 class="heading">
                <span class="title">"Updates"</span>
                <span class="link"><A href="/explore?chart=timeline">"All Updates"</A></span>
            </h3>

            <ul class="news index group">
                {updates.into_iter().map(|item| view! {
                    <NewsItem item=item />
                }).collect_view()}
            </ul>
        </div>
    }
}

#[component]
fn NewsItem(item: NewsItemData) -> impl IntoView {
    view! {
        <li class="post group">
            <div class="header module">
                <h4 class="heading">
                    <A href=item.href.to_string()>{item.title}</A>
                </h4>

                <p class="meta">
                    <span class="published">
                        "Published: "
                        <span class="date">{item.date}</span>
                    </span>
                    {item.comments.map(|c| view! {
                        <span class="comments">
                            "Comments: "
                            <A href=item.href.to_string()>{c}</A>
                        </span>
                    })}
                </p>
            </div>

            <blockquote class="userstuff">
                <p>{item.text}</p>
            </blockquote>
        </li>
    }
}