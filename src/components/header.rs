// Yeah the HTML and classing is messed up because everything is attuned to Ao3's original design. TODO

use leptos::prelude::*;
use leptos_router::{
    components::{A}
};

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header id="header" class="region">
            <h1 class="heading">
                <A href="/">
                    <span>"Archive Analysis"</span>
                    <img alt="Archive Analysis logo" class="logo" src=""/>
                </A>
            </h1>

            <div id="login" class="dropdown" aria-haspopup="true">
                <p class="user actions">
                    <A attr:id="login-dropdown" href="" attr:class="dropdown-toggle">
                        "check out other project"
                    </A>
                </p>
            </div>

            <nav aria-label="Site">
                <ul class="primary navigation actions">
                    <li class="dropdown" aria-haspopup="true">
                        <A href="/explore?chart=timeline" attr:class="dropdown-toggle">
                            "About"
                        </A>
                        <ul class="menu dropdown-menu">
                            <li><A href="/explore?chart=timeline">"Timeline"</A></li>
                            <li><A href="/explore?chart=composition">"Composition"</A></li>
                            <li><A href="/explore?chart=engagement">"Engagement"</A></li>
                            <li><A href="/explore?chart=compare">"Compare"</A></li>
                        </ul>
                    </li>
                </ul>
            </nav>

            <div class="clear"></div>
        </header>
    }
}