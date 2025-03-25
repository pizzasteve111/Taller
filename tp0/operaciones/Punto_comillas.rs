use crate::Pila;
use crate::operaciones::Operacion;

//el operador . imprime el tope de la pila

struct Punto_comillas{
    nombre: String,
    mensaje: String
}
impl Punto_comillas{
    pub fn new(mensaje: String)->Self{
        Punto_comillas{ nombre: "."" ".to_string(), mensaje: mensaje}
    }
}

impl Operacion for Punto{
    fn ejecutar(&self,pila: &mut Pila<i32>)->Result<(), &'static str>{
        println!(mensaje);
        Ok(())
    }
    pub fn getNombre()->String{ return self.nombre;}

}