use crate::operaciones::operacion_trait::Operacion; // use crate::Pila;
use crate::Pila::Pila;

pub struct Rot {
    nombre: String,
}
impl Rot {
    pub fn new() -> Self {
        Rot {
            nombre: "ROT".to_string(),
        }
    }
}

impl Operacion for Rot {
    fn ejecutar(&self, pila: &mut Pila<i32>) -> Result<(), &'static str> {
        if pila.largo() < 3 {
            return Err("La pila no tiene suficientes elementos para realizar swap");
        }
        let e1 = pila.desapilar().ok_or("Pila vacia")?;
        let e2 = pila.desapilar().ok_or("Pila vacia")?;
        let e3 = pila.desapilar().ok_or("Pila vacia")?;

        pila.apilar(e2);
        pila.apilar(e1);
        pila.apilar(e3);

        Ok(())
    }
    fn getNombre(&self) -> &str {
        return &self.nombre;
    }
}
