use crate::domain::image::Image;
use url::Url;
use std::error::Error;

/// Contrat pour un service qui peut scraper une page web.
pub trait PageScraper {
    fn scrape(&self, url: &Url) -> Result<(Vec<Image>, Vec<Url>), Box<dyn Error>>;
}

/// Contrat pour un service qui peut sauvegarder les données d'une image.
pub trait ImageRepository {
    fn save(&self, image: &Image, image_data: &[u8]) -> Result<(), Box<dyn Error>>;
}