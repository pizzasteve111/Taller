use crate::operaciones::operacion_trait::Operacion; // use crate::Pila;
use crate::Pila::Pila;

pub struct Cr {
    nombre: String,
}
impl Cr {
    pub fn new() -> Self {
        Cr {
            nombre: "CR".to_string(),
        }
    }
}

impl Operacion for Cr {
    fn ejecutar(&self, pila: &mut Pila<i32>) -> Result<(), &'static str> {
        println!();
        Ok(())
    }
    fn getNombre(&self) -> &str {
        return &self.nombre;
    }
}
