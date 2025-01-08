// no time made by gpt
pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<&str> = phrase.split_whitespace().collect(); // Divise la phrase en mots

    // Trie les mots en fonction du chiffre dans chaque mot
    words.sort_by_key(|word| {
        word.chars()
            .find(|c| c.is_digit(10)) // Cherche le chiffre
            .unwrap_or('0') // Valeur par défaut si aucun chiffre
            .to_digit(10) // Convertit en entier
            .unwrap_or(0)
    });

    // Supprime les chiffres et assemble les mots triés
    words
        .iter()
        .map(|word| word.chars().filter(|c| !c.is_digit(10)).collect::<String>()) // Enlève les chiffres
        .collect::<Vec<String>>() // Crée un vecteur de mots nettoyés
        .join(" ") // Assemble les mots avec des espaces
}
