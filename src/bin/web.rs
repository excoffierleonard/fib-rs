use fib_rs::Fib;

use leptos::mount::mount_to_body;
use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="app-container">
            <h1>"Fibonacci Calculator"</h1>
            <Range />
        </div>
    }
}

#[component]
fn Single() -> impl IntoView {
    let (value, set_value) = signal(Ok(0u128));
    let (result, set_result) = signal(String::new());

    let calculate = move |_| match value.get() {
        Ok(n) => set_result.set(format!("F({}) = {}", n, Fib::single(n))),
        Err(_) => set_result.set("Please enter a valid number".to_string()),
    };

    view! {
        <input
            type="number"
            placeholder="Enter a number"
            on:input:target=move |ev| { set_value.set(ev.target().value().parse::<u128>()) }
        />
        <button on:click=calculate>"Calculate"</button>
        <p style="overflow-wrap: break-word;">{result}</p>
    }
}

#[component]
fn Range() -> impl IntoView {
    let (start, set_start) = signal(0u128);
    let (end, set_end) = signal(0u128);
    let (result, set_result) = signal(String::new());

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
                type="number"
                placeholder="Start"
                on:input:target=move |ev| {
                    set_start.set(ev.target().value().parse::<u128>().unwrap_or(0))
                }
            />
            <input
                type="number"
                placeholder="End"
                on:input:target=move |ev| {
                    set_end.set(ev.target().value().parse::<u128>().unwrap_or(0))
                }
            />
        </div>
        <button on:click=calculate>"Calculate Range"</button>
        <p style="overflow-wrap: break-word;">{result}</p>
    }
}
