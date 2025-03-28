use crate::operaciones::operacion_trait::Operacion; // use crate::Pila;
use crate::Pila::Pila;

pub struct Not {
    nombre: String,
}
impl Not {
    pub fn new() -> Self {
        Not {
            nombre: "NOT".to_string(),
        }
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
    fn getNombre(&self) -> &str {
        return &self.nombre;
    }
}
