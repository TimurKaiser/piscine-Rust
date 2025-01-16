pub enum Security {
    Unknown,
    High,
    Medium,
    Low,
    BlockServer,
}

pub fn fetch_data(server: Result<String, String>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => server.unwrap(), // Panique sans message supplémentaire
        Security::High => server.expect("ERROR: program stops"), // Panique avec message spécifique
        Security::Medium => server.unwrap_or("WARNING: check the server".to_string()), // Retourne un avertissement
        Security::Low => server.unwrap_or_else(|err| format!("Not found: {}", err)), // Ajoute l'erreur au message
        Security::BlockServer => {
            if server.is_ok() {
                panic!("{}", server.unwrap()); // Panique avec la valeur de Ok
            } else {
                "WARNING: check the server".to_string() // Message par défaut pour les erreurs
            }
        }
    }
}
