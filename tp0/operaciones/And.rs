use crate::Pila;
use crate::operaciones::Operacion;

pub struct And{
    nombre: String,
}
impl And{
    pub fn new()->Self{
        And{ nombre: "AND".to_string(),}
    }
}

impl Operacion for And {
    fn ejecutar(&self, pila: &mut Pila<i32>) -> Result<(), &'static str> {
        let e1 = pila.desapilar().ok_or("Error desapilando")?;
        let e2 = pila.desapilar().ok_or("Error desapilando")?;

        if e1 == 0 || e2 == 0 {
            pila.apilar(0);
        } else {
            pila.apilar(-1);
        }
        Ok(())
    }
    pub fn getNombre()->String{ return self.nombre;}
}