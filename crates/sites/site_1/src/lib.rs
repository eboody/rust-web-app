mod config;
mod error;
use axum::{
	extract::Path,
	http::{HeaderMap, HeaderValue},
	response::{Html, IntoResponse, Response},
	routing::{get, post},
	Router,
};
use config::web_config;
use error::Error;
use inline_css::css;
use lib_directus::Ebook;
use maud::{html, Markup, PreEscaped, Render, DOCTYPE};
use reqwest::{header::AUTHORIZATION, Client, StatusCode};
use serde_json::Value;

pub fn router() -> Router {
	Router::new()
		.route("/", get(get_slash))
		.route("/image/:id", get(get_image))
}

async fn get_slash() -> Markup {
	page(ebooks_menu().await)
}

pub fn page(children: Markup) -> Markup {
	let folder = &web_config().WEB_FOLDER;

	let manifest: Value = serde_json::from_str(
		&std::fs::read_to_string(format!("{}/js/.vite/manifest.json", folder))
			.expect("manifest.json not found"),
	)
	.expect("Error parsing manifest.json");

	let main_js = manifest["index.html"]["file"]
		.as_str()
		.expect("index.html not found in manifest");
	let _main_css = manifest["index.html"]["css"][0]
		.as_str()
		.expect("index.css not found in manifest");

	html! {
		(DOCTYPE)
		html {
			head {
				title { "SITE1" }
				script type="module" src=(format!("/js/{}", main_js)) {}
				script src="https://unpkg.com/htmx.org@2.0.3" {}
				script src="https://cdn.jsdelivr.net/gh/gnat/css-scope-inline@main/script.js" {}
				script src="https://cdn.jsdelivr.net/gh/gnat/surreal@main/surreal.js" {}
				link rel="stylesheet" href="https://fonts.cdnfonts.com/css/minion-pro" {}
			}
			body hx-boost="true" {
				div id="app" x-data="app" {
					(children)
				}
			}
		}
	}
}
async fn ebooks_menu() -> Markup {
	let ebooks = lib_directus::get_ebooks().await.unwrap();

	let styles = PreEscaped(
		r#"
								me { margin: 30px; }

								.grid-auto-fit {
									display: grid;
									gap: 3rem;
									grid-template-columns: repeat(auto-fit, minmax(min(45ch, 100%), 1fr));

									container: grid-auto-fit / inline-size;
								}

								@layer card-styling {
									.card {
										--padding: 1rem;

										display: grid;
										overflow: hidden;
										background-color: white;
										border-radius: 4px;

										h2,
										h3 {
											color: black;
										}

										> img {
											object-fit: cover;
											width: 100%;
											height: 100%;
										}

										> :not(img) {
											margin-block-start: 0;
											margin-inline: 1rem;
										}

										> :not(img):first-child {
											margin-block-start: 1rem;
										}

										> :not(img):last-child {
											margin-block-end: 1rem;
										}
									}
								}



								@layer layout {
									.primary-layout {
										display: grid;
										grid-template-columns:
											[full-width] minmax(2rem, 1fr) [content-start] min(1600px, 100% - 4rem)
											[content-end] minmax(2rem, 1fr);

										& > * {
											grid-column: 2 / -2;
										}
									}
								}
	"#,
	);

	html! {
				div.ebook-container {
					style { (styles) }

					section {
						div.grid-auto-fit {
							@for ebook in ebooks {
								(ebook_card(&ebook))
							}
						}
					}

				}
	}
}

pub fn ebook_card(ebook: &Ebook) -> Markup {
	let styles = PreEscaped(
		r#"
                me {
									article.card {
                    box-shadow: rgba(0, 0, 0, 0.05) 0px 6px 24px 0px, rgba(0, 0, 0, 0.08) 0px 0px 0px 1px;
                    font-family: 'Capitolina', serif; text-align: center;
                    padding: 1rem;
                    transition: transform 0.2s, box-shadow 0.2s;

										@container grid-auto-fit (inline-size < calc(40ch * 2 + 1rem)) {
											.book-3d {
													max-width: 200px;
													--book-thickness: 15px;
											}
										}
									}
                article.card:hover {
                    transform: scale(1.01);
                    box-shadow: rgba(0, 0, 0, 0.05) 0px 6px 24px 0px, rgba(0, 0, 0, 0.08) 0px 0px 0px 1px,
                    rgba(0, 0, 0, 0.1) 0px 4px 6px -1px, rgba(0, 0, 0, 0.06) 0px 2px 4px -1px;
                }
								.subtext {
                    font-size: 20px;
                    line-height: 27px;
                    font-weight: 400;
                    color: #505050;
                    font-family: 'MyriadPro', sans-serif;
                    padding: 0 1rem 1rem 1rem;
                }
                a.download {
                    background-color: #4683ec;
                    color: white;
                    border-radius: 4px;
                    text-decoration: none;
                    display: inline-block;
                    text-align: center;
                    max-width: 50%;
                    font-family: 'MyriadPro', sans-serif;
                    font-size: 24px;
                    letter-spacing: 1.5px;
                    font-weight: 600;
                    padding: 0.5rem 1rem 0.25rem 1rem;
                    box-shadow: rgba(60, 64, 67, 0.3) 0px 1px 2px 0px, rgba(60, 64, 67, 0.15) 0px 1px 3px 1px;
                    margin-top: 1rem;
                }
								article.card:has(> img) {

									border: 2px solid var(--clr-primary-300);
									box-shadow: rgba(0, 0, 0, 0.12) 0px 1px 3px, rgba(0, 0, 0, 0.24) 0px 1px 2px;


									@container grid-auto-fit (inline-size > calc(30ch * 2 + 1rem)) {
										grid-column: span 2;

										display: grid;
										grid-template-columns: subgrid;
										gap: 0;

										> img {
											grid-column: 2;
											grid-row: 1 / 4;
										}
									}

									@container grid-auto-fit (inline-size > calc(30ch * 4 + 3rem)) {

										grid-column: span 2;
										grid-row: span 2;

										> :not(img) {
											grid-column: 1 / -1;
										}

										> img {
											grid-column: 1 / -1;
											grid-row: 1;
										}
									}
								}
							}
	"#,
	);

	html! {
		article.card
		hx_patch=(format!("https://tosapp.eman.network/ebooks/{}", ebook.id))
		hx_trigger="mouseover" {
			(ebook_image(ebook))
			h2 { (ebook.name) }
			p.subtext { (ebook.sub_text.clone().unwrap_or("".to_owned())) }
			a.download href=(ebook.get_file_download()) { "Download" }
		}
		style { (styles) }
	}
}

pub fn ebook_image(ebook: &Ebook) -> Markup {
	let styles = r#"
        me {
					.book {
							--book-thickness: 30px;
							--cover-color: slategray;
							perspective: 1000px;
							max-width: 250px;
							margin: 55px auto;
							transition: max-width 0.3s, --book-thickness 0.3s;
					}
					@keyframes book-3d {
							from { transform: rotateY(-15deg); }
							to   { transform: rotateY(-25deg); }
					}
					@keyframes book-3d-back {
							from { transform: rotateY(-25deg); }
							to   { transform: rotateY(-15deg); }
					}
					.book:hover .inner {
							animation: book-3d-back .3s ease-out forwards;
					}
					.book img {
							display: block;
							width: 100%;
							height: auto;
							border-radius: 0px 2px 2px 0px;
							transform: translateZ(var(--book-thickness));
							box-shadow: 5px 5px 20px rgba(0, 0, 0, 0.1);
					}
					.book::after {
							content: "";
							position: absolute;
							inset: 1px;
							height: 99%;
							border-radius: 3px;
							pointer-events: none;
							background: linear-gradient(
									90deg,
									rgba(0, 0, 0, 0.118) 0.65%,
									rgba(255, 255, 255, 0.2) 1.53%,
									rgba(255, 255, 255, 0.1) 2.38%,
									rgba(0, 0, 0, 0.05) 3.26%,
									rgba(255, 255, 255, 0.14) 5.68%,
									rgba(244, 244, 244, 0) 6.96%
							);
					}
					.inner::after {
						content: '';
						position: absolute;
						top: 0;
						left: 1%;
						width: 100%;
						height: 100%;
						transform: translateZ(calc(var(--book-thickness) * -1));
						background-color: var(--cover-color);
						border-radius: 0 2px 2px 0;
						box-shadow: -10px 0 50px 10px rgba(0,0,0, 0.3);
					}
					.inner::before {
						position: absolute;
						content: ' ';
						left: 100%;
						top: 1%;
						width: calc(var(--book-thickness) * 2);
						height: 98%;
						transform: translate(-55%,0) rotateY(90deg);
						background: linear-gradient(90deg, #fff 0%, hsl(0, 0%, 94%) 5%, #fff 10%, hsl(0, 0%, 94%) 15%, #fff 20%, hsl(0, 0%, 94%) 25%, #fff 30%, hsl(0, 0%, 94%) 35%, #fff 40%, hsl(0, 0%, 94%) 45%, #fff 50%, hsl(0, 0%, 94%) 55%, #fff 60%, hsl(0, 0%, 94%) 65%, #fff 70%, hsl(0, 0%, 94%) 75%, #fff 80%, hsl(0, 0%, 94%) 85%, #fff 90%, hsl(0, 0%, 94%) 95%, #fff 100%);
					}
					.inner {
						position: relative;
						transform-style: preserve-3d;
						transform: rotateY(-15deg);
					}
    "#;

	let styles = PreEscaped(format!(
		"{}
		.inner {{
			animation: book-3d 1s ease 0.{}s forwards;
		}}
		}}",
		styles,
		ebook.id / 3
	));

	html! {
		div.book {
			div.inner {
				img.cover
					src=(ebook.get_cover_image())
					alt=(ebook.name);
			}
		}
		style { (styles) }
	}
}

pub async fn get_image(
	Path(image_id): Path<String>,
) -> Result<Response, StatusCode> {
	let cover_image_url =
		format!("https://directus.eman.network/assets/{}", image_id);

	// Fetch the DIRECTUS_TOKEN from environment variables
	let token = std::env::var("DIRECTUS_TOKEN")
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	let client = Client::new();

	// Fetch the image from Directus
	let image_response = client
		.get(&cover_image_url)
		.bearer_auth(&token)
		.send()
		.await
		.map_err(|_| StatusCode::BAD_GATEWAY)?;

	if !image_response.status().is_success() {
		// Return a 404 if the image is not found
		return Err(StatusCode::NOT_FOUND);
	}

	// Get the content type from the Directus response headers
	let content_type = image_response
		.headers()
		.get(reqwest::header::CONTENT_TYPE)
		.cloned()
		.unwrap_or_else(|| HeaderValue::from_static("application/octet-stream"));

	// Read the image bytes
	let image_bytes = image_response
		.bytes()
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	// Build the response to send back to the client
	let response = Response::builder()
		.status(StatusCode::OK)
		.header(axum::http::header::CONTENT_TYPE, content_type)
		.body(image_bytes)
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	Ok(response.into_body().into_response())
}
