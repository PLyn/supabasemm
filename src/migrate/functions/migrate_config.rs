use crate::shared::models::ProjectConfig;
use leptos::prelude::*;

#[cfg(feature = "ssr")]
use super::json_diff;
#[cfg(feature = "ssr")]
use crate::migrate::migrate_page::ConfigItems;
#[cfg(feature = "ssr")]
use crate::shared::server_functions::{
    mgmt_api_get, mgmt_api_get_bytes, mgmt_api_patch, mgmt_api_post_deploy, mgmt_api_put,
};
#[cfg(feature = "ssr")]
use serde_json::Value;
#[cfg(feature = "ssr")]
use std::error::Error; // Add this import for the `Error` trait

#[server]
pub async fn migrate_config(
    project_config: Vec<ProjectConfig>,
    source_id: String,
    dest_id: String,
) -> Result<Vec<ProjectConfig>, ServerFnError> {
    //server only imports
    use crate::shared::server_functions::check_auth_status;
    use leptos_axum::extract;
    use tower_sessions::Session;

    let session: Session = extract().await?;
    let is_auth = check_auth_status(session.clone()).await?;
    if !is_auth {
        return Ok(Vec::new());
    }

    let mut new_project_config = project_config;
    for service in new_project_config.iter_mut() {
        match service.name.as_str() {
            "Auth" => {
                let session_key = format!("{:?}", ConfigItems::Auth);
                let auth_config: Option<String> = session.get(session_key.clone().as_str()).await?;
                migrate_auth(session_key.clone(), auth_config, dest_id.clone(), service).await?;

                eprintln!("Removing auth session data");
                session.remove::<String>(session_key.as_str()).await.ok();
            }
            "EdgeFunctions" => {
                let session_key = format!("{:?}", ConfigItems::EdgeFunctions);
                let functions_config: Option<String> =
                    session.get(session_key.clone().as_str()).await?;
                migrate_functions(
                    session_key.clone(),
                    functions_config,
                    source_id.clone(),
                    dest_id.clone(),
                    service,
                )
                .await?;

                eprintln!("Removing functions session data");
                session.remove::<String>(session_key.as_str()).await.ok();
            }
            _ => {}
        }
    }
    Ok(new_project_config)
}

#[cfg(feature = "ssr")]
async fn migrate_auth(
    key: String,
    config: Option<String>,
    dest_id: String,
    service_config: &mut ProjectConfig,
) -> Result<(), ServerFnError> {
    use crate::shared::models::AuthConfigStruct;

    if let Some(source_json) = config {
        let config_struct: AuthConfigStruct = serde_json::from_str(&source_json.clone())?;
        let patched_config: AuthConfigStruct = config_struct.remove_smtp_fields_if_disabled();
        let patch_json = serde_json::to_string(&patched_config)?;

        let _response = mgmt_api_patch(
            format!("/projects/{}/config/auth", dest_id),
            patch_json.clone(),
        )
        .await?;

        let dest_new_json = mgmt_api_get(format!("/projects/{}/config/auth", dest_id)).await?;

        let source_value: Value = serde_json::from_str(&source_json)?;
        let dest_value: Value = serde_json::from_str(&dest_new_json)?;
        let project_config_entry = json_diff(key, source_value, dest_value).await?;

        if let Some(new_config_entry) = project_config_entry {
            if new_config_entry.name == service_config.name {
                *service_config = new_config_entry;
            }
        }
    }
    Ok(())
}

#[cfg(feature = "ssr")]
use std::str;

#[cfg(feature = "ssr")]
pub fn extract_source_from_eszip(eszip_content: &str) -> Result<String, String> {
    // Look for the source code section marker
    let source_marker = "---SUPABASE-SOURCE-CODE-ESZIP---";
    let npm_marker = "---SUPABASE-NPM-RC-SCOPES---";

    // Find the source code section
    if let Some(source_start) = eszip_content.find(source_marker) {
        // Start after the marker
        let code_start = source_start + source_marker.len();

        // Find the end marker (NPM scopes section)
        if let Some(npm_start) = eszip_content[code_start..].find(npm_marker) {
            // Extract the code between markers
            let source_code = &eszip_content[code_start..code_start + npm_start];

            // Clean up the extracted code
            let cleaned_code = source_code.trim();

            // The content between markers might have the code repeated with version numbers
            // Look for the first occurrence starting with the import statement
            if let Some(import_start) =
                cleaned_code.find("import \"jsr:@supabase/functions-js/edge-runtime.d.ts\";")
            {
                // Find where this instance of the code ends
                // It typically ends with "});" followed by a version number like "1.1"

                // Search for the end pattern
                let search_from = import_start;
                if let Some(end_pos) = cleaned_code[search_from..].find("});\n1") {
                    // Extract just the first occurrence
                    let final_code = &cleaned_code[import_start..search_from + end_pos + 3]; // +3 to include "});"
                    return Ok(final_code.trim().to_string());
                } else {
                    // If no version marker found, look for the closing of Deno.serve
                    if let Some(end_pos) = find_last_closing_brace(cleaned_code, import_start) {
                        let final_code = &cleaned_code[import_start..=end_pos];
                        return Ok(final_code.trim().to_string());
                    }
                }
            }

            // If standard patterns don't work, return the cleaned code
            return Ok(cleaned_code.to_string());
        }
    }

    Err("Could not extract source code from ESZIP".to_string())
}

// Helper function to find the last closing brace of the main function
#[cfg(feature = "ssr")]
fn find_last_closing_brace(code: &str, start_pos: usize) -> Option<usize> {
    let code_from_start = &code[start_pos..];
    let mut brace_count = 0;
    let mut last_closing_pos = None;

    for (i, ch) in code_from_start.char_indices() {
        match ch {
            '{' => brace_count += 1,
            '}' => {
                brace_count -= 1;
                if brace_count == 0 && code_from_start[i..].starts_with("});") {
                    last_closing_pos = Some(start_pos + i + 2); // Include the ");"
                    break;
                }
            }
            _ => {}
        }
    }

    last_closing_pos
}

#[cfg(feature = "ssr")]
async fn migrate_functions(
    key: String,
    config: Option<String>,
    source_id: String,
    dest_id: String,
    service_config: &mut ProjectConfig,
) -> Result<(), ServerFnError> {
    use crate::shared::models::FunctionConfigStruct;
    use std::path::{Path, PathBuf}; // Import Path and PathBuf

    if let Some(source_json) = config {
        let mut functions_struct: Vec<FunctionConfigStruct> =
            serde_json::from_str(&source_json.clone())?;

        // Before iterating and deploying, prepare the functions_struct
        // (This part for updating functions_struct itself can stay if needed,
        // but the key change is below for the deployment logic)
        for function in &mut functions_struct {
            if let Some(ref mut entrypoint_path) = function.entrypoint_path {
                // This modification seems to be for updating the config, not for deployment
                // Keep it if it's for reflecting changes in the project_config data structure
                *entrypoint_path = entrypoint_path.replace(&source_id, &dest_id);
            }
            if let Some(ref mut import_map_path) = function.import_map_path {
                *import_map_path = import_map_path.replace(&source_id, &dest_id);
            }
        }
        let patch_json = serde_json::to_string(&functions_struct)?;
        // This PUT might be redundant if the deploy endpoint handles creation/update.
        // If it's just for updating the list of functions, then keep it.
        let _put = mgmt_api_put(format!("/projects/{}/functions", dest_id), patch_json).await?;

        eprintln!("{:?}", functions_struct);

        for function in functions_struct {
            if let Some(fn_slug) = function.slug.clone() {
                // Get the ESZIP body as a string
                let fn_body_string = mgmt_api_get(format!(
                    "/projects/{}/functions/{}/body",
                    source_id, fn_slug
                ))
                .await?;

                eprintln!("Processing function:");
                eprintln!("  Slug: {}", fn_slug);
                eprintln!(
                    "  ESZIP content length: {} characters",
                    fn_body_string.len()
                );

                eprintln!(
                    "  ESZIP preview: {}",
                    &fn_body_string.chars().take(500).collect::<String>()
                );

                // Extract source code from ESZIP string
                let source_code = match extract_source_from_eszip(&fn_body_string) {
                    Ok(code) => {
                        eprintln!("  Successfully extracted source code");
                        eprintln!("  Code length: {} characters", code.len());
                        eprintln!(
                            "  Code preview: {}",
                            &code.chars().take(200).collect::<String>()
                        );
                        code
                    }
                    Err(e) => {
                        eprintln!("  Failed to extract source: {}", e);
                        // Fix 1: Specify the generic argument for ServerFnError::ServerError
                        return Err(ServerFnError::ServerError(
                            "Access token not found in session".to_string(),
                        ));
                    }
                };

                // Update project references in the source code
                let updated_source = source_code.replace(&source_id, &dest_id);

                // --- IMPORTANT CHANGE HERE ---
                let relative_entrypoint_path = if let Some(original_entrypoint) =
                    function.entrypoint_path.clone()
                {
                    // Strip the "file:///" prefix if it exists
                    let path_str = original_entrypoint
                        .strip_prefix("file:///")
                        .unwrap_or(&original_entrypoint);

                    // Create a PathBuf and then extract the "components" to find the last meaningful segments
                    let path = PathBuf::from(path_str);

                    // Supabase's internal paths often look like /tmp/user_fn_.../{version}/index.ts or /tmp/user_fn_.../{version}/source/index.ts
                    // We need to find the part after the project-specific temp directory and version.
                    // A simple approach is to find the last two or three components if the structure is known.
                    // For example, if it's "source/index.ts" or just "index.ts"
                    let mut components_iter = path.components().rev(); // Iterate from the end

                    // Get the actual file name (e.g., "index.ts")
                    let file_name = components_iter
                        .next()
                        .and_then(|c| c.as_os_str().to_str())
                        .ok_or_else(|| {
                            // Fix 2: Specify the generic argument for ServerFnError::ServerError
                            ServerFnError::<String>::ServerError(
                                "Could not determine entrypoint filename".to_string(),
                            )
                        })
                        .unwrap_or("");

                    // Check if there's a parent directory like "source"
                    let parent_dir = components_iter.next().and_then(|c| c.as_os_str().to_str());

                    if let Some(dir) = parent_dir {
                        if dir != "src"
                            && dir != "deno_dist"
                            && dir != "output"
                            && !dir.starts_with("user_fn_")
                            && !dir.starts_with("tmp")
                        {
                            // If the parent directory is not one of the temporary/internal ones,
                            // and it's not a root temp dir, assume it's part of the actual relative path.
                            format!("{}/{}", dir, file_name)
                        } else {
                            // Otherwise, just the filename
                            file_name.to_string()
                        }
                    } else {
                        file_name.to_string()
                    }
                } else {
                    return Err(ServerFnError::ServerError(format!(
                        "Access token not found in session"
                    )));
                };
                // --- END IMPORTANT CHANGE ---

                // Prepare the update payload for the PATCH endpoint (if needed)
                // Note: The PATCH endpoint for functions usually updates metadata, not the code itself.
                // The `deploy` endpoint is for code updates.
                // If this PATCH is causing an issue or is redundant, you might remove it.
                let update_payload = serde_json::json!({
                    "name": function.name.clone().unwrap_or(fn_slug.clone()),
                    // Do NOT include "body" here, as the PATCH /functions/{slug} endpoint
                    // does not expect the body of the function. The deploy endpoint handles the body.
                    "verify_jwt": function.verify_jwt.unwrap_or(true)
                });

                eprintln!("  Updating function metadata (if applicable)...");

                // Call the update endpoint (if you intend to update metadata separately)
                // This call might be optional/redundant if `deploy` handles all updates.
                let update_url = format!("/projects/{}/functions/{}", dest_id, fn_slug);
                let _result = mgmt_api_patch(update_url, update_payload.to_string()).await?; // This likely updates name, verify_jwt etc. but not the code

                eprintln!("  Function metadata updated successfully!");
                eprintln!("  Deploying function with source code...");

                // Call the deploy endpoint with the corrected relative entrypoint_path
                let _patch = mgmt_api_post_deploy(
                    dest_id.clone(),
                    fn_slug,
                    updated_source,
                    relative_entrypoint_path, // Use the extracted relative path
                    function.import_map_path.clone(),
                )
                .await?;

                eprintln!("  Function deployed successfully!");
            }
        }

        // Get the updated configuration for diffing
        let dest_new_json = mgmt_api_get(format!("/projects/{}/functions", dest_id)).await?;
        let source_value: Value = serde_json::from_str(&source_json)?;
        let dest_value: Value = serde_json::from_str(&dest_new_json)?;
        let project_config_entry = json_diff(key, source_value, dest_value).await?;

        if let Some(new_config_entry) = project_config_entry {
            if new_config_entry.name == service_config.name {
                *service_config = new_config_entry;
            }
        }
    }
    Ok(())
}
