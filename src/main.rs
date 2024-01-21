mod app;
mod components;
mod pages;
mod types;

fn main() {
    #[cfg(feature = "logging")]
    {
        console_error_panic_hook::set_once();
        let _ = console_log::init_with_level(log::Level::Debug);
    }
    leptos::mount_to_body(app::App)
}
