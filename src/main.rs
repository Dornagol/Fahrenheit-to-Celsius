use std::io;

fn convert(unit: &str, value: f32) -> f32 {
    if unit == "fahrenheit" {
        value * 1.8 + 32.0
    } else if unit == "celsius" {
        (value - 32.0) / 1.8
    } else {
        panic!("Unité invalide ! Utilisez \"fahrenheit\" ou \"celsius\".");
    }
}

fn main() {
    println!("Entrez le type de conversion souhaitée ('fahrenheit' ou 'celsius') :");
    let mut unit = String::new();

    io::stdin()
        .read_line(&mut unit)
        .expect("Erreur de lecture de l'unité");

    let unit = unit.trim();

    if unit != "fahrenheit" && unit != "celsius" {
        println!("Unité invalide. Veuillez utiliser 'fahrenheit' ou 'celsius'.");
        return;
    }

    println!("Entrez une valeur :");
    let mut value = String::new();

    io::stdin()
        .read_line(&mut value)
        .expect("Erreur de lecture de la valeur");

    let value: f32 = match value.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Valeur invalide. Veuillez entrer un nombre.");
            return;
        }
    };

    let degrees = convert(unit, value);
    println!("La conversion est : {:.2}", degrees);
}