// no time made by gpt
pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<&str> = phrase.split_whitespace().collect(); // Divise la phrase en mots et crée un vecteur

    // Trie les mots en fonction du chiffre présent dans chaque mot
    words.sort_by_key(|word| {
        word.chars()
            .find(|c| c.is_digit(10)) // Trouve le premier chiffre dans le mot
            .unwrap_or('0') // Par défaut, retourne '0' si aucun chiffre n'est trouvé
            .to_digit(10) // Convertit en nombre
            .unwrap_or(0) // Par défaut 0
    });

    words.join(" ") // Assemble les mots triés en une seule chaîne séparée par des espaces
}
