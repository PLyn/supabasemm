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
use crate::shared::models::SupabaseProject;

#[server]
pub async fn test() -> Result<Vec<SupabaseProject>, ServerFnError> {
    eprintln!("Session details - start");
    use crate::server::api_request::api_request;
    use leptos_axum::extract; // Import the extract function
    use tower_sessions::Session;
    let session: Session = extract().await?;

    // Print the entire session struct using the Debug trait
    eprintln!("Session details: {:?}", session);

    let url = format!("{}/projects", "https://api.supabase.com/v1");
    let api_response = api_request(session, url).await;

    match api_response {
        Ok(response) => {
            let status = response.status();
            if status.is_success() {
                match response.json::<Vec<SupabaseProject>>().await {
                    Ok(projects) => {
                        eprintln!("Successfully parsed projects.{:?}", projects);
                        Ok(projects) // Return the Vec<SupabaseProject> on success
                    }
                    Err(e) => {
                        eprintln!("Error parsing JSON: {:?}", e);
                        Err(ServerFnError::ServerError(format!(
                            "Error parsing JSON: {:?}",
                            e
                        ))) // Return a ServerFnError with the string
                    }
                }
            } else {
                let status_code = status.as_u16();
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                eprintln!(
                    "HTTP request failed with status {}: {}",
                    status_code, error_text
                );
                Err(ServerFnError::ServerError(format!(
                    "HTTP request failed with status {}: {}",
                    status_code, error_text
                ))) // Return a ServerFnError with the string
            }
        }
        Err(e) => {
            eprintln!("Error from api_request: {}", e);
            Err(ServerFnError::ServerError(format!(
                "Error from api_request: {}",
                e
            ))) // Return a ServerFnError with the string
        }
    }
}
