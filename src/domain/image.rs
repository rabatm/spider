#[derive(Debug, Clone)]
pub struct Image {
    url: String,
    file_name: String,
}

impl Image {
    /// Crée une nouvelle instance de Image.
    pub fn new(url: String, file_name: String) -> Self {
        Self { url, file_name }
    }

    /// Retourne une référence sur l'URL de l'image ("getter").
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Retourne une référence sur le nom de fichier de l'image ("getter").
    pub fn file_name(&self) -> &str {
        &self.file_name
    }
}