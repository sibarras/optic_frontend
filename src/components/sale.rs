use crate::components::{frame_brand::*, sale_kind::*};
use crate::request;
use leptos::*;

#[component]
pub fn Sale(cx: Scope, id: ReadSignal<u32>) -> impl IntoView {
    let sale = create_resource(cx, id, request::fetch_sale);
    let (frame_brand, set_frame_brand) = create_signal::<u32>(cx, 1);
    let (sale_kind, set_sale_kind) = create_signal::<u32>(cx, 1);

    view! { cx,
        <Transition fallback=move || view! { cx, <div>"Loading (Suspense Fallback)..."</div> }>
            {
                move || sale.read().map(|data| match data {
                    Ok(sale) => {
                        set_frame_brand(sale.frame_brand_id.unwrap_or(1) as u32);
                        set_sale_kind(sale.sale_kind_id as u32);
                        view! { cx,
                            <pre>{format!("{sale:?}")}</pre>
                            <FrameBrand id=frame_brand/>
                            <SaleKind id=sale_kind/>
                        }.into_view(cx)
                    },
                    Err(e) => view! { cx, <pre>{format!("{e:?}")}</pre> }.into_view(cx),
                })
            }
        </Transition>
    }
}
