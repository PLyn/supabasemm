// src/metrics/metrics_page.rs
use super::functions::websocket_metrics_stream;
use crate::shared::models::{Project, ProjectMetrics};
use crate::shared::server_functions::get_projects;
use leptos::prelude::*;
use leptos::task::spawn_local;

const METRICS_REFRESH_TIME: i8 = 60;

#[component]
pub fn MetricsPage() -> impl IntoView {
    let selected_project_rw = RwSignal::new("".to_string());
    let project_metrics_rw: RwSignal<Vec<ProjectMetrics>> = RwSignal::new(Vec::new());
    let projects_rw: RwSignal<Vec<Project>> = RwSignal::new(Vec::new());

    let websocket_enabled_rw = RwSignal::new(false);
    let websocket_status_rw = RwSignal::new("Disconnected".to_string());

    let seconds_until_update_rw = RwSignal::new(METRICS_REFRESH_TIME);
    let countdown_interval_handle_rw: RwSignal<Option<IntervalHandle>> = RwSignal::new(None);

    Effect::new(move |_| {
        spawn_local(async move {
            if projects_rw.get_untracked().is_empty() {
                let projects_result = get_projects().await;
                if let Ok(projects_list) = projects_result {
                    projects_rw.set(projects_list);
                };
            }
        });
    });

    let start_countdown = move || {
        if let Some(handle) = countdown_interval_handle_rw.get_untracked() {
            handle.clear();
        }

        seconds_until_update_rw.set(METRICS_REFRESH_TIME);

        let handle = set_interval_with_handle(
            move || {
                let current = seconds_until_update_rw.get_untracked();
                if current > 0 {
                    seconds_until_update_rw.set(current - 1);
                } else {
                    seconds_until_update_rw.set(METRICS_REFRESH_TIME);
                }
            },
            std::time::Duration::from_secs(1),
        )
        .expect("Failed to create interval");

        countdown_interval_handle_rw.set(Some(handle));
    };

    let stop_countdown = move || {
        if let Some(handle) = countdown_interval_handle_rw.get_untracked() {
            handle.clear();
            countdown_interval_handle_rw.set(None);
        }
        seconds_until_update_rw.set(METRICS_REFRESH_TIME);
    };

    let start_websocket = move |_: leptos::ev::MouseEvent| {
        if selected_project_rw.get_untracked().is_empty() {
            websocket_status_rw.set("Error: No project selected".to_string());
            return;
        }

        websocket_enabled_rw.set(true);
        websocket_status_rw.set("Connecting...".to_string());

        spawn_local(async move {
            use futures::{channel::mpsc, SinkExt, StreamExt};

            let (mut tx, rx) = mpsc::channel(1);

            let project_ref = selected_project_rw.get_untracked();
            if let Err(e) = tx.send(Ok(project_ref)).await {
                websocket_status_rw.set(format!("Failed to send project ref: {}", e));
                websocket_enabled_rw.set(false);
                stop_countdown();
                return;
            }

            match websocket_metrics_stream(rx.into()).await {
                Ok(mut messages) => {
                    websocket_status_rw
                        .set("Connected - Receiving metrics every minute".to_string());
                    start_countdown();

                    while let Some(msg) = messages.next().await {
                        match msg {
                            Ok(metrics) => {
                                project_metrics_rw.set(metrics);
                                websocket_status_rw
                                    .set("Connected - Live updates active".to_string());
                                seconds_until_update_rw.set(METRICS_REFRESH_TIME);
                            }
                            Err(e) => {
                                websocket_status_rw.set(format!("Error: {}", e));
                                leptos::logging::warn!("WebSocket error: {}", e);
                            }
                        }
                    }

                    websocket_status_rw.set("Disconnected".to_string());
                    websocket_enabled_rw.set(false);
                    stop_countdown();
                }
                Err(e) => {
                    websocket_status_rw.set(format!("Connection failed: {}", e));
                    websocket_enabled_rw.set(false);
                    stop_countdown();
                    leptos::logging::warn!("WebSocket connection failed: {}", e);
                }
            }
        });
    };

    let stop_websocket = move |_: leptos::ev::MouseEvent| {
        websocket_enabled_rw.set(false);
        websocket_status_rw.set("Disconnected".to_string());
        stop_countdown();
    };

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
        <br />

        <div style="background-color: #f0f0f0; padding: 15px; margin: 10px 0; border-radius: 5px;">
            <h4>"Live Metrics via WebSocket"</h4>
            <p><strong>"Status: "</strong> {move || websocket_status_rw.get()}</p>

            <Show when=move || websocket_enabled_rw.get()>
                <p style="color: #28a745; font-weight: bold;">
                    <strong>"Next update in: "</strong>
                    {move || seconds_until_update_rw.get()} " seconds"
                </p>
            </Show>

            <Show
                when=move || !websocket_enabled_rw.get()
                fallback=move || view! {
                    <button
                        on:click=stop_websocket
                        style="background-color: #dc3545; color: white; padding: 8px 16px; border: none; border-radius: 4px; cursor: pointer;">
                        "Stop Live Updates"
                    </button>
                }
            >
                <button
                    on:click=start_websocket
                    disabled=move || selected_project_rw.get().is_empty()
                    style="background-color: #28a745; color: white; padding: 8px 16px; border: none; border-radius: 4px; cursor: pointer;"
                    prop:disabled=move || selected_project_rw.get().is_empty()>
                    "Enable Live Updates"
                </button>
            </Show>
        </div>

        <h3>"Project Metrics"</h3>
        <Show
            when=move || !project_metrics_rw.get().is_empty()
            fallback=move || view! { <p>"No metrics available. Select a project and enable live updates or refresh manually."</p> }
        >
            <p><strong>{move || project_metrics_rw.get().len()}</strong>" metrics available"</p>
            <table>
                <thead>
                    <tr>
                        <th>"Metric Name"</th>
                        <th>"Value"</th>
                        <th>"Labels"</th>
                        <th>"Timestamp"</th>
                    </tr>
                </thead>
                <tbody>
                    <For
                        each=move || project_metrics_rw.get()
                        key=|metric| format!("{}_{}", metric.metric_name, metric.timestamp)
                        children=move |metric| {
                            view! {
                                <tr>
                                    <td>{metric.metric_name}</td>
                                    <td>{metric.value}</td>
                                    <td style="font-size: 0.8em; max-width: 200px; word-wrap: break-word;">{metric.labels}</td>
                                    <td style="font-size: 0.8em;">{metric.timestamp}</td>
                                </tr>
                            }
                        }
                    />
                </tbody>
            </table>
        </Show>
    }
}
