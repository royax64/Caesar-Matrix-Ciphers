
/**************************************************
 *  Autor: Rogel Axel Guel Lerma                  *
 *  Matricula: 1957977                            *
 *  Materia: Criptografía y Seguridad Gpo.032     *
 **************************************************
 *  Aviso: Presione Ctrl + p para imprimir...     *
 **************************************************
 *  Todas las funciones están en este archivo.    *
 **************************************************/

use inquire::{Select, CustomType, validator::Validation, Text};
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
fn menu_cesar_algorithm(){
    clearscreen::clear().expect("Error: No se pudo limpiar la pantalla.");
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
            "Desencriptar" => do_cesar_algorithm(-1 * key), //-key desencripta...
            "<- Atras..." => {},
            _ => panic!("Error: Opción inválida. ¿¿¿!!¿¿cómo??!?!?")
        },
        Err(_) => println!("Error: Hubo un error al capturar tu respuesta, ¿cancelaste?")
    };
}

fn do_cesar_algorithm(key: i32) {
    let alphabet =      //95 caracteres permitidos. 
    " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
    let clear_message = Text::new("Ingrese su mensaje a (des)encriptar...")
        .with_help_message(&(format!("Usando la llave {key}]\n[Alfabeto:{alphabet}")))
        .with_validator( |msg: &str| {
            if (*msg).is_empty() || !(*msg).bytes().all(|b| (32..127).contains(&b)) { 
                Ok(Validation::Invalid("Hay caracteres prohibidos en la cadena. Mira el alfabeto".into()))
            } else {
                Ok(Validation::Valid)
            }
        })
        .prompt()
        .expect("Error: No se pudo leer tu entrada, ¿cancelaste?");

    //Shift
    let mut cipher_message: String = String::from("");
    for char_byte in clear_message.bytes() {
        let cipher_byte: u32 = ((((char_byte as i32) - 32 + key).rem_euclid(95)) + 32) as u32;
        let cipher_char = char::from_u32(cipher_byte).unwrap();
        cipher_message.push(cipher_char);
    }
    println!("\nMensaje dado:{clear_message}\nCifrado de César:{cipher_message}\n");

    let return_options: Vec<&str> = vec!["<- Atrás","Inicio"];
    let return_selected = Select::new("¿Qué desea hacer ahora?", return_options).prompt();
    match return_selected {
        Ok(ans) => match ans {
            "<- Volver" => menu_cesar_algorithm(),
            "Inicio" => {},
            _ => panic!("Error: Opción inválida. ¿¿¿!!¿¿cómo??!?!?")
        },
        Err(_) => println!("Error: Hubo un error al capturar tu respuesta, ¿cancelaste?")
    };
}

fn menu_rsa_algorithm(){
    clearscreen::clear().expect("Error: No se pudo limpiar la pantalla.");
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
                "Algoritmo de César" => menu_cesar_algorithm(),
                "Algoritmo RSA" => menu_rsa_algorithm(),
                "Acerca de" => print_man_page(),
                "Imprimir código fuente" => print_src_code(),
                "Salir" => process::exit(0),
                _ => panic!("Error: Opción inválida. ¿¿¿!!¿¿cómo??!?!?")
            },
            Err(_) => println!("Error: Hubo un error al capturar tu respuesta, ¿cancelaste?")
        }
    }
}
