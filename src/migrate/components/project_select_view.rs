use leptos::prelude::*;

#[component]
pub fn ProjectSelectForm(
    source_project: ReadSignal<String>,
    set_source_project: WriteSignal<String>,
    dest_project: ReadSignal<String>,
    set_dest_project: WriteSignal<String>,
) -> impl IntoView {
    view!(
        <br />
        <label>Select Source Project</label>
        <select
          on:change:target=move |ev| {
            set_source_project.set(ev.target().value().parse().unwrap());
          }
          prop:value=move || source_project.get().to_string()
        >
          <option value="a">"a"</option>
          <option value="b">"b"</option>
          <option value="c">"c"</option>
        </select>
        <br />

        <br />
        <label>Select Destination Project</label>
        <select
          on:change:target=move |ev| {
            set_dest_project.set(ev.target().value().parse().unwrap());
          }
          prop:value=move || dest_project.get().to_string()
        >
          <option value="0">"0"</option>
          <option value="1">"1"</option>
          <option value="2">"2"</option>
        </select>
        <br />
    )
}
