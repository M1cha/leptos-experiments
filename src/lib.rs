use gloo_timers::callback::Timeout;
use leptos::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (show, set_show) = create_signal(cx, false);
    let ref_input = create_node_ref::<html::Input>(cx);

    let on_show = move || {
        if !show.get() {
            return;
        }

        if let Some(ref_input) = ref_input.get() {
            log::info!("FOCUS NOW");
            ref_input.focus().unwrap();
        }
    };

    // TODO: we need an event that runs a callback after inserting the DOM node
    create_effect(cx, move |_| {
        if show.get() {
            let timeout = Timeout::new(1_000, move || {
                on_show();
            });
            timeout.forget();
        }
    });

    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=move |_| set_show.update(|b| *b = !*b)>"Toggle"</button>

        <Show
            when=move || show.get()
            fallback=|_| ()
        >
            <input
              _ref=ref_input
              id="main_input"
              type="text"
              placeholder="Placeholder"
            />
        </Show>
    }
}
