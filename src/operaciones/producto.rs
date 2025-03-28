use crate::operaciones::operacion_trait::Operacion; // use crate::Pila;
use crate::Pila::Pila;

pub struct Producto {
    nombre: String,
}
impl Producto {
    pub fn new() -> Self {
        Producto {
            nombre: "*".to_string(),
        }
    }
}

impl Operacion for Producto {
    fn ejecutar(&self, pila: &mut Pila<i32>) -> Result<(), &'static str> {
        let e1 = pila.desapilar().ok_or("Error desapilando")?;
        let e2 = pila.desapilar().ok_or("Error desapilando")?;
        pila.apilar(e1 * e2);

        Ok(())
    }
    fn getNombre(&self) -> &str {
        return &self.nombre;
    }
}
