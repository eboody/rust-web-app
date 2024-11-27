/*
 * Stirling PDF API
 *
 * API documentation for all Server-Side processing. Please note some functionality might be UI only and missing from here.
 *
 * The version of the OpenAPI document: 0.34.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use async_trait::async_trait;
use reqwest;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};

#[async_trait]
pub trait ConvertApi: Send + Sync {
    async fn convert_to_image<'file_input, 'image_format, 'single_or_multiple, 'color_type, 'dpi>(&self, file_input: Option<std::path::PathBuf>, image_format: Option<&'image_format str>, single_or_multiple: Option<&'single_or_multiple str>, color_type: Option<&'color_type str>, dpi: Option<&'dpi str>) -> Result<Vec<String>, Error<ConvertToImageError>>;
    async fn convert_to_pdf<'file_input, 'fit_option, 'color_type, 'auto_rotate>(&self, file_input: Option<Vec<std::path::PathBuf>>, fit_option: Option<&'fit_option str>, color_type: Option<&'color_type str>, auto_rotate: Option<bool>) -> Result<Vec<String>, Error<ConvertToPdfError>>;
    async fn html_to_pdf<'file_input, 'zoom>(&self, file_input: Option<std::path::PathBuf>, zoom: Option<f32>) -> Result<Vec<String>, Error<HtmlToPdfError>>;
    async fn markdown_to_pdf<'file_input>(&self, file_input: Option<std::path::PathBuf>) -> Result<Vec<String>, Error<MarkdownToPdfError>>;
    async fn pdf_to_csv<'file_input, 'page_id>(&self, file_input: Option<std::path::PathBuf>, page_id: Option<f64>) -> Result<String, Error<PdfToCsvError>>;
    async fn pdf_to_pdf_a<'file_input, 'output_format>(&self, file_input: Option<std::path::PathBuf>, output_format: Option<&'output_format str>) -> Result<Vec<String>, Error<PdfToPdfAError>>;
    async fn process_file_to_pdf<'file_input>(&self, file_input: Option<std::path::PathBuf>) -> Result<Vec<String>, Error<ProcessFileToPdfError>>;
    async fn process_pdf_to_html<'file_input>(&self, file_input: Option<std::path::PathBuf>) -> Result<Vec<String>, Error<ProcessPdfToHtmlError>>;
    async fn process_pdf_to_presentation<'file_input, 'output_format>(&self, file_input: Option<std::path::PathBuf>, output_format: Option<&'output_format str>) -> Result<Vec<String>, Error<ProcessPdfToPresentationError>>;
    async fn process_pdf_to_rt_for_txt<'file_input, 'output_format>(&self, file_input: Option<std::path::PathBuf>, output_format: Option<&'output_format str>) -> Result<Vec<String>, Error<ProcessPdfToRtForTxtError>>;
    async fn process_pdf_to_word<'file_input, 'output_format>(&self, file_input: Option<std::path::PathBuf>, output_format: Option<&'output_format str>) -> Result<Vec<String>, Error<ProcessPdfToWordError>>;
    async fn process_pdf_to_xml<'file_input>(&self, file_input: Option<std::path::PathBuf>) -> Result<Vec<String>, Error<ProcessPdfToXmlError>>;
    async fn url_to_pdf<'url_input>(&self, url_input: &'url_input str) -> Result<Vec<String>, Error<UrlToPdfError>>;
}

pub struct ConvertApiClient {
    configuration: Arc<configuration::Configuration>
}

impl ConvertApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}



#[async_trait]
impl ConvertApi for ConvertApiClient {
    /// This endpoint converts a PDF file to image(s) with the specified image format, color type, and DPI. Users can choose to get a single image or multiple images.  Input:PDF Output:Image Type:SI-Conditional
    async fn convert_to_image<'file_input, 'image_format, 'single_or_multiple, 'color_type, 'dpi>(&self, file_input: Option<std::path::PathBuf>, image_format: Option<&'image_format str>, single_or_multiple: Option<&'single_or_multiple str>, color_type: Option<&'color_type str>, dpi: Option<&'dpi str>) -> Result<Vec<String>, Error<ConvertToImageError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/api/v1/convert/pdf/img", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form = reqwest::multipart::Form::new();
        // TODO: support file upload for 'fileInput' parameter
        if let Some(local_var_param_value) = image_format {
            local_var_form = local_var_form.text("imageFormat", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = single_or_multiple {
            local_var_form = local_var_form.text("singleOrMultiple", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = color_type {
            local_var_form = local_var_form.text("colorType", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = dpi {
            local_var_form = local_var_form.text("dpi", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<ConvertToImageError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// This endpoint converts one or more images to a PDF file. Users can specify whether to stretch the images to fit the PDF page, and whether to automatically rotate the images. Input:Image Output:PDF Type:MISO
    async fn convert_to_pdf<'file_input, 'fit_option, 'color_type, 'auto_rotate>(&self, file_input: Option<Vec<std::path::PathBuf>>, fit_option: Option<&'fit_option str>, color_type: Option<&'color_type str>, auto_rotate: Option<bool>) -> Result<Vec<String>, Error<ConvertToPdfError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/api/v1/convert/img/pdf", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form = reqwest::multipart::Form::new();
        // TODO: support file upload for 'fileInput' parameter
        if let Some(local_var_param_value) = fit_option {
            local_var_form = local_var_form.text("fitOption", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = color_type {
            local_var_form = local_var_form.text("colorType", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = auto_rotate {
            local_var_form = local_var_form.text("autoRotate", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<ConvertToPdfError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// This endpoint takes an HTML or ZIP file input and converts it to a PDF format.
    async fn html_to_pdf<'file_input, 'zoom>(&self, file_input: Option<std::path::PathBuf>, zoom: Option<f32>) -> Result<Vec<String>, Error<HtmlToPdfError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/api/v1/convert/html/pdf", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form = reqwest::multipart::Form::new();
        // TODO: support file upload for 'fileInput' parameter
        if let Some(local_var_param_value) = zoom {
            local_var_form = local_var_form.text("zoom", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<HtmlToPdfError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// This endpoint takes a Markdown file input, converts it to HTML, and then to PDF format. Input:MARKDOWN Output:PDF Type:SISO
    async fn markdown_to_pdf<'file_input>(&self, file_input: Option<std::path::PathBuf>) -> Result<Vec<String>, Error<MarkdownToPdfError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/api/v1/convert/markdown/pdf", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let local_var_form = reqwest::multipart::Form::new();
        // TODO: support file upload for 'fileInput' parameter
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<MarkdownToPdfError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// This operation takes an input PDF file and returns CSV file of whole page. Input:PDF Output:CSV Type:SISO
    async fn pdf_to_csv<'file_input, 'page_id>(&self, file_input: Option<std::path::PathBuf>, page_id: Option<f64>) -> Result<String, Error<PdfToCsvError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/api/v1/convert/pdf/csv", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form = reqwest::multipart::Form::new();
        // TODO: support file upload for 'fileInput' parameter
        if let Some(local_var_param_value) = page_id {
            local_var_form = local_var_form.text("pageId", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<PdfToCsvError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// This endpoint converts a PDF file to a PDF/A file. PDF/A is a format designed for long-term archiving of digital documents. Input:PDF Output:PDF Type:SISO
    async fn pdf_to_pdf_a<'file_input, 'output_format>(&self, file_input: Option<std::path::PathBuf>, output_format: Option<&'output_format str>) -> Result<Vec<String>, Error<PdfToPdfAError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/api/v1/convert/pdf/pdfa", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form = reqwest::multipart::Form::new();
        // TODO: support file upload for 'fileInput' parameter
        if let Some(local_var_param_value) = output_format {
            local_var_form = local_var_form.text("outputFormat", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<PdfToPdfAError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// This endpoint converts a given file to a PDF using LibreOffice API  Input:ANY Output:PDF Type:SISO
    async fn process_file_to_pdf<'file_input>(&self, file_input: Option<std::path::PathBuf>) -> Result<Vec<String>, Error<ProcessFileToPdfError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/api/v1/convert/file/pdf", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let local_var_form = reqwest::multipart::Form::new();
        // TODO: support file upload for 'fileInput' parameter
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<ProcessFileToPdfError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// This endpoint converts a PDF file to HTML format. Input:PDF Output:HTML Type:SISO
    async fn process_pdf_to_html<'file_input>(&self, file_input: Option<std::path::PathBuf>) -> Result<Vec<String>, Error<ProcessPdfToHtmlError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/api/v1/convert/pdf/html", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let local_var_form = reqwest::multipart::Form::new();
        // TODO: support file upload for 'fileInput' parameter
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<ProcessPdfToHtmlError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// This endpoint converts a given PDF file to a Presentation format. Input:PDF Output:PPT Type:SISO
    async fn process_pdf_to_presentation<'file_input, 'output_format>(&self, file_input: Option<std::path::PathBuf>, output_format: Option<&'output_format str>) -> Result<Vec<String>, Error<ProcessPdfToPresentationError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/api/v1/convert/pdf/presentation", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form = reqwest::multipart::Form::new();
        // TODO: support file upload for 'fileInput' parameter
        if let Some(local_var_param_value) = output_format {
            local_var_form = local_var_form.text("outputFormat", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<ProcessPdfToPresentationError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// This endpoint converts a given PDF file to Text or RTF format. Input:PDF Output:TXT Type:SISO
    async fn process_pdf_to_rt_for_txt<'file_input, 'output_format>(&self, file_input: Option<std::path::PathBuf>, output_format: Option<&'output_format str>) -> Result<Vec<String>, Error<ProcessPdfToRtForTxtError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/api/v1/convert/pdf/text", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form = reqwest::multipart::Form::new();
        // TODO: support file upload for 'fileInput' parameter
        if let Some(local_var_param_value) = output_format {
            local_var_form = local_var_form.text("outputFormat", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<ProcessPdfToRtForTxtError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// This endpoint converts a given PDF file to a Word document format. Input:PDF Output:WORD Type:SISO
    async fn process_pdf_to_word<'file_input, 'output_format>(&self, file_input: Option<std::path::PathBuf>, output_format: Option<&'output_format str>) -> Result<Vec<String>, Error<ProcessPdfToWordError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/api/v1/convert/pdf/word", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form = reqwest::multipart::Form::new();
        // TODO: support file upload for 'fileInput' parameter
        if let Some(local_var_param_value) = output_format {
            local_var_form = local_var_form.text("outputFormat", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<ProcessPdfToWordError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// This endpoint converts a PDF file to an XML file. Input:PDF Output:XML Type:SISO
    async fn process_pdf_to_xml<'file_input>(&self, file_input: Option<std::path::PathBuf>) -> Result<Vec<String>, Error<ProcessPdfToXmlError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/api/v1/convert/pdf/xml", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let local_var_form = reqwest::multipart::Form::new();
        // TODO: support file upload for 'fileInput' parameter
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<ProcessPdfToXmlError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// This endpoint fetches content from a URL and converts it to a PDF format. Input:N/A Output:PDF Type:SISO
    async fn url_to_pdf<'url_input>(&self, url_input: &'url_input str) -> Result<Vec<String>, Error<UrlToPdfError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/api/v1/convert/url/pdf", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form = reqwest::multipart::Form::new();
        local_var_form = local_var_form.text("urlInput", url_input.to_string());
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<UrlToPdfError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

}

/// struct for typed errors of method [`convert_to_image`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConvertToImageError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`convert_to_pdf`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConvertToPdfError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`html_to_pdf`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HtmlToPdfError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`markdown_to_pdf`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MarkdownToPdfError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`pdf_to_csv`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PdfToCsvError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`pdf_to_pdf_a`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PdfToPdfAError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`process_file_to_pdf`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProcessFileToPdfError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`process_pdf_to_html`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProcessPdfToHtmlError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`process_pdf_to_presentation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProcessPdfToPresentationError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`process_pdf_to_rt_for_txt`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProcessPdfToRtForTxtError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`process_pdf_to_word`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProcessPdfToWordError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`process_pdf_to_xml`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProcessPdfToXmlError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`url_to_pdf`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UrlToPdfError {
    UnknownValue(serde_json::Value),
}

