mod config;
mod error;
use config::web_config;
use lib_directus::Ebook;
pub use ryde::*;
use serde_json::Value;

pub fn router() -> Router {
	Router::new()
		.route("/", get(get_slash))
		.route("/ebooks_menu", get(ebooks_menu))
		.route("/ebooks/:id", patch(increment_ebook_hover))
}

pub async fn increment_ebook_hover(
	Path(id): Path<u32>,
) -> core::result::Result<(), Error> {
	let url = format!("https://directus.eman.network/items/eBooks/{}", id);
	let client = reqwest::Client::new();
	let ebook = client.get(&url).send().await?.json::<Ebook>().await?;
	let response = client
		.patch(&url)
		.json(&serde_json::json!({
			"times_hovered_over": ebook.times_hovered_over + 1
		}))
		.send()
		.await?;
	let text = response.text().await?;
	println!("{:#?}", text);

	Ok(())
}

async fn get_slash() -> Html {
	html! {
		<Page>
			<div hx-get="/ebooks_menu" hx-swap="outerHTML" hx-trigger="load"></div>
		</Page>
	}
}

async fn ebooks_menu() -> Component {
	let ebooks = lib_directus::get_ebooks().await.unwrap();
	html! {
				<div class="ebook-container">
					<style>
								"
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
							"
					</style>

					<section>
						<div class="grid-auto-fit">
								{
									ebooks.iter().map(|ebook| html! {
										<EbookCard ebook=&ebook />
									}).collect::<Vec<_>>()
								}
						</div>
					</section>

				</div>
	}
}

#[allow(non_snake_case)]
pub fn EbookImage(ebook: &Ebook) -> Component {
	html! {
	<div class="book-3d">
			<style>"
				me {
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

				me:hover .book-3d__inner {
						animation: book-3d-back .3s ease-out forwards;
				}

				me img {
					display:block;
					width:100%;
					height: auto;
					border-radius: 0px 2px 2px 0px;
					transform: translateZ( var(--book-thickness) );
					box-shadow: 5px 5px 20px rgba(0,0,0, 0.1);
				}

				me::after{
					content: \"\";
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
			"</style>
	  <div class="book-3d__inner">
			<img class="book-3d__cover" src={ebook.get_cover_image()} alt=ebook.name>
			<style>
				{format!("
						me {{
							position: relative;
							transform-style: preserve-3d;
							animation: book-3d 1s ease 0.{}s forwards;
							transform: rotateY(-15deg);
						}}
				", ebook.id / 3)}
				"

				me::after {
					content: '';
					position: absolute;
					top: 0;
					left: 1%;
					width: 100%;
					height: 100%;
					transform: translateZ( calc( var(--book-thickness) * -1 ) );
					background-color: var(--cover-color);
					border-radius: 0 2px 2px 0;
					box-shadow: -10px 0 50px 10px rgba(0,0,0, 0.3);
				}
				me::before {
					position: absolute;
					content: ' ';
					left: 100%;
					top: 1%;
					width: calc( var(--book-thickness) * 2 );
					height: 98%;
					transform: translate(-55%,0) rotateY( 90deg );
					background: linear-gradient( 90deg , #fff 0%, hsl(0, 0%, 94%) 5%, #fff 10%, hsl(0, 0%, 94%) 15%, #fff 20%, hsl(0, 0%, 94%) 25%, #fff 30%, hsl(0, 0%, 94%) 35%, #fff 40%, hsl(0, 0%, 94%) 45%, #fff 50%, hsl(0, 0%, 94%) 55%, #fff 60%, hsl(0, 0%, 94%) 65%, #fff 70%, hsl(0, 0%, 94%) 75%, #fff 80%, hsl(0, 0%, 94%) 85%, #fff 90%, hsl(0, 0%, 94%) 95%, #fff 100% );
				}
				"
			</style>
	  </div>
	</div>
			}
}

#[allow(non_snake_case)]
pub fn EbookCard(ebook: &Ebook) -> Component {
	html! {
			  <article class="card" hx-patch=format!("https://tosapp.eman.network/ebooks/{}", ebook.id) hx-trigger="mouseover">
					<style>
						"
						me { 
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
						me:hover {
								transform: scale(1.01);
								box-shadow: rgba(0, 0, 0, 0.05) 0px 6px 24px 0px, rgba(0, 0, 0, 0.08) 0px 0px 0px 1px,
 rgba(0, 0, 0, 0.1) 0px 4px 6px -1px, rgba(0, 0, 0, 0.06) 0px 2px 4px -1px;
						}

						me:has(> img) {

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
						"	
					</style>
					<article>
						<EbookImage ebook=ebook />
						<h2>{&ebook.name}</h2>
						<p class="subtext">{&ebook.sub_text}
							<style>
							"me {
								font-size: 20px;
								line-height: 27px;
								font-weight: 400;
								margin: 0;
								color: #505050;
								font-family: \"MyriadPro\", sans-serif;
								padding: 0 1rem 1rem 1rem;
							}"
							</style>
						</p>
						<a href=ebook.get_file_download()>
							Download
							<style>
								"
								me {
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
								"
							</style>
						</a>
						<style>
							"
							me {
								align-items: center;
								display: flex;
								flex-direction: column;
								justify-content: space-evenly;
							}
							"
						</style>
					</article>
		  </article>
	}
}

#[allow(non_snake_case)]
pub fn Page(els: Elements) -> Component {
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
		<!DOCTYPE html>
		<html>
			<head>
				<script type="module" src={format!("/js/{}", main_js)}></script>
				<script src="https://unpkg.com/htmx.org@2.0.3"></script>
				<script src="https://cdn.jsdelivr.net/gh/gnat/css-scope-inline@main/script.js"></script>
				<script src="https://cdn.jsdelivr.net/gh/gnat/surreal@main/surreal.js"></script>
				<link href="https://fonts.cdnfonts.com/css/minion-pro" rel="stylesheet">
				<style>
							"
								@font-face {
									font-family: 'Capitolina';
									src: url('https://theobjectivestandard.com/wp-content/themes/tos/dist/fonts/capitolina/CapitolinaW03-Bold.woff2') format('woff2');
									font-weight: bold;
									font-style: normal;
								}
								@font-face {
									font-family: 'MyriadPro';
									src: url('https://theobjectivestandard.com/wp-content/themes/tos/dist/fonts/myriadPro/MyriadPro-Cond.woff2') format('woff2');
									font-weight: bold;
									font-style: normal;
								}
								@layer theme {
									:root {
										/* Neutral Colors */
										--clr-neutral-900: #111; /* Dark gray/black for text and backgrounds */
										--clr-neutral-800: #333; /* Darker gray for secondary text */
										--clr-neutral-700: #555; /* Medium gray for subtext or borders */
										--clr-neutral-200: #f5f5f5; /* Light gray background color */
										--clr-neutral-100: #fff; /* Pure white for backgrounds and cards */

										/* Primary Blue Colors */
										--clr-primary-600: #003366; /* Dark blue for headers and accents */
										--clr-primary-500: #004080; /* Slightly lighter blue for active links */
										--clr-primary-400: #0059b3; /* Standard blue for buttons and links */
										--clr-primary-300: #0066cc; /* Bright blue for highlights or hover states */
										--clr-primary-200: #0073e6; /* Light blue, possible for hover effects or subtler accents */

										/* Secondary/Accent Color */
										--clr-accent: #c71e1d; /* Red color for warnings or highlights (based on the "Donate" button) */

										/* Typography Colors */
										--clr-text-primary: var(--clr-neutral-900); /* Primary text color */
										--clr-text-secondary: var(--clr-neutral-800); /* Secondary text color */
										--clr-link: var(--clr-primary-400); /* Link color */
										--clr-link-hover: var(--clr-primary-500); /* Hover state for links */
									}
								}
							"
				</style>
				<title>SITE1</title>
			</head>
			<body hx-boost=true>
				<div id="app" x-data="app">
					{els}
				</div>
			</body>
		</html>
	}
}
