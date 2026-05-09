use leptos::prelude::*;
use leptos_router::{
    components::{A},
};

#[component]
pub fn DashboardCard(heading: String, body: String, href: String, odd: bool) -> impl IntoView {
    view! {
        <div class=move || if odd { "module odd account" } else { "module account" }>
            <h4 class="heading">{heading}</h4>
            <p>{body}</p>
            <p class="actions"><A href=href>"edit values"</A></p>
        </div>
    }
}