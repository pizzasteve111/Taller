use crate::operaciones::operacion_trait::Operacion; // use crate::Pila;
use crate::Pila::Pila;
use std::collections::HashMap;

// use crate::Hash_Forth;
use crate::Hash_Forth::Hash_Forth;

//el struct word tiene un nombre y un cuerpo
//el cuerpo es un conjunto de datos del tipo Operacion
//cuando queremos ejecutar la word, se recorre el arreglo de operaciones en el cuerpo
//y se las ejecuta una por una
pub struct Word<'a> {
    nombre: String,
    cuerpo: Vec<&'a dyn Operacion>,
}

impl<'a> Word<'a> {
    pub fn new(nombre: String, cuerpo: Vec<&'a dyn Operacion>) -> Self {
        Word { nombre, cuerpo }
    }
}

impl<'a> Operacion for Word<'a> {
    fn ejecutar(&self, pila: &mut Pila<i32>) -> Result<(), &'static str> {
        //los numeros son una Operacion
        //logica para apilar numeros tambien
        for instruccion in &self.cuerpo {
            instruccion.ejecutar(pila)?;
        }
        Ok(())
    }
    fn getNombre(&self) -> &str {
        return &self.nombre;
    }
}
