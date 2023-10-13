use inquire::{Select, CustomType, validator::Validation};

fn cesar_algorithm() {
    let key = CustomType::<u8>::new("Escriba la llave que desee usar para la encripción/desencripción.")
        .with_error_message("Ingrese un número mayor o igual a 1")
        .with_help_message("La llave es el número de rotaciones que se le da al abecedario.")
        .with_validator( |val: &u8| {
            if *val < 1 {
                Ok(Validation::Invalid("El número debe ser mayor que 0".into()))
            } else {
                Ok(Validation::Valid)
            }
        })
        .prompt().expect("u8");

    let cesar_options: Vec<&str> = vec!["Encriptar","Desencriptar", "<- Atras..."];
    let cesar_selected = Select::new("¿Desea encriptar o desencriptar?", cesar_options)
        .with_help_message(&(format!("Usando la llave {key}")))
        .prompt();
    match cesar_selected {
        Ok(ans) => match ans {
            "Encriptar" => encrypt_cesar_algorithm(key),
            "Desencriptar" => decrypt_cesar_algorithm(key),
            "<- Atras..." => main(),
            _ => panic!("Error: Opción inválida. ¿¿¿!!¿¿cómo??!?!?")
        },
        Err(_) => println!("Error: Hubo un error al capturar tu respuesta, ¿cancelaste?")
    };
}

fn encrypt_cesar_algorithm(key: u8) {
    println!("{key}");
}

fn decrypt_cesar_algorithm(key: u8) {
    println!("{key}");
}


fn rsa_algorithm() {
    println!("RSA!!!");
}

fn main() {
    let algorithm_options: Vec<&str> = vec!["Algoritmo de César", "Algoritmo RSA"];
    let bienvenida = "Te damos la bienvenida a este PIA. Seleccione un método de encriptación:";
    let selected = Select::new(&bienvenida, algorithm_options).prompt();

    match selected {
        Ok(ans) => match ans {
            "Algoritmo de César" => cesar_algorithm(),
            "Algoritmo RSA" => rsa_algorithm(),
            _ => panic!("Error: Opción inválida. ¿¿¿!!¿¿cómo??!?!?")
        },
        Err(_) => println!("Error: Hubo un error al capturar tu respuesta, ¿cancelaste?")
    }

}
