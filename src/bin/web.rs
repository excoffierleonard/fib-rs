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

    // Signal to hold the result of the calculation
    let (result, set_result) = signal(Vec::<String>::new());

    view! {
        <div class="app-container">
            <h1 class="app-title">"Fibonacci Calculator"</h1>
            <div class="mode-toggle">
                <div class="toggle-label" class:toggle-active=move || value.get()>
                    "Single"
                </div>
                <button 
                    class="toggle-button" 
                    aria-label="Toggle calculation mode"
                    on:click=move |_| toggle()
                >
                    <div
                        class="toggle-thumb"
                        class:toggle-thumb-left=move || value.get()
                        class:toggle-thumb-right=move || !value.get()
                    ></div>
                </button>
                <div class="toggle-label" class:toggle-active=move || !value.get()>
                    "Range"
                </div>
            </div>
            <Calculator set_result=set_result is_single_mode=value />
            <div class="result-container">
                <For each=move || result.get() key=|line| line.clone() let:line>
                    <p class="result-line">{line}</p>
                </For>
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

    // Helper function to format number for display
    let format_num = |result: Result<u128, _>| -> String {
        match result {
            Ok(0) => "".to_string(),
            Ok(n) => n.to_string(),
            Err(_) => "".to_string(),
        }
    };

    // Create memoized values for display formatting
    let value_display = Memo::new(move |_| format_num(value.get()));
    let start_display = Memo::new(move |_| format_num(start.get()));
    let end_display = Memo::new(move |_| format_num(end.get()));

    // Combined calculate function that handles both modes
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
                    <div class="range-inputs">
                        <input
                            class="number-input"
                            type="number"
                            placeholder="Start"
                            prop:value=move || start_display.get()
                            on:input:target=move |ev| {
                                set_start.set(ev.target().value().parse::<u128>())
                            }
                        />
                        <input
                            class="number-input"
                            type="number"
                            placeholder="End"
                            prop:value=move || end_display.get()
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
                prop:value=move || value_display.get()
                on:input:target=move |ev| { set_value.set(ev.target().value().parse::<u128>()) }
            />
        </Show>
        <button on:click=calculate>"Calculate"</button>
    }
}
