use crate::prelude::*;

pub struct FormField<'a> {
  pub label: Option<&'a str>,
  pub name: &'a str,
  pub input_type: &'a str,
  pub placeholder: Option<&'a str>,
  pub value: Option<&'a str>,
}

impl Render for FormField<'_> {
  fn render(&self) -> Markup {
    html! {
      //label { (self.label.unwrap_or(self.name)) }
      input
      type=(self.input_type)
      id=(self.name)
      name=(self.name)
      placeholder=(self.placeholder.unwrap_or(""))
      value=(self.value.unwrap_or(""))
      {}
      (css())
    }
  }
}

css! {
  me {
    label {
      color: var(--text-2);
    }
    input {
      color: var(--text-1);
    }
  }
}
