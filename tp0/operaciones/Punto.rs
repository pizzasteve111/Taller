use crate::Pila;
use crate::operaciones::Operacion;

//el operador . imprime el tope de la pila

struct Punto{
    nombre: String,
}
impl Punto{
    pub fn new()->Self{
        Punto{ nombre: ".".to_string(),}
    }
}

impl Operacion for Punto{
    fn ejecutar(&self,pila: &mut Pila<i32>)->Result<(), &'static str>{
        let e1=pila.desapilar().ok_or("Error desapilando")?;
        println!("{}",e1);
        Ok(())
    }
    pub fn getNombre()->String{ return self.nombre;}

}