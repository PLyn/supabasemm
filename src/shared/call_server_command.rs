use leptos::prelude::*;

#[server]
pub async fn call_server_command(
    command: String,
    db_string: String,
) -> Result<String, ServerFnError> {
    use crate::server::run_command::run_command;

    run_command(command, db_string).await
}
