use crate::Pila;
use crate::operaciones::Operacion;

pub struct EsMenorQue{
    nombre: String,
}
impl EsMenorQue{
    pub fn new()->Self{
        EsMenorQue{ nombre: "<".to_string(),}
    }
}

impl Operacion for EsMenorQue {
    fn ejecutar(&self, pila: &mut Pila<i32>) -> Result<(), &'static str> {
        let e1 = pila.desapilar().ok_or("Error desapilando")?;
        let e2 = pila.desapilar().ok_or("Error desapilando")?;

        if e2 < e1 {
            pila.apilar(-1);
        } else {
            pila.apilar(0);
        }
        Ok(())
    }
    pub fn getNombre()->String{ return self.nombre;}

}