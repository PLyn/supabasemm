use crate::shared::models::Project;
use leptos::prelude::*;
#[server]
pub async fn check_cache_hit_ratio(input: String) -> Result<String, ServerFnError> {
    use std::process::Stdio;
    use tokio::process::Command;

    println!("Spawning 'supabase inspect db cache-hit' command...");

    let mut text: String = "".to_string();

    // Configure the command
    let mut cmd = Command::new("supabase");
    cmd.args(&["inspect", "db", "cache-hit", "--db-url", input.as_str()]);

    // We want to capture the output (stdout and stderr)
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    // Spawn the child process
    let child = cmd.spawn();

    match child {
        Ok(child_process) => {
            println!(
                "Command spawned successfully. PID: {:?}",
                child_process.id()
            );

            // Await the completion of the process and capture its output
            // The `output()` method waits for the process to exit and collects all output.
            // If you need to stream the output, you'd handle child.stdout/stderr directly.
            match child_process.wait_with_output().await {
                Ok(output) => {
                    if output.status.success() {
                        let stdout_str = String::from_utf8_lossy(&output.stdout).to_string();
                        let stderr_str = String::from_utf8_lossy(&output.stderr).to_string();

                        println!("\nCommand completed successfully!");

                        // Store the logs in variables
                        let supabase_stdout_logs = stdout_str;
                        let supabase_stderr_logs = stderr_str;

                        println!("\n--- Supabase STDOUT ---");
                        if supabase_stdout_logs.is_empty() {
                            println!("(empty)");
                        } else {
                            println!("{}", supabase_stdout_logs);
                            text = supabase_stdout_logs.to_string().clone();
                        }

                        println!("\n--- Supabase STDERR ---");
                        if supabase_stderr_logs.is_empty() {
                            println!("(empty)");
                        } else {
                            println!("{}", supabase_stderr_logs);
                        }

                        // Here you can further process supabase_stdout_logs and supabase_stderr_logs
                        // For example:
                        // if supabase_stdout_logs.contains("cache hit rate") {
                        //     println!("\nSuccessfully found 'cache hit rate' in logs.");
                        // }
                    } else {
                        let stderr_str = String::from_utf8_lossy(&output.stderr).to_string();
                        eprintln!("\nCommand failed with status: {}", output.status);
                        eprintln!("--- Supabase STDERR ---");
                        if stderr_str.is_empty() {
                            eprintln!("(empty, or error details not sent to stderr)");
                            let stdout_str = String::from_utf8_lossy(&output.stdout).to_string();
                            if !stdout_str.is_empty() {
                                eprintln!("--- Supabase STDOUT (in case of error) ---");
                                eprintln!("{}", stdout_str);
                            }
                        } else {
                            eprintln!("{}", stderr_str);
                            text = stderr_str.to_string().clone();
                        }
                    }
                }
                Err(e) => {
                    eprintln!("\nFailed to wait for command output: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to spawn command: {}", e);
            eprintln!("Please ensure 'supabase' CLI is installed and in your PATH.");
        }
    }
    Ok(text)
}

#[server]
pub async fn test() -> Result<Vec<Project>, ServerFnError> {
    // Imports
    use crate::server::api::call_api;
    use crate::server::api::handle_response_error;
    use leptos_axum::extract; // Import the extract function
    use tower_sessions::Session;

    let session: Session = extract().await?;
    let url = format!("{}/projects", "https://api.supabase.com/v1");
    let api_response = call_api(session, url).await?;

    if api_response.status().is_success() {
        match api_response.json::<Vec<Project>>().await {
            Ok(projects) => {
                eprintln!("Successfully parsed projects.{:?}", projects);
                Ok(projects) // Return the Vec<SupabaseProject> on success
            }
            Err(e) => {
                eprintln!("Error parsing JSON: {:?}", e);
                Err(ServerFnError::ServerError(format!(
                    "Error parsing JSON: {:?}",
                    e
                )))
            }
        }
    } else {
        Err(handle_response_error(api_response).await)
    }
}
