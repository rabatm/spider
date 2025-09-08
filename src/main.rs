use clap::Parser;
use scraper::{Html, Selector}; 
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

fn main() {
    let args = Args::parse();
    
    //On transforme l'URL de l'utilisateur en une URL de base structurée
    let base_url = Url::parse(&args.url)
        .expect("ERREUR : URL de base invalide.");
    println!("Récupération de l'URL : {}", base_url);

    let response = reqwest::blocking::get(&args.url)
        .expect("ERREUR : Impossible de récupérer l'URL.");
    
    let body = response.text()
        .expect("ERREUR : Impossible de lire le corps de la réponse.");

    println!("--- Analyse du HTML ---");

    // On transforme le texte HTML en un document qu'on peut interroger
    let document = Html::parse_document(&body);

    // On crée un "sélecteur" CSS pour trouver toutes les balises <img>
    let image_selector = Selector::parse("img").unwrap();

    // On cherche tous les éléments qui correspondent à notre sélecteur
    for element in document.select(&image_selector) {
        // Pour chaque élément trouvé, on essaie de récupérer son attribut "src"
        if let Some(src) = element.value().attr("src") {
            match base_url.join(src) {
                Ok(full_url) => {
                    let url_as_string = full_url.to_string();

                    if url_as_string.ends_with(".jpg") || url_as_string.ends_with(".jpeg") || 
                        url_as_string.ends_with(".jpeg") || url_as_string.ends_with(".png")  ||
                        url_as_string.ends_with(".gif") || url_as_string.ends_with(".bmp") 
                    {
                            println!("Image à télécharger : {}", full_url);
                    }
                    
                }
                Err(e) => {
                    // On affiche une erreur si l'URL est malformée, mais on continue
                    println!("Impossible de traiter l'URL '{}': {}", src, e);
                }
            }
        }
    }
}
