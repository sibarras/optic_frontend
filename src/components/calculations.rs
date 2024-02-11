use crate::model;
use leptos::*;
// use leptos_dom::console_log;
use std::collections::hash_map::HashMap;

#[component]
pub fn Calculations(
    cx: Scope,
    sales: ReadSignal<Vec<model::Sale>>,
    brands: ReadSignal<Vec<model::FrameBrand>>,
    kinds: ReadSignal<Vec<model::SaleKind>>,
) -> impl IntoView {
    let get_brand_name = move |id| {
        brands
            .get()
            .iter()
            .find(|s| s.id == id)
            .unwrap()
            .name
            .clone()
    };
    let get_kind_name = move |id| {
        kinds
            .get()
            .iter()
            .find(|s| s.id == id)
            .unwrap()
            .name
            .clone()
    };

    // console_log(&format!("{:?}{:?}", get_brand_name, get_kind_name));

    let total_sales = move || {
        sales
            .get()
            .iter()
            .filter(|s| s.sale_kind_id != 1)
            .map(|s| s.quantity)
            .sum::<i32>()
    };

    let total_attended = move || sales.get().iter().map(|s| s.quantity).sum::<i32>();

    let aggr_results = move || {
        let mut sk_hmp: HashMap<String, u32> = HashMap::new();
        let mut fb_hmp: HashMap<String, u32> = HashMap::new();

        for sl in sales.get() {
            let sk_name = get_kind_name(Some(sl.sale_kind_id));
            sk_hmp.entry(sk_name).and_modify(|v| *v += 1).or_insert(1);

            let fb_name = get_brand_name(sl.frame_brand_id);
            fb_hmp.entry(fb_name).and_modify(|v| *v += 1).or_insert(1);
        }
        view! {cx,
            <div>
                <p>"Total Sale Kinds: " {format!("{:?}", sk_hmp)}</p>
                <p>"Total Frame Brands: " {format!("{:?}", fb_hmp)}</p>
            </div>
        }
    };

    view! {cx,
        <div>
            <p>"Total Sales: " {total_sales}</p>
            <p>"Total Attended: " {total_attended}</p>
            <div>"Aggregated Calculations: " {aggr_results}</div>
        </div>
    }
}
