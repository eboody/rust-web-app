use crate::prelude::*;
use lib_core::model::Ebook;

pub struct PopupSignupForm<'a> {
	pub ebook: &'a Ebook,
}

impl Render for PopupSignupForm<'_> {
	fn render(&self) -> Markup {
		let app_url = std::env::var("APP_URL")
			.unwrap_or_else(|_| "https://tos.eman.network".to_string());

		let url = format!("{}/ebooks/signup", app_url);

		html! {
			form
			hx-post=(url)
			hx-swap="outerHTML"
			hx-target="#popup"
			hx-trigger="submit" {
				(FormField {
					name: "first_name",
					label: Some("First Name"),
					input_type: "text",
					placeholder: Some("First Name"),
					value: None
				})
				(FormField {
					name: "email",
					label: Some("Email"),
					input_type: "email",
					placeholder: Some("Email"),
					value: None
				})
				(FormField {
					name: "ebook_name",
					input_type: "hidden",
					label: None,
					placeholder: None,
					value: Some(self.ebook.title.as_deref().unwrap_or("No title"))
				})

				.button-container{
					(Button::Primary { href: None, text: "Yes, send me my free ebook!" })
					(Button::Secondary { href: None, text: "No Thanks" })
				}
			}
			(js())
			(css())
		}
	}
}

js! {
	function validateInput(ev, pattern, eventToSend) {
		let regex = new RegExp(pattern);
		let inputIsValid = regex.test(ev.target.value);
		if (inputIsValid) {
			me("button.primary").send(eventToSend);
		} else {
			me("button.primary").disabled = true;
		}
	}

	function handleValidation(ev, stateKey) {
		me(ev)[stateKey] = true;
		if (me(ev).validEmail && me(ev).validFirstName) {
			me(ev).disabled = false;
		}
	}

	me("button.primary").on("valid-email", (ev) => handleValidation(ev, "validEmail"));

	me("button.primary").on("valid-first-name", (ev) => handleValidation(ev, "validFirstName"));

	me("form input[name='email']").on("input", (ev) => {
		validateInput(ev, "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$", "valid-email");
	});

	me("form input[name='first_name']").on("input", (ev) => {
		validateInput(ev, "^[a-zA-Z]{2,}$", "valid-first-name");
	});
}

css! {
	me {
		form {
			grid-area: form;
			display: grid;
			min-width: 300px;
			justify-self: center;
			align-items: baseline;
			justify-content: center;
			gap: var(--size-3);

			label {
				align-self: center;
			}

			input {
				padding: var(--size-2);
				background-color: var(--surface-3);
				box-shadow:
					var(--surface-3) 1px 1px 1px 0px inset,
					var(--surface-4) -1px -1px 6px 1px inset;
			}
		}
		.button-container {
			display: flex;
			flex-direction: column;
			align-items: center;
			margin-top: var(--size-6);
			gap: var(--size-3);
			width: 100%;

			button.secondary {
				background-color: transparent;
				color: var(--text-2);
				border: 1px solid var(--text-2);
				width: 100%;
			}
		}
	}
}
