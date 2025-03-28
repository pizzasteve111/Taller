use std::collections::HashMap;
//este struct define el hash de funciones
use crate::operaciones::operacion_trait::Operacion;
use crate::operaciones::{
    and::And, cr::Cr, drop::Drop, dup::Dup, emit::Emit, esMayorQue::EsMayorQue,
    esMenorQue::EsMenorQue, igual::Igual, not::Not, or::Or, over::Over, producto::Producto,
    punto::Punto, puntocomillas::Punto_comillas, resta::Resta, suma::Suma, swap::Swap,
};

//las claves del hash representan las operaciones validas
//el valor asociado a dicha operacion es el struct con su implementacion interna
//Ej + ===> Struct Suma pero si re defino el simbolo +
//queda + ====> Nuevo Struct
pub struct Hash_Forth {
    diccionario: HashMap<String, Box<dyn Operacion>>,
}

impl Hash_Forth {
    pub fn new() -> Self {
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
        diccionario.insert(
            ".\"".to_string(),
            Box::new(Punto_comillas::new("".to_string())),
        );
        diccionario.insert(">".to_string(), Box::new(EsMayorQue::new()));
        diccionario.insert("<".to_string(), Box::new(EsMenorQue::new()));
        diccionario.insert("=".to_string(), Box::new(Igual::new()));

        Hash_Forth { diccionario }
    }

    pub fn agregar_word(&mut self, nombre_word: String, word: Box<dyn Operacion>) {
        //si la palabra ya existe en el diccionario,
        //se reemplaza el valor
        //si no existia, se agrega clave-valor

        self.diccionario.insert(nombre_word, word);
    }
    pub fn obtener_operacion(&self, clave: &str) -> Option<&dyn Operacion> {
        self.diccionario.get(clave).map(|op| op.as_ref())
    }
    pub fn pertenece(&self, nombre: &str) -> bool {
        self.diccionario.contains_key(nombre)
    }
}
