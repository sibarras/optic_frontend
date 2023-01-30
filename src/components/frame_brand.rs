use crate::request;
use leptos::*;

#[component]
pub fn FrameBrand(cx: Scope, id: ReadSignal<u32>) -> impl IntoView {
    let frame_brand = create_resource(cx, id, request::fetch_frame_brand);

    view! { cx,
        <Transition fallback=move || view! { cx, <div>"Loading (Suspense Fallback)..."</div>}>
            {move || {
                    frame_brand.read().map(|data| match data {
                        Err(e) => view! { cx, <pre>{format!("{e:?}")}</pre> }.into_view(cx),
                        Ok(fb) => view! { cx, <pre>{format!("{fb:?}")}</pre> }.into_view(cx),
                    })
                }
            }
        </Transition>
    }
}
