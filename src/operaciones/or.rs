use crate::operaciones::operacion_trait::Operacion; // use crate::Pila;
use crate::Pila::Pila;

pub struct Or {
    nombre: String,
}
impl Or {
    pub fn new() -> Self {
        Or {
            nombre: "OR".to_string(),
        }
    }
}

impl Operacion for Or {
    fn ejecutar(&self, pila: &mut Pila<i32>) -> Result<(), &'static str> {
        let e1 = pila.desapilar().ok_or("Error desapilando")?;
        let e2 = pila.desapilar().ok_or("Error desapilando")?;

        if e1 != 0 || e2 != 0 {
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
