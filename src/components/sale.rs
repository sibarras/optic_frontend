use crate::{components::calculations::*, model, request};
use leptos::*;
use leptos_dom::console_log;

#[component]
pub fn SalesComponent(cx: Scope) -> impl IntoView {
    let (sales, set_sales) = create_signal::<Vec<model::Sale>>(cx, vec![]);
    // let all_sales = create_signal::<Option<model::PostSale>>(cx, None);
    let (new_sale, set_new_sale) = create_signal::<Option<model::PostSale>>(cx, None);
    let (sale_to_delete, set_sale_to_delete) = create_signal::<Option<u32>>(cx, None);
    let (sale_to_update, set_sale_to_update) = create_signal::<Option<model::Sale>>(cx, None);

    let (brands, set_brands) = create_signal::<Vec<model::FrameBrand>>(cx, vec![]);
    let (kinds, set_kinds) = create_signal::<Vec<model::SaleKind>>(cx, vec![]);

    let fetch_sales = create_resource(cx, || (), request::fetch_sales);
    let post_sale = create_resource(cx, new_sale, request::post_sale);
    let update_sale = create_resource(cx, sale_to_update, request::update_sale);
    let delete_sale = create_resource(cx, sale_to_delete, request::delete_sale);
    let fetch_frame_brands = create_resource(cx, || (), request::fetch_frame_brands);
    let fetch_sale_kinds = create_resource(cx, || (), request::fetch_sale_kinds);

    view! { cx,
        <div>
            <h3> "Total Sales Records" </h3>
            <div>
            "Add New Purchase"
            <button on:click = move |_| set_new_sale(Some(model::PostSale::default()))>
            "+"
            </button>
            </div>
            <Transition fallback=move || view! { cx, <div>"Initial Sales Read..."</div> }>
            {
                if let Some(Ok(sls)) = fetch_sales.read(cx) {
                    set_sales.set(sls);
                    console_log("Initial sales loaded.");
                } else {
                console_log("No sales to load initially.");
                };
            }
            </Transition>
            <Transition fallback = move || view! { cx, <div>"Loading Brands and Kinds..."</div> }>
            {
                if let (Some(Ok(brnds)), Some(Ok(knds))) = (fetch_frame_brands.read(cx), fetch_sale_kinds.read(cx)) {
                    set_brands(brnds);
                    set_kinds(knds);
                };
            }
            </Transition>
            <Transition fallback=move || view! { cx, <div>"Adding New Sale ..."</div> }>
            {
                if let Some(Ok(sl)) = post_sale.read(cx) {
                    set_new_sale.set(None);
                    set_sales.update(|sls| sls.push(sl));
                    console_log("New sale Added.");
                } else {
                    console_log("No sales to add...");
                };
            }
            </Transition>
            <Transition fallback = move || view! { cx, <div>"Loading Delete Sale Effect..."</div> }>
            {
                if let Some(Ok(id)) = delete_sale.read(cx) {
                    set_sale_to_delete.set_untracked(None);
                    set_sales.update(|sls| {
                        *sls = sls
                            .iter()
                            .filter(|sl| sl.id != Some(id as i64))
                            .cloned()
                            .collect::<Vec<_>>();
                    });
                } else {
                    console_log("No sale to delete...");
                };
            }
            </Transition>
            <Transition fallback = move || view! { cx, <div>"Loading Update Sale Effect..."</div> }>
            {
                if let Some(Ok(sale)) = update_sale.read(cx) {
                    console_log("Updating Sale...");
                    set_sale_to_update.set_untracked(None);
                    // set_sale_to_update(None);
                    set_sales.update(|sls| {
                        for sl in sls {
                            if sl.id == sale.id {
                                *sl = sale;
                                break
                            }
                        }
                    });
                } else {
                    console_log("No sales to Update!");
                };
            }
            </Transition>
            <SalesTable
                sales=sales
                brands=brands
                kinds=kinds
                set_delete_id=set_sale_to_delete
                set_update=set_sale_to_update
            />
            <Calculations sales=sales brands=brands kinds=kinds/>
        </div>
    }
}

#[component]
pub fn SalesTable(
    cx: Scope,
    sales: ReadSignal<Vec<model::Sale>>,
    brands: ReadSignal<Vec<model::FrameBrand>>,
    kinds: ReadSignal<Vec<model::SaleKind>>,
    set_delete_id: WriteSignal<Option<u32>>,
    set_update: WriteSignal<Option<model::Sale>>,
) -> impl IntoView {
    const TABLE_HEADERS: [&str; 6] = ["ID", "Time", "Type", "Brand", "Quantity", "Delete"];

    view! { cx,
        <table>
            <thead>
                <tr> {TABLE_HEADERS.map(|h| view! { cx, <th>{h}</th> })} </tr>
            </thead>
            <tbody>
            {
                move || {
                    let mut sls = sales();
                    sls.sort_by(|a, b| a.id.cmp(&b.id));
                    sls.into_iter().rev().map(|sale| view! { cx,
                        < EntryView
                            sale = sale
                            set_delete_id = set_delete_id
                            kinds = kinds
                            brands = brands
                            set_sale = set_update
                        />
                    })
                    .collect::<Vec<_>>()
                }
            }
            </tbody>
        </table>
    }
}

#[component]
pub fn EntryView(
    cx: Scope,
    sale: model::Sale,
    set_delete_id: WriteSignal<Option<u32>>,
    kinds: ReadSignal<Vec<model::SaleKind>>,
    brands: ReadSignal<Vec<model::FrameBrand>>,
    set_sale: WriteSignal<Option<model::Sale>>,
) -> impl IntoView {
    // let selected_brand = create_rw_signal(cx, sale.frame_brand_id.unwrap() as u32);
    // let selected_kind = create_rw_signal(cx, sale.sale_kind_id as u32);
    // let selected_quantity = create_rw_signal(cx, sale.quantity as u32);

    let on_quantity_input = move |ev| {
        set_sale(Some(model::Sale {
            quantity: event_target_value(&ev).parse::<i32>().unwrap_or(0),
            ..sale
        }));
    };

    let on_salekind_input = move |ev| {
        set_sale(Some(model::Sale {
            sale_kind_id: event_target_value(&ev).parse::<i32>().unwrap_or(1),
            ..sale
        }));
    };

    let on_framebrand_input = move |ev| {
        set_sale(Some(model::Sale {
            frame_brand_id: event_target_value(&ev).parse::<i32>().ok(),
            ..sale
        }));
    };

    view! { cx,
        <tr>
        <td> {sale.id} </td>
        <td>
        {
            format!(
                "{}",
                chrono::DateTime::<chrono::Utc>::from(sale.date.unwrap())
                .format("%d/%m/%y %I:%M:%S %p")
            )
        }
        </td>
        <td>
            <select
                prop:value = sale.sale_kind_id
                on:input = on_salekind_input
            >
            { move ||
                kinds()
                .into_iter()
                .map(|k| view! { cx,
                    <option
                        prop:value={k.id.unwrap()}
                        prop:selected={k.id == Some(sale.sale_kind_id)}
                    > {k.name}
                    </option>
                })
                .collect::<Vec<_>>()
            }
            </select>
        </td>
        <td>
            <select
                prop:value = sale.frame_brand_id
                on:input = on_framebrand_input
            >
            { move ||
                brands()
                .into_iter()
                .map(|k| view! { cx,
                    <option
                        prop:value={k.id.unwrap()}
                        prop:selected={k.id == sale.frame_brand_id}
                    > {k.name}
                    </option>
                })
                .collect::<Vec<_>>()
            }
            </select>
        </td>
        <td>
            <input type="number"
                prop:value = sale.quantity
                on:input = on_quantity_input
            />
        </td>
        <td>
            <button on:click = move |_| set_delete_id(sale.id.map(|n| n as u32))>
            "X"
            </button>
        </td>
        </tr>
    }
}
