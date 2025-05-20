use leptos::prelude::*;

#[component]
pub fn ProjectSelectForm(
    source_project_rw: RwSignal<String>,
    dest_project_rw: RwSignal<String>,
) -> impl IntoView {
    view!(
        <br />
        <label>Select Source Project</label>
        <select
        on:change:target=move |ev| {
            source_project_rw.set(ev.target().value().parse().unwrap());
        }
        prop:value=move || source_project_rw.get().to_string()
        >
        <option value="a">"a"</option>
        <option value="b">"b"</option>
        <option value="c">"c"</option>
        </select>
        <br />

        <div style="color: dodgerblue; margin: 20px 0; display: inline-block;"> // This div is fine within the fragment
            <svg width="200" height="200" viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
                <polygon points="50,90 20,40 80,40 50,90" fill="currentColor" stroke="currentColor" stroke-width="2"/>
                <rect x="45" y="10" width="10" height="30" fill="currentColor" stroke="currentColor" stroke-width="2"/>
            </svg>
        </div>

        <br />
        <label>Select Destination Project</label>
        <select
        on:change:target=move |ev| {
            dest_project_rw.set(ev.target().value().parse().unwrap());
        }
        prop:value=move || dest_project_rw.get().to_string()
        >
        <option value="0">"0"</option>
        <option value="1">"1"</option>
        <option value="2">"2"</option>
        </select>
        <br />
    )
}
