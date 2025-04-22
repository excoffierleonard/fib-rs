use fib_rs::Fib;
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_use::{use_toggle, UseToggleReturn};

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    // Toggle state for the calculator mode (single or range)
    let UseToggleReturn {
        toggle,
        value,
        set_value: _,
    } = use_toggle(true);

    // Shared state for the result
    let (result, set_result) = signal(String::new());

    // The calculator component that will be displayed based on the toggle state
    let calculator = move || match value.get() {
        false => view! { <Range set_result=set_result /> }.into_any(),
        true => view! { <Single set_result=set_result /> }.into_any(),
    };

    // Classes for the toggle button
    let single_class = move || match value.get() {
        true => "toggle-active",
        false => "",
    };
    let range_class = move || match value.get() {
        false => "toggle-active",
        true => "",
    };
    let thumb_class = move || match value.get() {
        false => "toggle-thumb toggle-thumb-right",
        true => "toggle-thumb toggle-thumb-left",
    };

    view! {
        <div class="app-container">
            <h1>"Fibonacci Calculator"</h1>
            <div class="mode-toggle">
                <span class=single_class>"Single"</span>
                <button class="toggle-button" on:click=move |_| toggle()>
                    <div class=thumb_class></div>
                </button>
                <span class=range_class>"Range"</span>
            </div>
            <div>{calculator}</div>
            <p class="result-container">{result}</p>
        </div>
    }
}

#[component]
fn Single(set_result: WriteSignal<String>) -> impl IntoView {
    let (value, set_value) = signal(Ok(0u128));

    let calculate = move |_| match value.get() {
        Ok(n) => set_result.set({
            let result = Fib::single(n);
            format!("F({}) = {}", n, result)
        }),
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
    let (start, set_start) = signal(Ok(0u128));
    let (end, set_end) = signal(Ok(0u128));

    let calculate = move |_| {
        if let (Ok(start), Ok(end)) = (start.get(), end.get()) {
            if start > end {
                set_result.set("Invalid range: end < start".to_string());
            } else {
                let results = Fib::range(start, end);
                set_result.set(format!("{:?}", results));
            }
        } else {
            set_result.set("Invalid input(s).".to_string());
        }
    };

    view! {
        <div style="display: flex; gap: 1rem;">
            <input
                class="number-input"
                type="number"
                placeholder="Start"
                on:input:target=move |ev| { set_start.set(ev.target().value().parse::<u128>()) }
            />
            <input
                class="number-input"
                type="number"
                placeholder="End"
                on:input:target=move |ev| { set_end.set(ev.target().value().parse::<u128>()) }
            />
        </div>
        <button on:click=calculate>"Calculate"</button>
    }
}
