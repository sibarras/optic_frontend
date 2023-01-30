use crate::request;
use leptos::*;

#[component]
pub fn SaleKind(cx: Scope, id: ReadSignal<u32>) -> impl IntoView {
    let sale_kind = create_resource(cx, id, request::fetch_sale_kind);

    view! { cx,
        <Transition fallback=move || view! { cx, <div>"Loading (Suspense Fallback)..."</div>}>
            {move || {
                    sale_kind.read().map(|data| match data {
                        Err(e) => view! { cx, <pre>{format!("{e:?}")}</pre> }.into_view(cx),
                        Ok(sk) => view! { cx, <pre>{format!("{sk:?}")}</pre> }.into_view(cx),
                    })
                }
            }
        </Transition>
    }
}
