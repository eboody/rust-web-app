CREATE TABLE "sqlx_migrations" (
"version" bigint NOT NULL PRIMARY KEY,
"description" character varying NOT NULL,
"installed_on" timestamp NOT NULL,
"success" boolean NOT NULL,
"checksum" bytea NOT NULL,
"execution_time" bigint NOT NULL
);

-- ALTER TABLE "directus_collections" ALTER COLUMN "translations" TYPE jsonb USING "translations"::jsonb, ALTER COLUMN "item_duplication_fields" TYPE jsonb USING "item_duplication_fields"::jsonb;
-- ALTER TABLE "directus_notifications" -- ALTER COLUMN "timestamp" TYPE character varying USING "timestamp"::character varying;
-- ALTER TABLE "directus_webhooks" -- ALTER COLUMN "headers" TYPE jsonb USING "headers"::jsonb;
-- ALTER TABLE "ebooks" -- ALTER COLUMN "date_created" TYPE character varying USING "date_created"::character varying, -- ALTER COLUMN "date_updated" TYPE character varying USING "date_updated"::character varying, -- ALTER COLUMN "date_published" TYPE character varying USING "date_published"::character varying;
-- ALTER TABLE "ebooks_translations" -- ALTER COLUMN "ebooks_id" SET NOT NULL;
-- ALTER TABLE "directus_dashboards" -- ALTER COLUMN "date_created" TYPE character varying USING "date_created"::character varying;
-- ALTER TABLE "spatial_ref_sys" ADD COLUMN "proj_4_text" character varying;
-- ALTER TABLE "directus_files" -- ALTER COLUMN "metadata" TYPE jsonb USING "metadata"::jsonb, -- ALTER COLUMN "tus_data" TYPE jsonb USING "tus_data"::jsonb;
-- ALTER TABLE "directus_users" -- ALTER COLUMN "tags" TYPE jsonb USING "tags"::jsonb, -- ALTER COLUMN "last_access" TYPE character varying USING "last_access"::character varying, -- ALTER COLUMN "auth_data" TYPE jsonb USING "auth_data"::jsonb, -- ALTER COLUMN "theme_light_overrides" TYPE jsonb USING "theme_light_overrides"::jsonb, -- ALTER COLUMN "theme_dark_overrides" TYPE jsonb USING "theme_dark_overrides"::jsonb;
-- ALTER TABLE "directus_activity" -- ALTER COLUMN "timestamp" TYPE character varying USING "timestamp"::character varying;
-- ALTER TABLE "directus_versions" -- ALTER COLUMN "date_created" TYPE character varying USING "date_created"::character varying, -- ALTER COLUMN "date_updated" TYPE character varying USING "date_updated"::character varying, -- ALTER COLUMN "delta" TYPE jsonb USING "delta"::jsonb;
-- ALTER TABLE "directus_sessions" -- ALTER COLUMN "expires" TYPE character varying USING "expires"::character varying;
-- ALTER TABLE "directus_operations" -- ALTER COLUMN "options" TYPE jsonb USING "options"::jsonb, -- ALTER COLUMN "date_created" TYPE character varying USING "date_created"::character varying;
-- ALTER TABLE "directus_comments" -- ALTER COLUMN "date_created" TYPE character varying USING "date_created"::character varying, -- ALTER COLUMN "date_updated" TYPE character varying USING "date_updated"::character varying;
-- ALTER TABLE "directus_shares" -- ALTER COLUMN "date_created" TYPE character varying USING "date_created"::character varying, -- ALTER COLUMN "date_start" TYPE character varying USING "date_start"::character varying, -- ALTER COLUMN "date_end" TYPE character varying USING "date_end"::character varying;
-- ALTER TABLE "directus_settings" -- ALTER COLUMN "storage_asset_presets" TYPE jsonb USING "storage_asset_presets"::jsonb, -- ALTER COLUMN "basemaps" TYPE jsonb USING "basemaps"::jsonb, -- ALTER COLUMN "module_bar" TYPE jsonb USING "module_bar"::jsonb, -- ALTER COLUMN "custom_aspect_ratios" TYPE jsonb USING "custom_aspect_ratios"::jsonb, -- ALTER COLUMN "theme_light_overrides" TYPE jsonb USING "theme_light_overrides"::jsonb, -- ALTER COLUMN "theme_dark_overrides" TYPE jsonb USING "theme_dark_overrides"::jsonb, -- ALTER COLUMN "public_registration_email_filter" TYPE jsonb USING "public_registration_email_filter"::jsonb;
-- ALTER TABLE "directus_permissions" -- ALTER COLUMN "permissions" TYPE jsonb USING "permissions"::jsonb, -- ALTER COLUMN "validation" TYPE jsonb USING "validation"::jsonb, -- ALTER COLUMN "presets" TYPE jsonb USING "presets"::jsonb;
-- ALTER TABLE "directus_migrations" -- ALTER COLUMN "timestamp" TYPE character varying USING "timestamp"::character varying;
-- ALTER TABLE "articles_translations" -- ALTER COLUMN "articles_id" TYPE uuid USING "articles_id"::uuid;
-- ALTER TABLE "articles" ADD COLUMN "thing" jsonb;
-- ALTER TABLE "articles" -- ALTER COLUMN "thing" SET NOT NULL;
-- ALTER TABLE "articles" -- ALTER COLUMN "id" TYPE uuid USING "id"::uuid, -- ALTER COLUMN "date_created" TYPE character varying USING "date_created"::character varying, -- ALTER COLUMN "date_updated" TYPE character varying USING "date_updated"::character varying, ADD COLUMN "image" uuid, ADD COLUMN "author" uuid;
-- ALTER TABLE "directus_fields" -- ALTER COLUMN "options" TYPE jsonb USING "options"::jsonb, -- ALTER COLUMN "display_options" TYPE jsonb USING "display_options"::jsonb, -- ALTER COLUMN "translations" TYPE jsonb USING "translations"::jsonb, -- ALTER COLUMN "conditions" TYPE jsonb USING "conditions"::jsonb, -- ALTER COLUMN "validation" TYPE jsonb USING "validation"::jsonb;
-- ALTER TABLE "directus_presets" -- ALTER COLUMN "layout_query" TYPE jsonb USING "layout_query"::jsonb, -- ALTER COLUMN "layout_options" TYPE jsonb USING "layout_options"::jsonb, -- ALTER COLUMN "filter" TYPE jsonb USING "filter"::jsonb;
-- ALTER TABLE "directus_revisions" -- ALTER COLUMN "data" TYPE jsonb USING "data"::jsonb, -- ALTER COLUMN "delta" TYPE jsonb USING "delta"::jsonb;
-- ALTER TABLE "directus_flows" -- ALTER COLUMN "options" TYPE jsonb USING "options"::jsonb, -- ALTER COLUMN "date_created" TYPE character varying USING "date_created"::character varying;
-- ALTER TABLE "directus_panels" -- ALTER COLUMN "options" TYPE jsonb USING "options"::jsonb, -- ALTER COLUMN "date_created" TYPE character varying USING "date_created"::character varying;
