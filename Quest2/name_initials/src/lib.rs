// made by gpt, no time

pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut result = Vec::new(); // Crée un vecteur vide pour stocker les résultats

    for name in names { // Parcourt chaque nom dans le vecteur
        let mut initials = String::new(); // Crée une chaîne vide pour les initiales

        for word in name.split_whitespace() { // Divise le nom en mots
            let first_letter = &word[0..1]; // Prend la première lettre du mot
            initials.push_str(first_letter); // Ajoute la lettre à la chaîne
            initials.push('.'); // Ajoute un point après chaque initiale
            initials.push(' '); // Ajoute un espace
        }

        initials.pop(); // Supprime le dernier espace en trop
        result.push(initials); // Ajoute les initiales complètes au vecteur de résultats
    }

    result // Retourne le vecteur contenant toutes les initiales
}
