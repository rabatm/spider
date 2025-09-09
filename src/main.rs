mod application;
mod domain;
mod infrastructure;

use crate::application::use_case::ScrapeUseCase;
use crate::infrastructure::file_system_repository::FileSystemImageRepository;
use crate::infrastructure::reqwest_scraper::ReqwestScraper;
use clap::Parser;
use url::Url;
///un scraper web simple qui télécharge recursivement les images d'un site.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// URL du site à scraper
    #[arg(required = true)]
    url: String,
    /// Télécharger les images récursivement
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    recursive: bool,

    /// Niveau de profondeur maximum pour la récursion
    #[arg(short, long, default_value_t = 5)]
    level: u32,

    /// Chemin pour sauvegarder les fichiers téléchargés
    #[arg(short, long, default_value = "./data/")]
    path: String,

}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Lire la configuration
    let args = Args::parse();
    let base_url = Url::parse(&args.url)?;

    // 2. Créer les "appareils" concrets de l'infrastructure
    let scraper = ReqwestScraper;
    let repository = FileSystemImageRepository;

    // 3. Créer le Cas d'Usage et lui "injecter" les dépendances
    let scrape_use_case = ScrapeUseCase::new(scraper, repository);

    // 4. Lancer l'exécution
    println!("--- DÉMARRAGE DU SCRAPING (Clean Architecture) ---");
    scrape_use_case.execute(base_url, args.level)?;
    println!("--- SCRAPING TERMINÉ ---");

    Ok(())
}
