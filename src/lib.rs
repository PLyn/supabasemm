mod home;
pub mod routes;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use routes::App;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
