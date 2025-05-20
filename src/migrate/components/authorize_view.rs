use leptos::prelude::*;

#[component]
pub fn AuthorizeForm() -> impl IntoView {
    view!(
        <br />
        <button on:click=move |_| { window().location().set_href("/connect-supabase/login").unwrap(); }>
            Sign in with Supabase
        </button>
        <br />
    )
}
