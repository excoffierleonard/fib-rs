use fib_rs::Fib;
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_use::use_toggle;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let toggle_ret = use_toggle(true);
    let toggle = toggle_ret.toggle;
    let is_range = toggle_ret.value;
    // Shared state for the result
    let (result, set_result) = signal(String::new());

    let calculator = move || {
        let mode = is_range.get();
        if mode {
            view! { <Range set_result=set_result /> }.into_any()
        } else {
            view! { <Single set_result=set_result /> }.into_any()
        }
    };

    view! {
        <div class="app-container">
            <h1>"Fibonacci Calculator"</h1>
            <div class="mode-toggle">
                <span class=move || {
                    if !is_range.get() { "toggle-active" } else { "" }
                }>"Single"</span>
                <button class="toggle-button" on:click=move |_| toggle()>
                    <div class=move || {
                        if is_range.get() {
                            "toggle-thumb toggle-thumb-right"
                        } else {
                            "toggle-thumb toggle-thumb-left"
                        }
                    }></div>
                </button>
                <span class=move || if is_range.get() { "toggle-active" } else { "" }>"Range"</span>
            </div>
            <div>{calculator}</div>
            // Only show result here:
            <p style="overflow-wrap: break-word;">{result}</p>
        </div>
    }
}

#[component]
fn Single(set_result: WriteSignal<String>) -> impl IntoView {
    let (value, set_value) = signal(Ok(0u128));
    let calculate = move |_| match value.get() {
        Ok(n) => set_result.set(format!("F({}) = {}", n, Fib::single(n))),
        Err(_) => set_result.set("Please enter a valid number".to_string()),
    };

    view! {
        <input
            class="number-input"
            type="number"
            placeholder="Enter a number"
            on:input:target=move |ev| { set_value.set(ev.target().value().parse::<u128>()) }
        />
        <button on:click=calculate>"Calculate"</button>
    }
}

#[component]
fn Range(set_result: WriteSignal<String>) -> impl IntoView {
    let (start, set_start) = signal(0u128);
    let (end, set_end) = signal(0u128);

    let calculate = move |_| {
        if start.get() > end.get() {
            set_result.set("Invalid range: end < start".to_string());
        } else {
            let results = Fib::range(start.get(), end.get());
            set_result.set(format!("{:?}", results));
        }
    };

    view! {
        <div style="display: flex; gap: 1rem;">
            <input
                class="number-input"
                type="number"
                placeholder="Start"
                on:input:target=move |ev| {
                    set_start.set(ev.target().value().parse::<u128>().unwrap_or(0))
                }
            />
            <input
                class="number-input"
                type="number"
                placeholder="End"
                on:input:target=move |ev| {
                    set_end.set(ev.target().value().parse::<u128>().unwrap_or(0))
                }
            />
        </div>
        <button on:click=calculate>"Calculate"</button>
    }
}
