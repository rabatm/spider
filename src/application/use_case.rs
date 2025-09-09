use crate::application::ports::{ImageRepository, PageScraper};
use std::collections::HashSet;
use std::error::Error;
use url::Url;

pub struct ScrapeUseCase<S: PageScraper, R: ImageRepository> {
    scraper: S,
    repository: R,
}

impl<S: PageScraper, R: ImageRepository> ScrapeUseCase<S, R> {
    // Un constructeur pour lui donner ses "outils".
    pub fn new(scraper: S, repository: R) -> Self {
        Self { scraper, repository }
    }

    // La méthode principale qui contient toute la logique de haut niveau.
    pub fn execute(&self, base_url: Url, max_depth: u32) -> Result<(), Box<dyn Error>> {
        let mut a_visiter = vec![base_url];
        let mut visitees = HashSet::new(); // HashSet est plus efficace pour vérifier les doublons
        let mut profondeur_actuelle = 0;

        while !a_visiter.is_empty() && profondeur_actuelle < max_depth {
            if let Some(lien_a_traiter) = a_visiter.pop() {
                if !visitees.insert(lien_a_traiter.clone()) {
                    continue; // `insert` retourne false si l'élément existait déjà
                }

                // Le cas d'usage utilise ses outils (les traits)
                match self.scraper.scrape(&lien_a_traiter) {
                    Ok((images, nouveaux_liens)) => {
                        for image in images {
                            // Ici, pour être parfait, le scraper retournerait aussi les
                            // données de l'image. Pour simplifier, on re-télécharge.
                            if let Ok(response) = reqwest::blocking::get(image.url()) {
                                if let Ok(bytes) = response.bytes() {
                                    self.repository.save(&image, &bytes)?;
                                }
                            }
                        }
                        a_visiter.extend(nouveaux_liens);
                    }
                    Err(e) => eprintln!("Erreur de scraping pour {}: {}", lien_a_traiter, e),
                }
            }
            profondeur_actuelle += 1;
        }
        Ok(())
    }
}