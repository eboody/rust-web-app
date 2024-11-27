/*
 * Coolify
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// Team : Team model
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Team {
    /// The unique identifier of the team.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// The name of the team.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description of the team.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the team is personal or not.
    #[serde(rename = "personal_team", skip_serializing_if = "Option::is_none")]
    pub personal_team: Option<bool>,
    /// The date and time the team was created.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// The date and time the team was last updated.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Whether SMTP is enabled or not.
    #[serde(rename = "smtp_enabled", skip_serializing_if = "Option::is_none")]
    pub smtp_enabled: Option<bool>,
    /// The email address to send emails from.
    #[serde(rename = "smtp_from_address", skip_serializing_if = "Option::is_none")]
    pub smtp_from_address: Option<String>,
    /// The name to send emails from.
    #[serde(rename = "smtp_from_name", skip_serializing_if = "Option::is_none")]
    pub smtp_from_name: Option<String>,
    /// The email addresses to send emails to.
    #[serde(rename = "smtp_recipients", skip_serializing_if = "Option::is_none")]
    pub smtp_recipients: Option<String>,
    /// The SMTP host.
    #[serde(rename = "smtp_host", skip_serializing_if = "Option::is_none")]
    pub smtp_host: Option<String>,
    /// The SMTP port.
    #[serde(rename = "smtp_port", skip_serializing_if = "Option::is_none")]
    pub smtp_port: Option<String>,
    /// The SMTP encryption.
    #[serde(rename = "smtp_encryption", skip_serializing_if = "Option::is_none")]
    pub smtp_encryption: Option<String>,
    /// The SMTP username.
    #[serde(rename = "smtp_username", skip_serializing_if = "Option::is_none")]
    pub smtp_username: Option<String>,
    /// The SMTP password.
    #[serde(rename = "smtp_password", skip_serializing_if = "Option::is_none")]
    pub smtp_password: Option<String>,
    /// The SMTP timeout.
    #[serde(rename = "smtp_timeout", skip_serializing_if = "Option::is_none")]
    pub smtp_timeout: Option<String>,
    /// Whether to send test notifications via SMTP.
    #[serde(rename = "smtp_notifications_test", skip_serializing_if = "Option::is_none")]
    pub smtp_notifications_test: Option<bool>,
    /// Whether to send deployment notifications via SMTP.
    #[serde(rename = "smtp_notifications_deployments", skip_serializing_if = "Option::is_none")]
    pub smtp_notifications_deployments: Option<bool>,
    /// Whether to send status change notifications via SMTP.
    #[serde(rename = "smtp_notifications_status_changes", skip_serializing_if = "Option::is_none")]
    pub smtp_notifications_status_changes: Option<bool>,
    /// Whether to send scheduled task notifications via SMTP.
    #[serde(rename = "smtp_notifications_scheduled_tasks", skip_serializing_if = "Option::is_none")]
    pub smtp_notifications_scheduled_tasks: Option<bool>,
    /// Whether to send database backup notifications via SMTP.
    #[serde(rename = "smtp_notifications_database_backups", skip_serializing_if = "Option::is_none")]
    pub smtp_notifications_database_backups: Option<bool>,
    /// Whether to send server disk usage notifications via SMTP.
    #[serde(rename = "smtp_notifications_server_disk_usage", skip_serializing_if = "Option::is_none")]
    pub smtp_notifications_server_disk_usage: Option<bool>,
    /// Whether Discord is enabled or not.
    #[serde(rename = "discord_enabled", skip_serializing_if = "Option::is_none")]
    pub discord_enabled: Option<bool>,
    /// The Discord webhook URL.
    #[serde(rename = "discord_webhook_url", skip_serializing_if = "Option::is_none")]
    pub discord_webhook_url: Option<String>,
    /// Whether to send test notifications via Discord.
    #[serde(rename = "discord_notifications_test", skip_serializing_if = "Option::is_none")]
    pub discord_notifications_test: Option<bool>,
    /// Whether to send deployment notifications via Discord.
    #[serde(rename = "discord_notifications_deployments", skip_serializing_if = "Option::is_none")]
    pub discord_notifications_deployments: Option<bool>,
    /// Whether to send status change notifications via Discord.
    #[serde(rename = "discord_notifications_status_changes", skip_serializing_if = "Option::is_none")]
    pub discord_notifications_status_changes: Option<bool>,
    /// Whether to send database backup notifications via Discord.
    #[serde(rename = "discord_notifications_database_backups", skip_serializing_if = "Option::is_none")]
    pub discord_notifications_database_backups: Option<bool>,
    /// Whether to send scheduled task notifications via Discord.
    #[serde(rename = "discord_notifications_scheduled_tasks", skip_serializing_if = "Option::is_none")]
    pub discord_notifications_scheduled_tasks: Option<bool>,
    /// Whether to send server disk usage notifications via Discord.
    #[serde(rename = "discord_notifications_server_disk_usage", skip_serializing_if = "Option::is_none")]
    pub discord_notifications_server_disk_usage: Option<bool>,
    /// Whether to show the boarding screen or not.
    #[serde(rename = "show_boarding", skip_serializing_if = "Option::is_none")]
    pub show_boarding: Option<bool>,
    /// Whether to enable resending or not.
    #[serde(rename = "resend_enabled", skip_serializing_if = "Option::is_none")]
    pub resend_enabled: Option<bool>,
    /// The resending API key.
    #[serde(rename = "resend_api_key", skip_serializing_if = "Option::is_none")]
    pub resend_api_key: Option<String>,
    /// Whether to use instance email settings or not.
    #[serde(rename = "use_instance_email_settings", skip_serializing_if = "Option::is_none")]
    pub use_instance_email_settings: Option<bool>,
    /// Whether Telegram is enabled or not.
    #[serde(rename = "telegram_enabled", skip_serializing_if = "Option::is_none")]
    pub telegram_enabled: Option<bool>,
    /// The Telegram token.
    #[serde(rename = "telegram_token", skip_serializing_if = "Option::is_none")]
    pub telegram_token: Option<String>,
    /// The Telegram chat ID.
    #[serde(rename = "telegram_chat_id", skip_serializing_if = "Option::is_none")]
    pub telegram_chat_id: Option<String>,
    /// Whether to send test notifications via Telegram.
    #[serde(rename = "telegram_notifications_test", skip_serializing_if = "Option::is_none")]
    pub telegram_notifications_test: Option<bool>,
    /// Whether to send deployment notifications via Telegram.
    #[serde(rename = "telegram_notifications_deployments", skip_serializing_if = "Option::is_none")]
    pub telegram_notifications_deployments: Option<bool>,
    /// Whether to send status change notifications via Telegram.
    #[serde(rename = "telegram_notifications_status_changes", skip_serializing_if = "Option::is_none")]
    pub telegram_notifications_status_changes: Option<bool>,
    /// Whether to send database backup notifications via Telegram.
    #[serde(rename = "telegram_notifications_database_backups", skip_serializing_if = "Option::is_none")]
    pub telegram_notifications_database_backups: Option<bool>,
    /// The Telegram test message thread ID.
    #[serde(rename = "telegram_notifications_test_message_thread_id", skip_serializing_if = "Option::is_none")]
    pub telegram_notifications_test_message_thread_id: Option<String>,
    /// The Telegram deployment message thread ID.
    #[serde(rename = "telegram_notifications_deployments_message_thread_id", skip_serializing_if = "Option::is_none")]
    pub telegram_notifications_deployments_message_thread_id: Option<String>,
    /// The Telegram status change message thread ID.
    #[serde(rename = "telegram_notifications_status_changes_message_thread_id", skip_serializing_if = "Option::is_none")]
    pub telegram_notifications_status_changes_message_thread_id: Option<String>,
    /// The Telegram database backup message thread ID.
    #[serde(rename = "telegram_notifications_database_backups_message_thread_id", skip_serializing_if = "Option::is_none")]
    pub telegram_notifications_database_backups_message_thread_id: Option<String>,
    /// The custom server limit.
    #[serde(rename = "custom_server_limit", skip_serializing_if = "Option::is_none")]
    pub custom_server_limit: Option<String>,
    /// Whether to send scheduled task notifications via Telegram.
    #[serde(rename = "telegram_notifications_scheduled_tasks", skip_serializing_if = "Option::is_none")]
    pub telegram_notifications_scheduled_tasks: Option<bool>,
    /// The Telegram scheduled task message thread ID.
    #[serde(rename = "telegram_notifications_scheduled_tasks_thread_id", skip_serializing_if = "Option::is_none")]
    pub telegram_notifications_scheduled_tasks_thread_id: Option<String>,
    /// The members of the team.
    #[serde(rename = "members", skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<models::User>>,
}

impl Team {
    /// Team model
    pub fn new() -> Team {
        Team {
            id: None,
            name: None,
            description: None,
            personal_team: None,
            created_at: None,
            updated_at: None,
            smtp_enabled: None,
            smtp_from_address: None,
            smtp_from_name: None,
            smtp_recipients: None,
            smtp_host: None,
            smtp_port: None,
            smtp_encryption: None,
            smtp_username: None,
            smtp_password: None,
            smtp_timeout: None,
            smtp_notifications_test: None,
            smtp_notifications_deployments: None,
            smtp_notifications_status_changes: None,
            smtp_notifications_scheduled_tasks: None,
            smtp_notifications_database_backups: None,
            smtp_notifications_server_disk_usage: None,
            discord_enabled: None,
            discord_webhook_url: None,
            discord_notifications_test: None,
            discord_notifications_deployments: None,
            discord_notifications_status_changes: None,
            discord_notifications_database_backups: None,
            discord_notifications_scheduled_tasks: None,
            discord_notifications_server_disk_usage: None,
            show_boarding: None,
            resend_enabled: None,
            resend_api_key: None,
            use_instance_email_settings: None,
            telegram_enabled: None,
            telegram_token: None,
            telegram_chat_id: None,
            telegram_notifications_test: None,
            telegram_notifications_deployments: None,
            telegram_notifications_status_changes: None,
            telegram_notifications_database_backups: None,
            telegram_notifications_test_message_thread_id: None,
            telegram_notifications_deployments_message_thread_id: None,
            telegram_notifications_status_changes_message_thread_id: None,
            telegram_notifications_database_backups_message_thread_id: None,
            custom_server_limit: None,
            telegram_notifications_scheduled_tasks: None,
            telegram_notifications_scheduled_tasks_thread_id: None,
            members: None,
        }
    }
}
