use leptos::prelude::*;
use crate::shared::server_functions::{check_auth_status, get_projects};
use crate::shared::models::{Project, ProjectConfig};

#[component]
pub fn HomePage() -> impl IntoView {
   let check_auth = OnceResource::new(check_auth_status());

   //local resource as it depends on auth call being true and doing both as resource led to race conditions
   let projects = LocalResource::new(move || async move { 
        if check_auth.await.unwrap() {
            let projects_result = get_projects().await;
            if let Ok(projects) = projects_result {
                return projects;
            }
        }
        Vec::new()
   });

    let source_project_rw: RwSignal<String> = RwSignal::new("".to_string());

    Effect::new(move |_| {
        if let Some(project_list) = projects.get() {
            if let Some(first_project) = project_list.first() {
                source_project_rw.set(first_project.id.clone());
            }
        }
    });

   let count = RwSignal::new(0);
   let on_click = move |_| *count.write() += 1;

   view! {
      <Suspense
        fallback=move || view! { <p>"Loading..."</p> }
      >
         "Auth Status"
         {move || check_auth.get()}
      </Suspense>
      
    <select class="select select-info w-full" bind:Selected=source_project_rw >
        {move || {
            projects
                .get() 
                .map(|project_vec| {
                    project_vec.into_iter()
                        .map(|project| {
                            let display_text = format!("{} - {} - {} - {}", project.id, project.name, project.region, project.status);
                            view! { <option value={project.id.clone()}>{display_text}</option> }
                        })
                        .collect_view()
                })
                .unwrap_or_else(|| { 
                    vec![].into_iter().collect_view()
                })
        }}
    </select>
      <h1>"Welcome to Leptos!"</h1>
      <button on:click=on_click>"Click Me: " {count}</button>
      <div>"Test area below"</div>
      <h1>"Supabase Connect Example (Rust/Axum)"</h1>
      <h1 class="text-3xl font-bold text-red-600">Hello world!</h1>
      <button class="btn btn-primary">test button</button>
      <p>"This is an example of implementing Supabase OAuth integration."</p>
      <p>
         <a href="/connect-supabase/login" rel="external">"Login with Supabase"</a>
      </p>
      <p>
         <a href="/connect-supabase/projects" rel="external">"View Projects (if authenticated)"</a>
      </p>
    }
}
