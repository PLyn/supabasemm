use leptos::prelude::*;
//use std::process::Stdio;
//use tokio::process::Command;

#[server]
pub async fn check_cache_hit_ratio(input: String) -> Result<String, ServerFnError> {
    println!("Spawning 'supabase inspect db cache-hit' command...");

    // Configure the command
    //let mut cmd = Command::new("supabase");
    //cmd.args(&["inspect", "db", "cache-hit"]);
    Ok(input)
}
