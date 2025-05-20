use leptos::html::Input;
use leptos::prelude::*;

#[component]
pub fn AuthorizeForm(
    input_ref: NodeRef<Input>,
    auth_status: ReadSignal<String>,
    set_auth_status: WriteSignal<String>,
) -> impl IntoView {
    view!(
        <br />
        <button on:click=move |_| { window().location().set_href("/connect-supabase/login").unwrap(); }>
        Sign in with Supabase
        </button>
        <br />
    )
}
