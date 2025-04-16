use fib_rs::fib;

use leptos::mount::mount_to_body;
use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (value, set_value) = signal(Ok(0u128));
    let (result, set_result) = signal(String::new());

    let calculate = move |_| match value.get() {
        Ok(n) => set_result.set(format!("F({}) = {}", n, fib(n))),
        Err(_) => set_result.set("Please enter a valid number".to_string()),
    };

    view! {
        <div>
            <h1>"Fibonacci Calculator"</h1>
            <input
                type="number"
                placeholder="Enter a number"
                on:input:target=move |ev| { set_value.set(ev.target().value().parse::<u128>()) }
            />
            <button on:click=calculate>"Calculate"</button>
            <p style="white-space: pre-wrap; word-wrap: break-word;">{result}</p>
        </div>
    }
}
