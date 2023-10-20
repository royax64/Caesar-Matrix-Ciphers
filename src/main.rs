

/**************************************************
 *  Autor: Rogel Axel Guel Lerma                  *
 *  Matricula: 1957977                            *
 *  Materia: Criptografía y Seguridad Gpo.032     *
 **************************************************
 *  Aviso: Presione Ctrl + p para imprimir...     *
 **************************************************/

use inquire::{Select, CustomType, validator::Validation};
use std::process;
use std::process::Command;

// Funciones con código dependiente del SO.
fn print_man_page(){
    if cfg!(windows){
        Command::new("notepad")
            .arg("src/win_program_info.txt")
            .spawn()
            .expect("Error: No se pudo abrir el documento de información.")
            .wait()
            .unwrap();
    } else {
        Command::new("man")
            .arg("src/program_info")
            .spawn()
            .expect("Error: No se pudo abrir el documento de información.")
            .wait()
            .unwrap();
    }
}

fn print_src_code(){
    webbrowser::open("src/main.rs")
        .expect("Error: ¿Apoco no tienes un navegador instalado?");
}

//Funciones del cifrado de César
fn cesar_algorithm(){
    let key = CustomType::<i32>::new("Escriba la llave que desee usar para la encripción/desencripción.")
        .with_error_message("Ingrese un número diferente de 0")
        .with_help_message("La llave es el número de rotaciones que se le da al abecedario.")
        .with_validator( |val: &i32| {
            if *val == 0 {
                Ok(Validation::Invalid("El número NO debe ser igual a 0".into()))
            } else {
                Ok(Validation::Valid)
            }
        })
        .prompt().expect("i32");

    let cesar_options: Vec<&str> = vec!["Encriptar","Desencriptar", "<- Atras..."];
    let cesar_selected = Select::new("¿Desea encriptar o desencriptar?", cesar_options)
        .with_help_message(&(format!("Usando la llave {key}")))
        .prompt();
    match cesar_selected {
        Ok(ans) => match ans {
            "Encriptar" => do_cesar_algorithm(key),
            "Desencriptar" => do_cesar_algorithm(-1 * key),
            "<- Atras..." => {},
            _ => panic!("Error: Opción inválida. ¿¿¿!!¿¿cómo??!?!?")
        },
        Err(_) => println!("Error: Hubo un error al capturar tu respuesta, ¿cancelaste?")
    };
}

fn do_cesar_algorithm(key: i32) {
//usa -key para desencriptar
    println!("{key}");
}

fn rsa_algorithm(){
    println!("RSA!!!");
}




fn main() {
    loop {
        clearscreen::clear().expect("Error: No se pudo limpiar la pantalla.");
        let algorithm_options: Vec<&str> = vec!["Algoritmo de César", "Algoritmo RSA", "Acerca de", "Imprimir código fuente", "Salir"];
        let bienvenida = "Te damos la bienvenida a este PIA. Seleccione una opción o un algoritmo:";
        let selected = Select::new(&bienvenida, algorithm_options).prompt();

        match selected {
            Ok(ans) => match ans {
                "Algoritmo de César" => cesar_algorithm(),
                "Algoritmo RSA" => rsa_algorithm(),
                "Acerca de" => print_man_page(),
                "Imprimir código fuente" => print_src_code(),
                "Salir" => process::exit(0),
                _ => panic!("Error: Opción inválida. ¿¿¿!!¿¿cómo??!?!?")
            },
            Err(_) => println!("Error: Hubo un error al capturar tu respuesta, ¿cancelaste?")
        }
    }
}
