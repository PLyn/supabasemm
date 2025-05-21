use leptos::prelude::*;
use leptos::task::spawn_local;
use super::functions::{check_auth_status, get_project_metrics, get_projects};
use crate::shared::models::{Project};
use crate::metrics::functions::ProjectMetrics;

#[component]
pub fn MetricsPage() -> impl IntoView {
    let is_authenticated_rw = RwSignal::new(false);
    let selected_project_rw = RwSignal::new("".to_string());
    let project_metrics_rw: RwSignal<Vec<ProjectMetrics>> = RwSignal::new(Vec::new()); 
    let projects_rw: RwSignal<Vec<Project>> = RwSignal::new(Vec::new()); 
    let loading_metrics_rw = RwSignal::new(false); 
    let metrics_error_rw = RwSignal::new(None::<String>); 

    Effect::new(move |_| {
        // Only added here to make this effect re-run every time current_step_rw changes
        let current_project_ref = selected_project_rw.get();
        // This effect will run on initial load
        spawn_local(async move {
            let auth_status = check_auth_status().await;
            let is_authenticated = match auth_status {
                Ok(status) => status,
                Err(e) => {
                    eprintln!("Error checking auth status: {:?}", e);
                    false
                }
            };
            is_authenticated_rw.set(is_authenticated);
        });
    });

    Effect::new(move |_| {
        //Only added here to make this effect re-run every time current_step_rw changes
        let current_project_ref = selected_project_rw.get();
        let is_authenticated = is_authenticated_rw.get();
        if is_authenticated && projects_rw.get().is_empty() {
            spawn_local(async move {
                let projects_result = get_projects().await;
                match projects_result {
                    Ok(projects_list) => {
                        projects_rw.set(projects_list);    
                    }
                    Err(e) => {
                        eprintln!("Failed to fetch projects: {:?}", e);
                    }
                }
            });
        }
    });

    Effect::new(move |_| {
        let current_project_ref = selected_project_rw.get();
        let authenticated = is_authenticated_rw.get();

        if !current_project_ref.is_empty() && authenticated {
            loading_metrics_rw.set(true);
            metrics_error_rw.set(None);
            spawn_local(async move {
                let metrics_result = get_project_metrics(current_project_ref).await;
                loading_metrics_rw.set(false);
                match metrics_result {
                    Ok(metrics) => {
                        project_metrics_rw.set(metrics);
                    }
                    Err(e) => {
                        metrics_error_rw.set(Some(format!("Failed to load metrics: {:?}", e)));
                    }
                }
            });
        } else {
            project_metrics_rw.set(Vec::new());
        }
    });

    view! {
        <h2>"Supabase Live Metrics"</h2>

        <br />
        <label>"Select Source Project"</label>
        <select
            on:change:target=move |ev| {
                selected_project_rw.set(ev.target().value().parse().unwrap());
            }
            prop:value=move || selected_project_rw.get().to_string()>
                <Suspense fallback=move || view! { <option value="">Loading projects...</option> }>
                    {move || {
                        if !projects_rw.get().is_empty() && selected_project_rw.get().is_empty() {
                            selected_project_rw.set(projects_rw.get()[0].id.clone());
                        }

                        projects_rw
                            .get()
                            .into_iter()
                            .map(|project| {
                                let display_text = format!(
                                "{} - {} - {} - {}",
                                project.id, project.name, project.region, project.status);
                    
                                view! { <option value={project.id.clone()}>{display_text}</option> }
                            })
                            .collect_view()
                        }
                    }
                </Suspense>
        </select>
        <br />

        <h3>"Project Metrics"</h3>
        {move || {
            if loading_metrics_rw.get() {
                view! { <p>"Loading metrics..."</p> }.into_any()
            } else if let Some(error) = metrics_error_rw.get() {
                view! { <p style="color: red;">{error}</p> }.into_any()
            } else {
                let metrics = project_metrics_rw.get();
                if metrics.is_empty() {
                    view! { <p>"No metrics available for this project."</p> }.into_any()
                } else {
                    view! {
                        <table>
                            <thead>
                                <tr>
                                    <th>"Metric Name"</th>
                                    <th>"Value"</th>
                                    <th>"Timestamp"</th>
                                </tr>
                            </thead>
                            <tbody>
                                {metrics
                                    .into_iter()
                                    .map(|metric| {
                                        view! {
                                            <tr>
                                                <td>{metric.metric_name}</td>
                                                <td>{metric.value}</td>
                                                <td>{metric.timestamp}</td>
                                            </tr>
                                        }
                                    })
                                    .collect_view()}
                            </tbody>
                        </table>
                    }.into_any()
                }
            }
        }}
    }
}