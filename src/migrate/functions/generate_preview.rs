use leptos::{html::Object, prelude::*};
use super::api_call;

#[server]
pub async fn generate_preview(
    source_project_rw: String,
    dest_project_rw: String,
)  -> Result<String, ServerFnError> {
    use futures::join;
    use gloo_timers::future::TimeoutFuture;

    join!(
        async {
            let auth_config_url = format!("/v1/projects/{}/config/auth", source_project_rw);
            let auth_config = api_call(auth_config_url);
            eprintln!("Task 1 complete");
        },
        async {
            let auth_config_url = format!("/v1/projects/{}/config/storage", source_project_rw);
            let auth_config = api_call(auth_config_url);
            eprintln!("Task 1 complete");
        },

        async {
            let auth_config_url = format!("/v1/projects/{}/config/auth", dest_project_rw);
            let auth_config = api_call(auth_config_url);
            eprintln!("Task 1 complete");
        },
        async {
            let auth_config_url = format!("/v1/projects/{}/config/storage", dest_project_rw);
            let auth_config = api_call(auth_config_url);
            eprintln!("Task 1 complete");
        }
    );
    Ok("".to_string())
}
