use leptos::*;
mod components;
mod model;
mod request;

use components::sale::*;

pub fn app(cx: Scope) -> impl IntoView {
    view! { cx,
        <div>
            <SalesComponent />
        </div>
    }
}
