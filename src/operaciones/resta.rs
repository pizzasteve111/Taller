use crate::operaciones::operacion_trait::Operacion; // use crate::Pila;
use crate::Pila::Pila;

pub struct Resta {
    nombre: String,
}
impl Resta {
    pub fn new() -> Self {
        Resta {
            nombre: "-".to_string(),
        }
    }
}
impl Operacion for Resta {
    fn ejecutar(&self, pila: &mut Pila<i32>) -> Result<(), &'static str> {
        let e1 = pila.desapilar().ok_or("Error desapilando")?;
        let e2 = pila.desapilar().ok_or("Error desapilando")?;
        pila.apilar(e2 - e1);

        Ok(())
    }
    fn getNombre(&self) -> &str {
        return &self.nombre;
    }
}
