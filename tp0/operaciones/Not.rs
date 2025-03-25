use crate::Pila;
use crate::operaciones::Operacion;

pub struct Not{
    nombre: String,
}
impl Not{
    pub fn new()->Self{
        Not{ nombre: "NOT".to_string(),}
    }
}

impl Operacion for Not {
    fn ejecutar(&self, pila: &mut Pila<i32>) -> Result<(), &'static str> {
        let e1 = pila.desapilar().ok_or("Error desapilando")?;

        if e1 == 0 {
            pila.apilar(-1);
        } else {
            pila.apilar(0);
        }
        Ok(())
    }
    pub fn getNombre()->String{ return self.nombre;}

}