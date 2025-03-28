use crate::operaciones::operacion_trait::Operacion; // use crate::Pila;
use crate::Pila::Pila;

pub struct Numero {
    valor: i32,
}

impl Numero {
    pub fn new(valor: i32) -> Self {
        Numero { valor }
    }
}

impl Operacion for Numero {
    fn ejecutar(&self, pila: &mut Pila<i32>) -> Result<(), &'static str> {
        pila.push(self.valor); // Apilar el número en el stack
        Ok(())
    }

    fn getNombre(&self) -> &str {
        "NUMERO" // Nombre genérico para números
    }
}
