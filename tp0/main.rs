mod Pila; 
mod operaciones; 
use crate::operaciones::{Operacion, Suma, Resta, Producto, And, Or, Not, Dup, Drop, Swap, Over, Cr, Emit, Punto, Punto_comillas, EsMayorQue, EsMenorQue, Igual};

use Pila::Pila; 
use operaciones::*;
use std::collections::HashMap;
mod Hash_Forth;
use Hash_Forth::Hash_Forth;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::{self, Write};

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
pub fn main(ruta_archivo: &str) -> io::Result<()>{
    let mut diccionario = Hash_Forth::new();
    let mut pila = Pila::new();

    let ruta = Path::new(ruta_archivo);
    let archivo = File::open(&ruta)?;

    let lector = io::BufReader::new(archivo);

    for linea in lector.lines(){
        let linea = linea?;
        let vector: Vec<&str> = linea.split_whitespace().collect();
        //manejo 4 casos
        //1) si el elemento es un numero, lo apilo en mi pila
        //2)si el elemento es un : llamo a una funcion que crea la word hasta el ;
        //3) si el elemento es if, llamo a mi funcion para crear la instruccion if
        //4) si no es ninguna de las anteriores, hago pattern matching para ver que tipo de operacion es

        for i in 0..vector.len(){
            if let Ok(vector[i]) = vector[i].parse::<i16>(){
                pila.apilar(instruccion);

            } else if *vector[i]== ":"{
                //si quiero definir word, itero hasta el ;
                //luego actualizo mi indice general para que siga iterando luego del ;
                //entonces mi funcion setWord crea la word y ademas actualiza el indice
                //para que siga iterando luego del ;
                let word Operacion=setWord(&vector,&i,&diccionario)?;
                //esto es el caso solo donde quiero definir la word
                //no la ejecuto directamente, solo quedaria en el diccionario

            } else if *vector[i]== "IF"{
                //lo mismo que con set word
                //creo el vector de instrucciones iterando hasta un else o then
                //luego devuelvo mi operacion IF y ademas actualizo mi indice
                //para que itere pasado el then
                let _if Operacion = setIf(&vector,&i);
                //una vez tengo mi if definido, lo ejecuto basado en el trait Operacion
                _if.ejecutar(pila)
            }else{
                //caso donde es una instruccion
                //si la instruccion es valida en el diccionario,
                //la creo y la ejecuto
                if es_valido(vector[i],&diccionario){
                    let instruccion Operacion=setOperacion(&vector[i],diccionario);
                    instruccion.ejecutar(&pila);
                }else{
                    println!("instruccion invalida");
                }
            }

        }
        
        //Falta la logica de ir imprimiendo por pantalla en caso de output
        //Devolver los Errores en caso de falla
    }
    escribir_Stack_fth(pila)?;

}

//como en setWord, luego del if viene un conjunto de operaciones,
//si me encuentro con un else, voy a tener dos conjuntos de operaciones
//la idea es que luego de iterar el indice de main ya continue luego del then
pub fn setIf(vector: &Vec<&str>, indice: usize)-> Operacion{
    let cuerpo=<Box<dyn Operacion>>;
    let mut contador: int= 0;

    for i in indice+1..vector.len(){
        //termino de construir mi if
        if *vector[i]=="THEN"{
            *indice=indice+contador;
            return new If_then(cuerpo);
        }else if *vector[i]=="ELSE"{
            *indice=indice+contador;
            return setIf_Else(vector,cuerpo,indice);
        }else{
            //si es una instruccion comun, va al cuerpo
            let instruccion Operacion=setOperacion(&vector[i]);
            cuerpo.push(instruccion);
        }
        *contador+=1;
    }
    
}

pub fn setIf_Else(vector: &Vec<&str>,cuerpo_primero : <Box<dyn Operacion>>, indice: usize){
    let cuerpo_segundo=<Box<dyn Operacion>>;
    let mut contador: int= 0;

    //agrego las operaciones que vienen luego del else

    for i in indice+1..vector.len(){
        if *vector[i]=="THEN"{
            *indice=indice+contador;
            return new If_else_then(cuerpo_primero,cuerpo_segundo);
        }else{
            let instruccion Operacion=setOperacion(&vector[i]);
            cuerpo_segundo.push(instruccion);
        }
        *contador+=1;
    }
}




pub fn setWord(vector: &Vec<&str>, indice: usize,diccionario: &Hash_Forth)-> Operacion{
    //en este caso, el indice apunta al : y debe iterar hasta el ;
    //el primer argumento luego del: es el cuerpo de la word
    //lo que siga comprende el body
    let nombre: String= vector[indice+1]?;
    let cuerpo=<Box<dyn Operacion>>;
    let mut contador: int= 0;
    for i in indice+2..vector.len() {
        if *vector[i]==";"{
            //al terminar de construir mi word, quiero que el indice siga iterando desde el
            //; que termina de definir la word
            *indice=indice+contador;
            return new Word(nombre,cuerpo,diccionario)?;
        }else {
            let instruccion Operacion=setOperacion(&vector[i]);
            cuerpo.push(instruccion);
            
        }
        *contador+=1;
    }

    Ok(())
}

pub fn setOperacion(instruccion: &str, diccionario: Hash_Forth) -> Operacion{
    //verificamos que dicha operacion exista en el diccionario
    if diccionario.contains_key(instruccion){
        return diccionario.obtener_operacion(instruccion);
    }else{
        println!("operacion no soportada")
    }
}

//una vez termino de ejecutar las instrucciones de la pila, 
//voy escribiendo en un archivo los elementos que voy desapilando
pub fn esribir_Stack_fth(pila:&mut Pila<i32>) -> io::Result<()>{
    let mut stack_fth = File::create("stack.fth")?;
    writeln!(stack_fth,"Tope de la PIla: ")?;
    while !pila.esta_vacia(){
        let e1=pila.desapilar().ok_or("Error desapilando")?;
        writeln!(stack_fth,"{}", e1)?;
    }
    Ok(())
}