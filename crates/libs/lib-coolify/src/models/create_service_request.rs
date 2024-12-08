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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateServiceRequest {
    /// The one-click service type
    #[serde(rename = "type")]
    pub r#type: Type,
    /// Name of the service.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Description of the service.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Project UUID.
    #[serde(rename = "project_uuid")]
    pub project_uuid: String,
    /// Environment name.
    #[serde(rename = "environment_name")]
    pub environment_name: String,
    /// Server UUID.
    #[serde(rename = "server_uuid")]
    pub server_uuid: String,
    /// Destination UUID. Required if server has multiple destinations.
    #[serde(rename = "destination_uuid", skip_serializing_if = "Option::is_none")]
    pub destination_uuid: Option<String>,
    /// Start the service immediately after creation.
    #[serde(rename = "instant_deploy", skip_serializing_if = "Option::is_none")]
    pub instant_deploy: Option<bool>,
}

impl CreateServiceRequest {
    pub fn new(r#type: Type, project_uuid: String, environment_name: String, server_uuid: String) -> CreateServiceRequest {
        CreateServiceRequest {
            r#type,
            name: None,
            description: None,
            project_uuid,
            environment_name,
            server_uuid,
            destination_uuid: None,
            instant_deploy: None,
        }
    }
}
/// The one-click service type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "activepieces")]
    Activepieces,
    #[serde(rename = "appsmith")]
    Appsmith,
    #[serde(rename = "appwrite")]
    Appwrite,
    #[serde(rename = "authentik")]
    Authentik,
    #[serde(rename = "babybuddy")]
    Babybuddy,
    #[serde(rename = "budge")]
    Budge,
    #[serde(rename = "changedetection")]
    Changedetection,
    #[serde(rename = "chatwoot")]
    Chatwoot,
    #[serde(rename = "classicpress-with-mariadb")]
    ClassicpressWithMariadb,
    #[serde(rename = "classicpress-with-mysql")]
    ClassicpressWithMysql,
    #[serde(rename = "classicpress-without-database")]
    ClassicpressWithoutDatabase,
    #[serde(rename = "cloudflared")]
    Cloudflared,
    #[serde(rename = "code-server")]
    CodeServer,
    #[serde(rename = "dashboard")]
    Dashboard,
    #[serde(rename = "directus")]
    Directus,
    #[serde(rename = "directus-with-postgresql")]
    DirectusWithPostgresql,
    #[serde(rename = "docker-registry")]
    DockerRegistry,
    #[serde(rename = "docuseal")]
    Docuseal,
    #[serde(rename = "docuseal-with-postgres")]
    DocusealWithPostgres,
    #[serde(rename = "dokuwiki")]
    Dokuwiki,
    #[serde(rename = "duplicati")]
    Duplicati,
    #[serde(rename = "emby")]
    Emby,
    #[serde(rename = "embystat")]
    Embystat,
    #[serde(rename = "fider")]
    Fider,
    #[serde(rename = "filebrowser")]
    Filebrowser,
    #[serde(rename = "firefly")]
    Firefly,
    #[serde(rename = "formbricks")]
    Formbricks,
    #[serde(rename = "ghost")]
    Ghost,
    #[serde(rename = "gitea")]
    Gitea,
    #[serde(rename = "gitea-with-mariadb")]
    GiteaWithMariadb,
    #[serde(rename = "gitea-with-mysql")]
    GiteaWithMysql,
    #[serde(rename = "gitea-with-postgresql")]
    GiteaWithPostgresql,
    #[serde(rename = "glance")]
    Glance,
    #[serde(rename = "glances")]
    Glances,
    #[serde(rename = "glitchtip")]
    Glitchtip,
    #[serde(rename = "grafana")]
    Grafana,
    #[serde(rename = "grafana-with-postgresql")]
    GrafanaWithPostgresql,
    #[serde(rename = "grocy")]
    Grocy,
    #[serde(rename = "heimdall")]
    Heimdall,
    #[serde(rename = "homepage")]
    Homepage,
    #[serde(rename = "jellyfin")]
    Jellyfin,
    #[serde(rename = "jenkins")]
    Jenkins,
    #[serde(rename = "kuzzle")]
    Kuzzle,
    #[serde(rename = "listmonk")]
    Listmonk,
    #[serde(rename = "logto")]
    Logto,
    #[serde(rename = "mediawiki")]
    Mediawiki,
    #[serde(rename = "meilisearch")]
    Meilisearch,
    #[serde(rename = "metabase")]
    Metabase,
    #[serde(rename = "metube")]
    Metube,
    #[serde(rename = "minio")]
    Minio,
    #[serde(rename = "moodle")]
    Moodle,
    #[serde(rename = "mosquitto")]
    Mosquitto,
    #[serde(rename = "n8n")]
    N8n,
    #[serde(rename = "n8n-with-postgresql")]
    N8nWithPostgresql,
    #[serde(rename = "next-image-transformation")]
    NextImageTransformation,
    #[serde(rename = "nextcloud")]
    Nextcloud,
    #[serde(rename = "nocodb")]
    Nocodb,
    #[serde(rename = "odoo")]
    Odoo,
    #[serde(rename = "openblocks")]
    Openblocks,
    #[serde(rename = "pairdrop")]
    Pairdrop,
    #[serde(rename = "penpot")]
    Penpot,
    #[serde(rename = "phpmyadmin")]
    Phpmyadmin,
    #[serde(rename = "pocketbase")]
    Pocketbase,
    #[serde(rename = "posthog")]
    Posthog,
    #[serde(rename = "reactive-resume")]
    ReactiveResume,
    #[serde(rename = "rocketchat")]
    Rocketchat,
    #[serde(rename = "shlink")]
    Shlink,
    #[serde(rename = "slash")]
    Slash,
    #[serde(rename = "snapdrop")]
    Snapdrop,
    #[serde(rename = "statusnook")]
    Statusnook,
    #[serde(rename = "stirling-pdf")]
    StirlingPdf,
    #[serde(rename = "supabase")]
    Supabase,
    #[serde(rename = "syncthing")]
    Syncthing,
    #[serde(rename = "tolgee")]
    Tolgee,
    #[serde(rename = "trigger")]
    Trigger,
    #[serde(rename = "trigger-with-external-database")]
    TriggerWithExternalDatabase,
    #[serde(rename = "twenty")]
    Twenty,
    #[serde(rename = "umami")]
    Umami,
    #[serde(rename = "unleash-with-postgresql")]
    UnleashWithPostgresql,
    #[serde(rename = "unleash-without-database")]
    UnleashWithoutDatabase,
    #[serde(rename = "uptime-kuma")]
    UptimeKuma,
    #[serde(rename = "vaultwarden")]
    Vaultwarden,
    #[serde(rename = "vikunja")]
    Vikunja,
    #[serde(rename = "weblate")]
    Weblate,
    #[serde(rename = "whoogle")]
    Whoogle,
    #[serde(rename = "wordpress-with-mariadb")]
    WordpressWithMariadb,
    #[serde(rename = "wordpress-with-mysql")]
    WordpressWithMysql,
    #[serde(rename = "wordpress-without-database")]
    WordpressWithoutDatabase,
}

impl Default for Type {
    fn default() -> Type {
        Self::Activepieces
    }
}

