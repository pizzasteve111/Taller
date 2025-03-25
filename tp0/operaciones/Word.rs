use crate::Pila;
use crate::operaciones::Operacion;
use std::collections::HashMap;
mod Hash_Forth;
use Hash_Forth::Hash_Forth;

//el struct word tiene un nombre y un cuerpo
//el cuerpo es un conjunto de datos del tipo Operacion
//cuando queremos ejecutar la word, se recorre el arreglo de operaciones en el cuerpo
//y se las ejecuta una por una
struct Word{
    nombre: String,
    cuerpo: Vec<Box<dyn Operacion>>,
    
}

impl Word{
    pub fn new(nombre: String,cuerpo: Vec<Box<dyn Operacion>>,diccionario: &mut Hash_Forth)-> Self{
        //si el nombre del word es un numero, es invalido
        if nombre.parse::<i16>.is_ok(){
            println!("nombre invalido");
        }

        for op in &cuerpo{
            if !diccionario.pertenece(op.getNombre()){
                return Err("cuerpo invalido")
            }
        }
        
        let operacion = Word{&nombre, cuerpo}
        diccionario.agregar_word(nombre,Box::new(operacion));
        
        Ok(operacion)
    }
    
}

impl Operacion for Word{
    pub fn ejecutar(&self,pila: &mut Pila<i32>,diccionario: Hash_Forth)->Result<(), &'static str>{
        for instruccion in &self.cuerpo{
            if diccionario.pertenece(instruccion.getNombre()){
                instruccion.ejecutar(pila)?;
            }else{
                return Err("instruccion no soportada");
            }
            
            }
        Ok(())
    }
    pub fn getNombre()->String{ return self.nombre;}

        
}
