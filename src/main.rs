mod base_window;
mod log_window;

use leptos::*;
use log_window::LogWindow;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <LogWindow/>
        }
    })
}
