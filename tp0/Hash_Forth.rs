use std::collections::HashMap;
//este struct define el hash de funciones

mod operaciones; // Declara el módulo operaciones
use operaciones::*; 
use crate::operaciones::{Operacion, Suma, Resta, Producto, And, Or, Not, Dup, Drop, Swap, Over, Cr, Emit, Punto, Punto_comillas, EsMayorQue, EsMenorQue, Igual};


//las claves del hash representan las operaciones validas
//el valor asociado a dicha operacion es el struct con su implementacion interna
//Ej + ===> Struct Suma pero si re defino el simbolo +
//queda + ====> Nuevo Struct
struct Hash_Forth{
    diccionario: HashMap<String, Box<dyn Operacion>>,
}

impl Hash_Forth{
    pub fn new() -> Self{
        //seteo todas mis operaciones
        let mut diccionario: HashMap<String, Box<dyn Operacion>> = HashMap::new();

        // Agregar operaciones básicas al diccionario
        diccionario.insert("+".to_string(), Box::new(Suma::new()));
        diccionario.insert("-".to_string(), Box::new(Resta::new()));
        diccionario.insert("*".to_string(), Box::new(Producto::new()));
        diccionario.insert("AND".to_string(), Box::new(And::new()));
        diccionario.insert("OR".to_string(), Box::new(Or::new()));
        diccionario.insert("NOT".to_string(), Box::new(Not::new()));
        diccionario.insert("DUP".to_string(), Box::new(Dup::new()));
        diccionario.insert("DROP".to_string(), Box::new(Drop::new()));
        diccionario.insert("SWAP".to_string(), Box::new(Swap::new()));
        diccionario.insert("OVER".to_string(), Box::new(Over::new()));
        diccionario.insert("CR".to_string(), Box::new(Cr::new()));
        diccionario.insert("EMIT".to_string(), Box::new(Emit::new()));
        diccionario.insert(".".to_string(), Box::new(Punto::new()));
        diccionario.insert(".\"".to_string(), Box::new(Punto_comillas::new("".to_string())));
        diccionario.insert(">".to_string(), Box::new(EsMayorQue::new()));
        diccionario.insert("<".to_string(), Box::new(EsMenorQue::new()));
        diccionario.insert("=".to_string(), Box::new(Igual::new()));

        Hash_Forth{diccionario}
        
    }

    pub fn agregar_word(&mut self,nombre_word: String, word: Box<dyn Operacion){
        //si la palabra ya existe en el diccionario,
        //se reemplaza el valor
        //si no existia, se agrega clave-valor

        if self.diccionario.contains_key(nombre_word){
            //borro la clave vieja, se va a re definir el simbolo
            diccionario.remove(nombre_word);
        }
        diccionario.insert(nombre_word,word);
    }
    pub fn obtener_operacion(&self, clave: &str)->Option<&Box<dyn Operacion>>{
        self.diccionario.get(clave);
    }
    pub fn pertenece(&self, nombre: &str)->bool{
        return diccionario.contains_key(nombre);

    }

}