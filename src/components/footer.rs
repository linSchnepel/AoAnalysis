use leptos::prelude::*;
use leptos_router::{
    components::{A},
};

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div id="footer" role="contentinfo" class="region">
            <h3 class="landmark heading">"Footer"</h3>
            <ul class="navigation actions">
                <li class="module group">
                    <h4 class="heading">"Development"</h4>
                    <ul class="menu">
                        <li>
                            <A href="https://github.com/your-org/your-repo">
                                "aoanalysis v0.1.0"
                            </A>
                        </li>
                    </ul>
                </li>
            </ul>
        </div>
    }
}