use crate::operaciones::operacion_trait::Operacion; // use crate::Pila;
use crate::Pila::Pila;

pub struct Drop {
    nombre: String,
}
impl Drop {
    pub fn new() -> Self {
        Drop {
            nombre: "DROP".to_string(),
        }
    }
}

impl Operacion for Drop {
    fn ejecutar(&self, pila: &mut Pila<i32>) -> Result<(), &'static str> {
        pila.desapilar().ok_or("Pila vacia")?;
        Ok(())
    }
    fn getNombre(&self) -> &str {
        return &self.nombre;
    }
}
