
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
use std::fs;
use std::fs::File;
use std::io::Write;
use nalgebra::DMatrix;

/**************************************************
 *  Funciones con código dependiente del SO.      *
 **************************************************/

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

/**************************************************
 *  Funciones generales (todos los métodos)       *
 **************************************************/

fn ask_for_clear_message() -> String {
    let alphabet =      //95 caracteres permitidos. 
    " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
    let clear_message = Text::new("Ingrese su mensaje a (des)encriptar...")
        .with_help_message(&(format!("Alfabeto:{alphabet}")))
        .with_validator( |msg: &str| {
            if (*msg).is_empty() || !(*msg).bytes().all(|b| (32..127).contains(&b)) { 
                Ok(Validation::Invalid("Hay caracteres prohibidos en la cadena. Mira el alfabeto".into()))
            } else {
                Ok(Validation::Valid)
            }
        })
        .prompt()
        .expect("Error: No se pudo leer tu entrada, ¿cancelaste?");
    return clear_message;
}

fn exit_algorithm_menu() {
    let return_options: Vec<&str> = vec!["<- Volver", "Salir"];
    let return_selected = Select::new("¿Qué desea hacer ahora?", return_options).prompt();
    match return_selected {
        Ok(ans) => match ans {
            "<- Volver" => {},
            "Salir" => process::exit(0),
            _ => panic!("Error: Opción inválida. ¿¿¿!!¿¿cómo??!?!?")
        },
        Err(_) => println!("Error: Hubo un error al capturar tu respuesta, ¿cancelaste?")
    };
}

/**************************************************
 *  Funciones del cifrado de César.               *
 **************************************************/

fn menu_cesar_algorithm(){
    loop {
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
                "<- Atras..." => break,
                _ => panic!("Error: Opción inválida. ¿¿¿!!¿¿cómo??!?!?")
            },
            Err(_) => println!("Error: Hubo un error al capturar tu respuesta, ¿cancelaste?")
        };
    }
}

fn do_cesar_algorithm(key: i32) {
    let clear_message: &str = &(ask_for_clear_message());

    //Shift
    let mut cipher_message: String = String::from("");
    for char_byte in clear_message.bytes() {
        let cipher_byte: u32 = ((((char_byte as i32) - 32 + key).rem_euclid(95)) + 32) as u32;
        let cipher_char = char::from_u32(cipher_byte).unwrap();
        cipher_message.push(cipher_char);
    }
    println!("\nMensaje dado:{clear_message}\nCifrado de César:{cipher_message}\n");
    exit_algorithm_menu();
}

/**************************************************
 *  Funciones de la encriptación con matrices.    *
 **************************************************/

fn get_key_matrix(size: usize) -> DMatrix::<u8> {
    let alphabet =      //95 caracteres permitidos. 
    " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
    let key_upper_limit = size * size;
    let key_lower_limit = size * (size - 1) + 1;
    let key_message = Text::new("Ingrese una llave para (des)encriptar...")
        .with_help_message(&(format!("Alfabeto:{alphabet}")))
        .with_validator( move |msg: &str| {
            if (*msg).is_empty() || !(*msg).bytes().all(|b| (32..127).contains(&b)) { 
                return Ok(Validation::Invalid("Hay caracteres prohibidos en la cadena. Mira el alfabeto".into()));
            } else if (*msg).len() <= size * (size - 1) || (*msg).len() > size * size {
                return Ok(Validation::Invalid(format!("No puedes tener menos de {key_lower_limit} caracteres, ni más de {key_upper_limit}.").into()));
            } else if !encode_msg_to_matrix(msg, size).cast::<f32>().is_invertible() {
                return Ok(Validation::Invalid("Esta llave no puede ser usada, escriba otra.".into()));
            } else {
                return Ok(Validation::Valid);
            }
        })
        .prompt()
        .expect("Error: No se pudo leer tu entrada, ¿cancelaste?");

    return encode_msg_to_matrix(&key_message, size);
}

fn encode_msg_to_matrix(msg: &str, size: usize) -> DMatrix::<u8> {
    let mut msg_vector: Vec<u8> = msg.to_string().into_bytes();
    msg_vector.resize(size*size, 0);

    let clear_msg_matrix = DMatrix::<u8>::from_row_iterator(
        size, size, msg_vector.iter().cloned()
    );
    return clear_msg_matrix;
}

fn decode_matrix_to_msg(mat: DMatrix::<f32>) -> String {
    let mut clear_message: String = String::from("");
    let mat_transposed = mat.transpose();

    for num_ref in mat_transposed.iter(){
        let num = num_ref.to_owned().round();
        if num == (0 as f32) {break};
        if !(32..127).contains(&(num as i32)) {
            println!("Error: La llave es incorrecta, el mensaje no será el correcto.");
            process::exit(1);
        }
        let clear_char = match char::from_u32(num as u32){
            Some(character) => character, 
            None => {
                println!("Error: La llave es incorrecta, el mensaje no será el correcto.");
                process::exit(1);
            }
        };
        clear_message.push(clear_char);
    }

    return clear_message;
}

fn do_matrix_encryption(){
    let clear_message: &str = &(ask_for_clear_message());
    let matrix_n_size: usize = ((clear_message.len() as f64).sqrt()
                                .ceil() as u32)
                                .try_into().unwrap();
    let clear_msg_matrix = encode_msg_to_matrix(clear_message, matrix_n_size);
    let key_matrix = get_key_matrix(matrix_n_size);
    let cipher_message = clear_msg_matrix.clone().cast::<i32>() * key_matrix.clone().cast::<i32>();
    println!("\nEncriptando tu mensaje...{clear_msg_matrix} X {key_matrix} = {cipher_message}");

    let mut msg_file = File::create("Mensaje_encriptado").expect("Error: No se pudo crear el archivo, ¿tienes permisos?");
    for num in cipher_message.iter(){
        write!(msg_file, "{} ", num).expect("Error: No se pudo escribir al archivo, ¿tienes permisos?")
    }
    println!("Tu mensaje encriptado se guardó en un archivo llamado \"Mensaje_encriptado\".");

    exit_algorithm_menu();
}

fn do_matrix_decryption(){
    let file_str = fs::read_to_string("Mensaje_encriptado")
        .expect("Error: No se pudo leer el archivo, ¿ya encriptaste algo?");
    let cipher_msg_vec: Vec<i32> = file_str.split_ascii_whitespace()
                                   .collect::<Vec<_>>()
                                   .iter()
                                   .map(|x| x.parse::<i32>().unwrap())
                                   .collect();
    let size: usize = ((cipher_msg_vec.len() as f32).sqrt() as i32).try_into().unwrap();
    let cipher_msg_matrix = DMatrix::<i32>::from_iterator(
        size, size, cipher_msg_vec.iter().cloned()
    );
    println!("\nMensaje encriptado en archivo{cipher_msg_matrix}");
    let key_matrix = get_key_matrix(size);
    let inverse_key_matrix = key_matrix.clone().cast::<f32>().try_inverse().unwrap();
    let clear_msg_matrix = cipher_msg_matrix.clone().cast::<f32>() * inverse_key_matrix.clone();
    println!("\nDesencriptando tu mensaje, si la llave es incorrecta entonces el programa parará o se mostrará una desencriptación errónea...{cipher_msg_matrix} X {inverse_key_matrix} = {clear_msg_matrix}");
    let clear_message = decode_matrix_to_msg(clear_msg_matrix); 
    println!("Mensaje decifrado:{clear_message}\n");
    exit_algorithm_menu();
}

fn menu_matrix_algorithm(){
    loop {
        clearscreen::clear().expect("Error: No se pudo limpiar la pantalla.");
        let matrix_options: Vec<&str> = vec!["Encriptar","Desencriptar", "<- Atras..."];
        let matrix_selected = Select::new("¿Desea encriptar o desencriptar?", matrix_options)
            .prompt();
        match matrix_selected {
            Ok(ans) => match ans {
                "Encriptar" => do_matrix_encryption(),
                "Desencriptar" => do_matrix_decryption(),
                "<- Atras..." => break,
                _ => panic!("Error: Opción inválida. ¿¿¿!!¿¿cómo??!?!?")
            },
            Err(_) => println!("Error: Hubo un error al capturar tu respuesta, ¿cancelaste?")
        };
    }
}


/**************************************************
 *  Menú principal.                               *
 **************************************************/

fn main() {
    loop {
        clearscreen::clear().expect("Error: No se pudo limpiar la pantalla.");
        let algorithm_options: Vec<&str> = vec!["Algoritmo de César",
                                                "Algoritmo de matrices", 
                                                "Acerca de", 
                                                "Imprimir código fuente", 
                                                "Salir"];
        let bienvenida = "Te damos la bienvenida a este PIA. Seleccione una opción o un algoritmo:";
        let selected = Select::new(&bienvenida, algorithm_options).prompt();

        match selected {
            Ok(ans) => match ans {
                "Algoritmo de César" => menu_cesar_algorithm(),
                "Algoritmo de matrices" => menu_matrix_algorithm(),
                "Acerca de" => print_man_page(),
                "Imprimir código fuente" => print_src_code(),
                "Salir" => process::exit(0),
                _ => panic!("Error: Opción inválida. ¿¿¿!!¿¿cómo??!?!?")
            },
            Err(_) => panic!("Error: Hubo un error al capturar tu respuesta, ¿cancelaste?")
        }
    }
}
