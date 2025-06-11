use serde::{Deserialize, Serialize};

pub static AUTH_URL: &str = "https://api.supabase.com/v1/oauth/authorize";
pub static TOKEN_URL: &str = "https://api.supabase.com/v1/oauth/token";
pub static MGMT_API_BASE_URL: &str = "https://api.supabase.com/v1";

pub static REDIRECT_URL: &str = "http://localhost:3000/connect-supabase/oauth2/callback";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub region: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectConfig {
    pub name: String,
    pub diffs: Vec<DiffEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiffEntry {
    pub key: String,
    pub source_value: String,
    pub dest_value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectMetrics {
    pub timestamp: String,
    pub value: String,
    pub metric_name: String,
    pub labels: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApiKeyStruct {
    pub name: String,
    pub api_key: String,
    pub id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct AuthConfigStruct {
    // API Configuration
    // Team/Enterprise only
    // will comment out for now
    //#[serde(skip_serializing_if = "Option::is_none")]
    //pub api_max_request_duration: Option<u32>,

    // Team/Enterprise only. Will comment out for now.
    //#[serde(skip_serializing_if = "Option::is_none")]
    //pub db_max_pool_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_signup: Option<bool>,

    // External Authentication Providers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_anonymous_users_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_apple_additional_client_ids: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_apple_client_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_apple_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_apple_secret: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_azure_client_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_azure_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_azure_secret: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_azure_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_bitbucket_client_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_bitbucket_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_bitbucket_secret: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_discord_client_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_discord_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_discord_secret: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_email_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_facebook_client_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_facebook_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_facebook_secret: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_figma_client_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_figma_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_figma_secret: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_github_client_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_github_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_github_secret: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_gitlab_client_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_gitlab_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_gitlab_secret: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_gitlab_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_google_additional_client_ids: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_google_client_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_google_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_google_secret: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_google_skip_nonce_check: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_kakao_client_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_kakao_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_kakao_secret: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_keycloak_client_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_keycloak_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_keycloak_secret: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_keycloak_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_linkedin_oidc_client_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_linkedin_oidc_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_linkedin_oidc_secret: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_notion_client_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_notion_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_notion_secret: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_phone_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_slack_client_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_slack_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_slack_oidc_client_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_slack_oidc_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_slack_oidc_secret: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_slack_secret: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_spotify_client_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_spotify_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_spotify_secret: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_twitch_client_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_twitch_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_twitch_secret: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_twitter_client_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_twitter_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_twitter_secret: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_web3_solana_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_workos_client_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_workos_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_workos_secret: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_workos_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_zoom_client_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_zoom_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_zoom_secret: Option<String>,

    // Webhook Configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_custom_access_token_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_custom_access_token_secrets: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_custom_access_token_uri: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_mfa_verification_attempt_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_mfa_verification_attempt_secrets: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_mfa_verification_attempt_uri: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_password_verification_attempt_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_password_verification_attempt_secrets: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_password_verification_attempt_uri: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_send_email_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_send_email_secrets: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_send_email_uri: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_send_sms_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_send_sms_secrets: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_send_sms_uri: Option<String>,

    // JWT Configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_exp: Option<u32>,

    // Mailer Configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailer_allow_unverified_email_sign_ins: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailer_autoconfirm: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailer_otp_exp: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailer_otp_length: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailer_secure_email_change_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailer_subjects_confirmation: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailer_subjects_email_change: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailer_subjects_invite: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailer_subjects_magic_link: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailer_subjects_reauthentication: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailer_subjects_recovery: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailer_templates_confirmation_content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailer_templates_email_change_content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailer_templates_invite_content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailer_templates_magic_link_content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailer_templates_reauthentication_content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailer_templates_recovery_content: Option<String>,

    // MFA Configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_max_enrolled_factors: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_phone_enroll_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_phone_max_frequency: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_phone_otp_length: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_phone_template: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_phone_verify_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_totp_enroll_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_totp_verify_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_web_authn_enroll_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_web_authn_verify_enabled: Option<bool>,

    // Password Configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_hibp_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_min_length: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_required_characters: Option<String>,

    // Rate Limiting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_anonymous_users: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_email_sent: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_otp: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_sms_sent: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_token_refresh: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_verify: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_web3: Option<u32>,

    // Token Configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token_rotation_enabled: Option<bool>,

    // SAML Configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saml_allow_encrypted_assertions: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub saml_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub saml_external_url: Option<String>,

    // Security Configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_captcha_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_captcha_provider: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_captcha_secret: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_manual_linking_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_refresh_token_reuse_interval: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_update_password_require_reauthentication: Option<bool>,

    // Session Configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sessions_inactivity_timeout: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sessions_single_per_user: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sessions_tags: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sessions_timebox: Option<u32>,

    // Site Configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_url: Option<String>,

    // SMS Configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_autoconfirm: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_max_frequency: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_messagebird_access_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_messagebird_originator: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_otp_exp: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_otp_length: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_provider: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_template: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_test_otp: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_test_otp_valid_until: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_textlocal_api_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_textlocal_sender: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_twilio_account_sid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_twilio_auth_token: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_twilio_content_sid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_twilio_message_service_sid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_twilio_verify_account_sid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_twilio_verify_auth_token: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_twilio_verify_message_service_sid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_vonage_api_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_vonage_api_secret: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_vonage_from: Option<String>,

    // SMTP Configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smtp_admin_email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub smtp_host: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub smtp_max_frequency: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub smtp_pass: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub smtp_port: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub smtp_sender_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub smtp_user: Option<String>,

    // URI Configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri_allow_list: Option<String>,
}
impl AuthConfigStruct {
    /// Check if custom SMTP is fully configured
    pub fn has_custom_smtp(&self) -> bool {
        self.smtp_admin_email.is_some()
            && self.smtp_host.is_some()
            && self.smtp_port.is_some()
            && self.smtp_user.is_some()
            && self.smtp_pass.is_some()
    }

    /// Prepare the struct for PATCH request by removing SMTP-dependent fields
    /// if SMTP is not fully configured
    pub fn remove_smtp_fields_if_disabled(mut self) -> Self {
        if !self.has_custom_smtp() {
            // Remove fields that require custom SMTP
            self.smtp_sender_name = None;
            self.rate_limit_email_sent = None;

            // Also remove partial SMTP configuration to avoid confusion
            self.smtp_admin_email = None;
            self.smtp_host = None;
            self.smtp_port = None;
            self.smtp_user = None;
            self.smtp_pass = None;
            self.smtp_max_frequency = None;
        }
        self
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct FunctionConfigStruct {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_jwt: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_map: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_map_path: Option<String>,
}
