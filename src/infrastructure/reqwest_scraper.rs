use crate::application::ports::PageScraper;
use crate::domain::image::Image;
use scraper::{Html, Selector};
use std::error::Error;
use url::Url;

// Notre structure concrète
pub struct ReqwestScraper;

// On implémente le contrat PageScraper
impl PageScraper for ReqwestScraper {
    fn scrape(&self, url: &Url) -> Result<(Vec<Image>, Vec<Url>), Box<dyn Error>> {
        println!("--- Scraping de : {} ---", url);

        // Logique Reqwest pour récupérer le HTML
        let response = reqwest::blocking::get(url.clone())?;
        let body = response.text()?;
        let document = Html::parse_document(&body);

        // Logique pour trouver les images
        let mut images_trouvees = Vec::new();
        let image_selector = Selector::parse("img")?;
        for element in document.select(&image_selector) {
            if let Some(src) = element.value().attr("src") {
                if let Ok(full_url) = url.join(src) {
                    let url_as_string = full_url.to_string();
                    let extensions = [".jpg", ".jpeg", ".png", ".gif", ".bmp"];
                    if extensions.iter().any(|ext| url_as_string.ends_with(ext)) {
                        if let Some(file_name) = full_url.path_segments().and_then(|s| s.last()) {
                            // On crée notre entité Image via son constructeur
                            let image = Image::new(url_as_string, file_name.to_string());
                            images_trouvees.push(image);
                        }
                    }
                }
            }
        }

        // Logique pour trouver les liens
        let mut liens_trouves = Vec::new();
        let link_selector = Selector::parse("a")?;
        for element in document.select(&link_selector) {
            if let Some(href) = element.value().attr("href") {
                if let Ok(lien_absolu) = url.join(href) {
                    liens_trouves.push(lien_absolu);
                }
            }
        }
        
        // On retourne le tuple (images, liens) comme promis dans le contrat
        Ok((images_trouvees, liens_trouves))
    }
}