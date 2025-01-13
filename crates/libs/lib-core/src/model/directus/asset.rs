use super::EbooksTranslations;

pub trait Asset {
  const BASE_URL: &'static str;
  fn file_url(&self) -> String;
}

impl Asset for EbooksTranslations {
  const BASE_URL: &'static str = "https://directus.eman.network/assets";
  fn file_url(&self) -> String {
    format!("{}/{}", Self::BASE_URL, self.file.as_ref().unwrap())
  }
}

pub trait CoverImage: Asset {
  fn cover_image_url(&self) -> String;
  fn thumbnail_url(&self, width: u32) -> String;
}

impl CoverImage for EbooksTranslations {
  fn cover_image_url(&self) -> String {
    format!("{}/{}", Self::BASE_URL, self.cover_image.as_ref().unwrap())
  }

  fn thumbnail_url(&self, width: u32) -> String {
    format!(
      "{}/{}$thumbnail={width}",
      Self::BASE_URL,
      self.cover_image.as_ref().unwrap()
    )
  }
}
