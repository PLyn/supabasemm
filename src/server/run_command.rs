use leptos::prelude::ServerFnError;
use std::process::Stdio;
use tokio::process::Command;

pub async fn run_command(command: String, db_string: String) -> Result<String, ServerFnError> {
    let mut cmd = Command::new("supabase");
    cmd.args(&[
        "inspect",
        "db",
        command.as_str(),
        "--db-url",
        db_string.as_str(),
    ]);

    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let child = cmd.spawn();

    match child {
        Ok(child_process) => {
            println!("Child Process PID: {:?}", child_process.id());

            match child_process.wait_with_output().await {
                Ok(output) => {
                    let stdout_logs = String::from_utf8_lossy(&output.stdout).to_string();
                    let stderr_logs = String::from_utf8_lossy(&output.stderr).to_string();

                    if output.status.success() {
                        if !stdout_logs.is_empty() {
                            println!("{}", stdout_logs);
                            return Ok(stdout_logs);
                        }

                        if !stderr_logs.is_empty() {
                            println!("{}", stderr_logs);
                            return Ok(stderr_logs);
                        }

                        Err(ServerFnError::new(
                            "Something has gone wrong. Command succeeded but no logs to output.",
                        ))
                    } else {
                        if !stderr_logs.is_empty() {
                            return Err(ServerFnError::new(stderr_logs));
                        }

                        if stderr_logs.is_empty() && !stdout_logs.is_empty() {
                            return Err(ServerFnError::new(stdout_logs));
                        }

                        Err(ServerFnError::new(
                            "Something has gone wrong. Command failed and no error logs to output.",
                        ))
                    }
                }
                Err(e) => Err(ServerFnError::new(format!("Command failed to run: {}", e))),
            }
        }
        Err(e) => Err(ServerFnError::new(format!(
            "Failed to spawn shell child process. Error: {}",
            e
        ))),
    }
}
