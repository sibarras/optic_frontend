use leptos::*;
mod components;
mod model;
mod request;

use components::sale::*;

pub fn app(cx: Scope) -> impl IntoView {
    let (sale_id, set_sale_id) = create_signal::<u32>(cx, 1);

    view! { cx,
        <div>
            <label>
            "What Sale do You Want?"
            <input type="number"
                prop:value={move || sale_id.get().to_string()}
                on:input=move |ev| {
                    let val = event_target_value(&ev).parse::<u32>().unwrap_or(0);
                    set_sale_id(val);
                }
            />
            </label>
            <div><Sale id=sale_id/></div>
            // <FrameBrand/>
            // <SaleKind />
        </div>
    }
}
