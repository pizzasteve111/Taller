use crate::operaciones::operacion_trait::Operacion; // use crate::Pila;
use crate::Pila::Pila;

pub struct Dup {
    nombre: String,
}
impl Dup {
    pub fn new() -> Self {
        Dup {
            nombre: "DUP".to_string(),
        }
    }
}

impl Operacion for Dup {
    fn ejecutar(&self, pila: &mut Pila<i32>) -> Result<(), &'static str> {
        let e1 = pila.desapilar().ok_or("Pila vacia")?;
        pila.apilar(e1);
        pila.apilar(e1);
        Ok(())
    }
    fn getNombre(&self) -> &str {
        return &self.nombre;
    }
}
