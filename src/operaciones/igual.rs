use crate::operaciones::operacion_trait::Operacion; // use crate::Pila;
use crate::Pila::Pila;
pub struct Igual {
    nombre: String,
}
impl Igual {
    pub fn new() -> Self {
        Igual {
            nombre: "=".to_string(),
        }
    }
}

impl Operacion for Igual {
    fn ejecutar(&self, pila: &mut Pila<i32>) -> Result<(), &'static str> {
        let e1 = pila.desapilar().ok_or("Error desapilando")?;
        let e2 = pila.desapilar().ok_or("Error desapilando")?;

        if e1 == e2 {
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
