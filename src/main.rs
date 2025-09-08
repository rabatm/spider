use clap::Parser;
use scraper::{Html, Selector}; 
use url::Url;
use std::path::PathBuf;
use std::fs;

mod domain;
mod application;
mod infrastructure;

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

fn scrape_page(url_a_visiter: &Url, path_de_sauvegarde: &str) -> Vec<Url> {
    let mut nouveaux_liens = Vec::new();
    let response = reqwest::blocking::get(url_a_visiter.clone())
        .expect("ERREUR : Impossible de récupérer l'URL.");
    let body = response.text()
        .expect("ERREUR : Impossible de lire le corps de la réponse.");
    let document = Html::parse_document(&body);
    // On crée un "sélecteur" CSS pour trouver toutes les balises <img>
    let image_selector = Selector::parse("img").unwrap();
    // On cherche tous les éléments qui correspondent à notre sélecteur
    for element in document.select(&image_selector) {
        // Pour chaque élément trouvé, on essaie de récupérer son attribut "src"
        if let Some(src) = element.value().attr("src") {
            match url_a_visiter.join(src) {
                Ok(full_url) => {
                    let url_as_string = full_url.to_string();

                    if url_as_string.ends_with(".jpg") || url_as_string.ends_with(".jpeg") || 
                        url_as_string.ends_with(".jpeg") || url_as_string.ends_with(".png")  ||
                        url_as_string.ends_with(".gif") || url_as_string.ends_with(".bmp") 
                    {
                        // On extrait le nom du fichier depuis l'URL
                        let file_name = full_url.path_segments()
                            .and_then(|segments| segments.last())
                            .unwrap_or("image.tmp");

                        // On construit le chemin de sauvegarde complet
                        let mut final_path = PathBuf::from(path_de_sauvegarde);
                        final_path.push(file_name); 
                        println!("Image à télécharger : {} -> {}", url_as_string, final_path.display());

                        // ... le code pour télécharger l'image ne change pas ...
                        let image_response = reqwest::blocking::get(full_url).unwrap();
                        let image_bytes = image_response.bytes().unwrap();
                        // On utilise maintenant notre chemin final
                        fs::write(&final_path, &image_bytes)
                            .expect("Erreur durant la sauvegarde de l'image.");
                        println!("-> Image sauvegardée dans : {}", final_path.display());
                    }
                }
                Err(e) => {
                    // On affiche une erreur si l'URL est malformée, mais on continue
                    println!("Impossible de traiter l'URL '{}': {}", src, e);
                }
            }
        }
    }
    let link_selector = Selector::parse("a").unwrap();
    for element in document.select(&link_selector) {
        if let Some(href) = element.value().attr("href") {
            if let Ok(lien_absolu) = url_a_visiter.join(href) {
                nouveaux_liens.push(lien_absolu);
            }
        }
    }
    nouveaux_liens
}


fn main() {
    let args = Args::parse();
    let mut visitees = Vec::new();
    let mut profondeur = args.level;
    fs::create_dir_all(&args.path).expect("Impossible de créer le dossier de sauvegarde.");
    let base_url = Url::parse(&args.url)
        .expect("ERREUR : URL de base invalide.");
    println!("Récupération de l'URL : {}", base_url);
    let mut a_visiter = scrape_page(&base_url, &args.path);
    println!("Récupération des liens :");
    while (a_visiter.len() > 0) && profondeur > 0
    {
        if let Some(prochain_lien) = a_visiter.pop()
        {
            if !visitees.contains(&prochain_lien)
            {
                let new_visiter = scrape_page(&prochain_lien, &args.path);
                a_visiter.pop();
                a_visiter.extend(new_visiter);
                visitees.push(prochain_lien);
            }
        }
        profondeur = profondeur - 1;
    }
}
