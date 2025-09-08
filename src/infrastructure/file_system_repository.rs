use crate::application::ports::ImageRepository;
use crate::domain::image::Image;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

// On crée une structure (même vide) qui portera notre logique.
pub struct FileSystemImageRepository;

// On dit à Rust que cette structure implémente le contrat ImageRepository.
impl ImageRepository for FileSystemImageRepository {
    // On doit donc fournir le code pour la méthode `save` exigée par le contrat.
    fn save(&self, image: &Image, image_data: &[u8]) -> Result<(), Box<dyn Error>> {
        let mut path = PathBuf::from("./data/"); // On pourrait rendre ce chemin configurable plus tard
        
        // On utilise notre getter pour récupérer le nom du fichier !
        path.push(image.file_name());

        fs::create_dir_all("./data/")?;
        fs::write(&path, image_data)?;
        
        println!("-> Image sauvegardée : {}", path.display());
        
        // Si tout s'est bien passé, on retourne Ok avec "rien" dedans : ()
        Ok(())
    }
}