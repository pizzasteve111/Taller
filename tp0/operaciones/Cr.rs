use crate::Pila;
use crate::operaciones::Operacion;


struct Cr{
    nombre: String,
}
impl Cr{
    pub fn new()->Self{
        Cr{ nombre: "CR".to_string(),}
    }
}

impl Operacion for Cr{
    fn ejecutar(&self,pila: &mut Pila<i32>)->Result<(), &'static str>{
        println();
        Ok(())
    }
    pub fn getNombre()->String{ return self.nombre;}

}