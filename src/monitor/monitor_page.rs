use super::check_cache_hit_ratio;
use super::monitor_functions::test;
use leptos::html::Input;
use leptos::prelude::*;
use leptos::task::spawn_local;

/// Renders the home page of your application.
#[component]
pub fn MonitorPage() -> impl IntoView {
    let input_ref = NodeRef::<Input>::new();
    let (conn_string, set_conn_string) = signal("".to_string());
    let (result_logs, set_result_logs) = signal("Output logs will be displayed here".to_string());

    view! {
        <h2>Supabase Monitor</h2>
        <h2>Set Database connection string</h2>

        <input node_ref=input_ref placeholder="Type something here." bind:value=(conn_string, set_conn_string) />
        <br />
        <button on:click=move |_| {
            spawn_local(async move {
                let input_value = input_ref.get_untracked().unwrap().value();
                let uppercase_text = check_cache_hit_ratio(input_value).await.unwrap_or_else(|e| e.to_string());

                set_result_logs.set(uppercase_text);
            });
        }>
        Check Cache hit Ratio
        </button>
        <br />
        <textarea rows=5 cols=40 bind:value=(result_logs, set_result_logs)></textarea>
        <button on:click=move |_| {
            spawn_local(async move {
                let projects = test().await;
                match projects {
                    Ok(projects_vec) => {
                        let projects_as_strings: Vec<String> = projects_vec
                                .into_iter() // Consume the Vec<SupabaseProject>
                                .map(|project| {
                                    // Format the project details into a String
                                    // You need to access project fields like project.name, project.id, etc.
                                    // Ensure SupabaseProject is imported and has necessary fields accessible
                                    format!(
                                        "Project: {} || ID: {} || Region: {} || Status: {})",
                                        project.name,
                                        project.id,
                                        project.region,
                                        project.status
                                    )
                                })
                                .collect();
                        let combined_output = projects_as_strings.join("\n");
                        set_result_logs.set(combined_output);
                    }
                    Err(e) => { // e is ServerFnError
                        eprintln!("Error calling test() server function: {:?}", e);
                        // Handle the error by setting the signal with the error message
                        set_result_logs.set(format!("Error fetching projects: {}", e));
                    }
                }
            });
        }>
        Get projects
        </button>
    }
}
