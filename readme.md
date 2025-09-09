# 🕷️ Arachnida - Un Scraper Web en Rust

Arachnida est un projet réalisé dans le cadre du cursus de l'école 42, axé sur le web scraping et l'analyse de métadonnées. Ce projet a été entièrement développé en **Rust** en suivant les principes de la **Clean Architecture** pour garantir un code modulaire, testable et maintenable.

Il se compose de deux programmes distincts :
* `spider` : Un outil en ligne de commande pour télécharger récursivement les images d'un site web.
* `scorpion` : Un outil pour extraire et afficher les métadonnées EXIF des images.

---

## 🧅 Architecture

Ce projet a été conçu en suivant les principes de la **Clean Architecture** pour séparer clairement les responsabilités du programme. L'organisation du code est divisée en trois couches principales :

* **💛 Domain** : Le cœur de l'application. Contient les entités métier pures (ex: la structure `Image`) sans aucune dépendance technique.
* **🧠 Application** : La couche logique. Définit les "contrats" (`traits`) qui décrivent ce que l'application doit faire, et les "Cas d'Usage" (Use Cases) qui orchestrent la logique métier.
* **🌍 Infrastructure** : La couche externe. Contient les implémentations concrètes des contrats en utilisant des outils techniques (ex: `reqwest` pour les requêtes HTTP, `scraper` pour le parsing HTML, `std::fs` pour l'écriture de fichiers).

Le choix d'implémenter la Clean Architecture pour ce projet, bien qu'il s'agisse d'un simple outil en ligne de commande, est une démarche délibérée visant à mettre en pratique des principes d'ingénierie logicielle robustes.

L'objectif était d'aller au-delà d'un simple script fonctionnel pour construire une application dont la structure offre plusieurs avantages clés :

* **Testabilité Accrue** : La logique métier (les "Cas d'Usage") est complètement découplée des détails techniques. Cela signifie qu'on peut la tester de manière unitaire sans avoir besoin d'un vrai réseau ou d'un vrai système de fichiers, en utilisant des "mocks".

* **Flexibilité et Évolution** : L'application est "pluggable". Aujourd'hui, elle sauvegarde les images sur le disque local. Demain, on pourrait facilement ajouter une nouvelle implémentation pour sauvegarder sur un service cloud (comme Amazon S3) sans changer une seule ligne de la logique métier.

* **Indépendance aux Frameworks** : Les bibliothèques comme `reqwest` ou `scraper` sont traitées comme des détails d'implémentation, et non comme le cœur de l'application. Le "métier" ne dépend pas d'elles, ce sont elles qui s'adaptent au "métier".

* **Compréhensibilité** : La séparation claire des couches rend le code plus facile à lire, à comprendre et à maintenir, pour moi-même ou pour un autre développeur (ou un correcteur !).

En somme, ce projet était autant un exercice sur l'écriture de code qui **fonctionne** qu'un exercice sur l'écriture de code **durable** et **bien conçu**.

---

## ✨ Fonctionnalités

### Spider
* Téléchargement récursif des images d'un site à partir d'une URL de départ.
* Profondeur de récursion configurable avec l'option `-l` (par défaut : 5).
* Chemin de sauvegarde des images configurable avec l'option `-p` (par défaut : `./data/`).
* Filtrage des images par extensions : `.jpg`, `.jpeg`, `.png`, `.gif`, `.bmp`.

### Scorpion
* Extraction et affichage des métadonnées (EXIF) d'un ou plusieurs fichiers image.
* Compatible avec les mêmes formats d'image que le `spider`.

---

## 🛠️ Installation et Compilation

Le projet utilise **Cargo**, le gestionnaire de paquets et outil de build de Rust. Un `Makefile` est également fourni pour simplifier les commandes courantes, conformément aux standards de 42.

1.  **Clonez le dépôt :**
    ```bash
    git clone [https://github.com/VOTRE_LOGIN/arachnida.git](https://github.com/VOTRE_LOGIN/arachnida.git)
    cd arachnida
    ```

2.  **Compilez le projet :**
    La manière la plus simple est d'utiliser le `Makefile`. Cette commande va compiler le projet en mode "release" (optimisé) et copier le binaire à la racine.
    ```bash
    make
    ```

---

## 🚀 Utilisation

### Spider
Le programme prend une URL en paramètre ainsi que des options.

```bash
./spider [-r] [-l PROFONDEUR] [-p CHEMIN] URL