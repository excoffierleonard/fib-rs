use fib_rs::Fib;
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_use::{UseToggleReturn, use_toggle};

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
    let (result, set_result) = signal(Vec::<String>::new());

    view! {
        <div class="app-container">
            <h1>"Fibonacci Calculator"</h1>
            <div class="mode-toggle">
                // Use class: directive for conditional classes - more idiomatic
                <span class:toggle-active=move || value.get()>"Single"</span>
                <button class="toggle-button" on:click=move |_| toggle()>
                    <div
                        class="toggle-thumb"
                        class:toggle-thumb-left=move || value.get()
                        class:toggle-thumb-right=move || !value.get()
                    ></div>
                </button>
                <span class:toggle-active=move || !value.get()>"Range"</span>
            </div>
            <Calculator set_result=set_result is_single_mode=value />
            <div class="result-container">
                {move || {
                    result.get().into_iter().map(|line| view! { <p>{line}</p> }).collect_view()
                }}
            </div>
        </div>
    }
}

#[component]
fn Calculator(set_result: WriteSignal<Vec<String>>, is_single_mode: Signal<bool>) -> impl IntoView {
    // Single mode signal
    let (value, set_value) = signal(Ok(0u128));

    // Range mode signals
    let (start, set_start) = signal(Ok(0u128));
    let (end, set_end) = signal(Ok(0u128));

    // Combined calculate function that handles both modes using match statements
    let calculate = move |_| match is_single_mode.get() {
        true => match value.get() {
            Ok(n) => {
                let result = Fib::single(n);
                set_result.set(vec![format!("F({}) = {}", n, result)]);
            }
            Err(_) => set_result.set(vec!["Please enter a valid number".to_string()]),
        },
        false => match (start.get(), end.get()) {
            (Ok(start_val), Ok(end_val)) if start_val > end_val => {
                set_result.set(vec!["Invalid range: end < start".to_string()]);
            }
            (Ok(start_val), Ok(end_val)) => {
                let results = Fib::range(start_val, end_val);
                let formatted = results
                    .into_iter()
                    .enumerate()
                    .map(|(i, v)| format!("F({}) = {}", start_val + i as u128, v))
                    .collect();
                set_result.set(formatted);
            }
            _ => {
                set_result.set(vec!["Invalid input(s).".to_string()]);
            }
        },
    };

    view! {
        <Show
            when=move || is_single_mode.get()
            fallback=move || {
                view! {
                    <div style="display: flex; gap: 1rem;">
                        <input
                            class="number-input"
                            type="number"
                            placeholder="Start"
                            on:input:target=move |ev| {
                                set_start.set(ev.target().value().parse::<u128>())
                            }
                        />
                        <input
                            class="number-input"
                            type="number"
                            placeholder="End"
                            on:input:target=move |ev| {
                                set_end.set(ev.target().value().parse::<u128>())
                            }
                        />
                    </div>
                }
            }
        >
            <input
                class="number-input"
                type="number"
                placeholder="Enter a number"
                on:input:target=move |ev| { set_value.set(ev.target().value().parse::<u128>()) }
            />
        </Show>
        <button on:click=calculate>"Calculate"</button>
    }
}
