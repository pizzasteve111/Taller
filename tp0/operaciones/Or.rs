use crate::Pila;
use crate::operaciones::Operacion;

pub struct Or{
    nombre: String,
}
impl Or{
    pub fn new()->Self{
        Or{ nombre: "OR".to_string(),}
    }
}

impl Operacion for Or {
    fn ejecutar(&self, pila: &mut Pila<i32>) -> Result<(), &'static str> {
        let e1 = pila.desapilar().ok_or("Error desapilando")?;
        let e2 = pila.desapilar().ok_or("Error desapilando")?;

        if e1 != 0 || e2 != 0 {
            pila.apilar(-1);
        } else {
            pila.apilar(0);
        }
        Ok(())
    }
    pub fn getNombre()->String{ return self.nombre;}

}