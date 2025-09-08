use clap::Parser;
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
    println!("URL: {}", args.url);
    println!("Recursive: {}", args.recursive);
    println!("Level: {}", args.level);
    println!("Path: {}", args.path);
}
