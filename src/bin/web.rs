use fib_rs::{fib, fib_range};

use leptos::mount::mount_to_body;
use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div>
            <h1>"Fibonacci Calculator"</h1>
        </div>
    }
}
