use crate::Pila;
use crate::operaciones::Operacion;

struct Drop;
impl Drop{
    pub fn new()->Self{
        Drop{ nombre: "DROP".to_string(),}
    }
}

impl Operacion for Drop{
    fn ejecutar(&self,pila: &mut Pila<i32>)->Result<(), &'static str>{
        pila.desapilar().ok_or("Pila vacia")?;
        Ok(())

    }
    pub fn getNombre()->String{ return self.nombre;}

}