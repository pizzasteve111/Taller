use crate::operaciones::operacion_trait::Operacion; // use crate::Pila;
use crate::Pila::Pila;

pub struct Suma {
    nombre: String,
}
impl Suma {
    pub fn new() -> Self {
        Suma {
            nombre: "+".to_string(),
        }
    }
}

impl Operacion for Suma {
    fn ejecutar(&self, pila: &mut Pila<i32>) -> Result<(), &'static str> {
        let e1 = pila.desapilar().ok_or("Error desapilando")?;
        let e2 = pila.desapilar().ok_or("Error desapilando")?;
        pila.apilar(e1 + e2);

        Ok(())
    }
    fn getNombre(&self) -> &str {
        return &self.nombre;
    }
}
