use std::env;
use std::io::{Error, ErrorKind};

use tp0::operaciones::Operacion;
use tp0::operaciones::{
    and::And, cr::Cr, drop::Drop, dup::Dup, emit::Emit, esMayorQue::EsMayorQue,
    esMenorQue::EsMenorQue, ifelsethen::If_else_then, ifthen::If_then, igual::Igual, not::Not,
    or::Or, over::Over, producto::Producto, punto::Punto, puntocomillas::Punto_comillas,
    resta::Resta, suma::Suma, swap::Swap, word::Word,
};
use tp0::Hash_Forth::Hash_Forth;
use tp0::Pila::Pila;

use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

//Seteo una pila y un hashmap que van a ser persistentes durante toda la ejecucion
//sobre estos ejecuto las operaciones/modificaciones que quiera hacer
//leo el archivo de entrada .fth linea por linea, hago un .split y cada elemento del vector
//seria una orden

//si es un numero, lo apilo
//si es una operacion, hago pattern matching y la identifico, luego se ejecuta a la pila
//si el vector comienza por : hago la logica de definir una word
// si una orden comienza por IF, itero y construyo su vector de instrucciones hasta un else o then
//si encuentro un else, itero otro bloque de instrucciones hasta el then.

//OUTPUT: si hay instruccion de output, se imprime en consola por stdout
//Al terminar todas las instrucciones, escribimos el estado final de la pila
//en un archivo llamado Stack.fth

//Main recibe la ruta al .fth del que queremos ejecutar las instrucciones
//no devuelve nada

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Uso: {} <ruta_al_archivo>", args[0]);
        std::process::exit(1);
    }

    let ruta_archivo = &args[1];
    ejecutar(ruta_archivo)
}

pub fn ejecutar(ruta_archivo: &str) -> io::Result<()> {
    let mut diccionario = Hash_Forth::new();
    let mut pila = Pila::<i32>::new();

    let ruta = Path::new(ruta_archivo);
    let archivo = File::open(&ruta)?;

    let lector = io::BufReader::new(archivo);

    for linea in lector.lines() {
        let linea = linea?;
        let vector: Vec<&str> = linea.split_whitespace().collect();
        //manejo 4 casos
        //1) si el elemento es un numero, lo apilo en mi pila
        //2)si el elemento es un : llamo a una funcion que crea la word hasta el ;
        //3) si el elemento es if, llamo a mi funcion para crear la instruccion if
        //4) si no es ninguna de las anteriores, hago pattern matching para ver que tipo de operacion es
        let mut i = 0;
        while i < vector.len() {
            let token = vector[i];

            if let Ok(numero) = token.parse::<i16>() {
                pila.apilar(numero.into());
            } else if token == ":" {
                //si quiero definir word, itero hasta el ;
                //luego actualizo mi indice general para que siga iterando luego del ;
                //entonces mi funcion setWord crea la word y ademas actualiza el indice
                //para que siga iterando luego del ;

                //esto es el caso solo donde quiero definir la word
                //no la ejecuto directamente, solo quedaria en el diccionario

                i = setWord(&vector, i, &mut diccionario)
                    .map_err(|e| Error::new(ErrorKind::Other, e))?;
            } else if token == "IF" {
                //lo mismo que con set word
                //creo el vector de instrucciones iterando hasta un else o then
                //luego devuelvo mi operacion IF y ademas actualizo mi indice
                //para que itere pasado el then

                i = setIf(&vector, i, &mut pila, &diccionario)
                    .map_err(|e| Error::new(ErrorKind::Other, e))?;
                //una vez tengo mi if definido, lo ejecuto basado en el trait Operacion
            } else {
                //caso donde es una instruccion
                //si la instruccion es valida en el diccionario,
                //la creo y la ejecuto
                if diccionario.pertenece(token) {
                    let instruccion = setOperacion(token, &mut diccionario)
                        .map_err(|e| Error::new(ErrorKind::Other, e))?;
                    instruccion
                        .ejecutar(&mut pila)
                        .map_err(|e| Error::new(ErrorKind::Other, e))?;
                } else {
                    return Err(Error::new(ErrorKind::Other, "input invalido"));
                }
            }
            i += 1;
        }
    }
    escribir_Stack_fth(&mut pila)?;

    Ok(())
}

//como en setWord, luego del if viene un conjunto de operaciones,
//si me encuentro con un else, voy a tener dos conjuntos de operaciones
//la idea es que luego de iterar el indice de main ya continue luego del then
pub fn setIf(
    vector: &[&str],
    indice: usize,
    pila: &mut Pila<i32>,
    diccionario: &Hash_Forth,
) -> Result<usize, &'static str> {
    let mut cuerpo: Vec<&dyn Operacion> = Vec::new();
    let mut contador = indice + 1;

    while contador < vector.len() {
        let token = vector[contador];
        if token == "THEN" {
            let if_then = If_then::new(cuerpo);
            if_then.ejecutar(pila)?;
            return Ok(contador);
        } else if token == "ELSE" {
            return setIf_Else(vector, contador, &mut cuerpo, pila, diccionario);
        } else {
            let instruccion = setOperacion(token, diccionario)?;
            cuerpo.push(instruccion);
        }
        contador += 1;
    }

    Err("Falta cerrar el IF")
}

pub fn setIf_Else(
    vector: &[&str],
    indice: usize,
    cuerpo_primero: &mut Vec<&dyn Operacion>,
    pila: &mut Pila<i32>,
    diccionario: &Hash_Forth,
) -> Result<usize, &'static str> {
    let mut cuerpo_segundo: Vec<&dyn Operacion> = Vec::new();
    let mut contador = indice + 1;

    while contador < vector.len() {
        let token = vector[contador];
        if token == "THEN" {
            let if_else_then = If_else_then::new(std::mem::take(cuerpo_primero), cuerpo_segundo);
            if_else_then.ejecutar(pila)?;
            return Ok(contador);
        } else {
            let instruccion = setOperacion(token, diccionario)?;
            cuerpo_segundo.push(instruccion);
        }
        contador += 1;
    }

    Err("Falta cerrar el If_else")
}

pub fn setWord<'a>(
    vector: &[&str],
    indice: usize,
    diccionario: &mut Hash_Forth,
) -> Result<usize, &'static str> {
    let nombre = if let Some(nombre) = vector.get(indice + 1) {
        if nombre.parse::<i16>().is_ok() {
            return Err("word invalida");
        }
        nombre
    } else {
        return Err("nombre invalido");
    };

    let cuerpo: Vec<&dyn Operacion> = Vec::new();
    let mut contador = indice + 2;

    while contador < vector.len() {
        let token = vector[contador];

        if token == ";" {
            break;
        } else {
            if let Some(instruccion) = diccionario.obtener_operacion(token) {
                // cuerpo.push(instruccion);
            } else {
                return Err("operacion no soportada");
            }
        }
        contador += 1;
    }

    for op in &cuerpo {
        if !diccionario.pertenece(op.getNombre()) {
            return Err("cuerpo invalido");
        }
    }

    let word = Word::new(nombre.to_string(), cuerpo);

    diccionario.agregar_word(nombre.to_string(), Box::new(word));

    Ok(contador)
}

pub fn setOperacion<'a>(
    instruccion: &str,
    diccionario: &'a Hash_Forth,
) -> Result<&'a dyn Operacion, &'static str> {
    diccionario
        .obtener_operacion(instruccion)
        .ok_or("operacion no soportada")
}

//luego de setOperacion, si el nombre de la operacion existe
// pub fn matchearOperacion(instruccion: &str)-> Operacion{

// }

//una vez termino de ejecutar las instrucciones de la pila,
//voy escribiendo en un archivo los elementos que voy desapilando
pub fn escribir_Stack_fth(pila: &mut Pila<i32>) -> io::Result<()> {
    let mut stack_fth = File::create("stack.fth")?;
    writeln!(stack_fth, "Tope de la PIla: ")?;
    while !pila.esta_vacia() {
        let e1 = pila
            .desapilar()
            .ok_or_else(|| Error::new(ErrorKind::Other, "Error desapilando"))?;
        writeln!(stack_fth, "{}", e1)?;
    }
    Ok(())
}
