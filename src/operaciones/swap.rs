// use crate::Pila;
use crate::operaciones::operacion_trait::Operacion;
use crate::Pila::Pila;
pub struct Swap {
    nombre: String,
}
impl Swap {
    pub fn new() -> Self {
        Swap {
            nombre: "SWAP".to_string(),
        }
    }
}

impl Operacion for Swap {
    fn ejecutar(&self, pila: &mut Pila<i32>) -> Result<(), &'static str> {
        if pila.largo() < 2 {
            return Err("La pila no tiene suficientes elementos para realizar swap");
        }
        let e1 = pila.desapilar().ok_or("Pila vacia")?;
        let e2 = pila.desapilar().ok_or("Pila vacia")?;

        pila.apilar(e2);
        pila.apilar(e1);

        Ok(())
    }
    fn getNombre(&self) -> &str {
        return &self.nombre;
    }
}
