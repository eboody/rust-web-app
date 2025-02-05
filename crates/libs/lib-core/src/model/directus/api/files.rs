use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};
use uuid::Uuid;

#[serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Files {
    /// Unique identifier for the file.
    pub id: Uuid,
    /// Where the file is stored. Either `local` for the local filesystem or the name of the storage adapter (for example `s3`).
    #[serde(rename = "storage", skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,
    /// Name of the file on disk. By default, Directus uses a random hash for the filename.
    #[serde(rename = "filename_disk", skip_serializing_if = "Option::is_none")]
    pub filename_disk: Option<String>,
    /// How you want to the file to be named when it's being downloaded.
    #[serde(rename = "filename_download", skip_serializing_if = "Option::is_none")]
    pub filename_download: Option<String>,
    /// Title for the file. Is extracted from the filename on upload, but can be edited by the user.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// MIME type of the file.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(
        rename = "folder",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub folder: Option<Option<Box<super::UpdateFileRequestFolder>>>,
    #[serde(rename = "uploaded_by", skip_serializing_if = "Option::is_none")]
    pub uploaded_by: Option<Box<super::FilesUploadedBy>>,
    /// When the file was created.
    #[serde(rename = "created_on", skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,
    #[serde(
        rename = "modified_by",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub modified_by: Option<Option<Box<super::FilesModifiedBy>>>,
    #[serde(rename = "modified_on", skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<String>,
    /// Character set of the file.
    #[serde(
        rename = "charset",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub charset: Option<Option<String>>,
    /// Size of the file in bytes.
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(rename = "filesize", skip_serializing_if = "Option::is_none")]
    pub filesize: Option<i32>,
    /// Width of the file in pixels. Only applies to images.
    #[serde(
        rename = "width",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub width: Option<Option<i32>>,
    /// Height of the file in pixels. Only applies to images.
    #[serde(
        rename = "height",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub height: Option<Option<i32>>,
    /// Duration of the file in seconds. Only applies to audio and video.
    #[serde(
        rename = "duration",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub duration: Option<Option<i32>>,
    /// Where the file was embedded from.
    #[serde(
        rename = "embed",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub embed: Option<Option<String>>,
    /// Description for the file.
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<Option<String>>,
    /// Where the file was created. Is automatically populated based on Exif data for images.
    #[serde(
        rename = "location",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub location: Option<Option<String>>,
    /// Tags for the file. Is automatically populated based on Exif data for images.
    #[serde(
        rename = "tags",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tags: Option<Option<Vec<String>>>,
    /// IPTC, Exif, and ICC metadata extracted from file
    #[serde(
        rename = "metadata",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata: Option<Option<json::Value>>,
    #[serde(
        rename = "focal_point_x",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub focal_point_x: Option<Option<i32>>,
    #[serde(
        rename = "focal_point_y",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub focal_point_y: Option<Option<i32>>,
    #[serde(
        rename = "tus_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tus_id: Option<Option<String>>,
    #[serde(
        rename = "tus_data",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tus_data: Option<Option<json::Value>>,
    /// When the file was last uploaded/replaced.
    #[serde(rename = "uploaded_on", skip_serializing_if = "Option::is_none")]
    pub uploaded_on: Option<String>,
}

impl Files {
    pub fn new(id: Uuid) -> Files {
        Files {
            id,
            storage: None,
            filename_disk: None,
            filename_download: None,
            title: None,
            r#type: None,
            folder: None,
            uploaded_by: None,
            created_on: None,
            modified_by: None,
            modified_on: None,
            charset: None,
            filesize: None,
            width: None,
            height: None,
            duration: None,
            embed: None,
            description: None,
            location: None,
            tags: None,
            metadata: None,
            focal_point_x: None,
            focal_point_y: None,
            tus_id: None,
            tus_data: None,
            uploaded_on: None,
        }
    }
}
