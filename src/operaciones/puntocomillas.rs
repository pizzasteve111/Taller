use crate::operaciones::operacion_trait::Operacion; // use crate::Pila;
use crate::Pila::Pila;

//el operador . imprime el tope de la pila

pub struct Punto_comillas {
    nombre: String,
    mensaje: String,
}
impl Punto_comillas {
    pub fn new(mensaje: String) -> Self {
        Punto_comillas {
            nombre: ".`` ".to_string(),
            mensaje: mensaje,
        }
    }
}

impl Operacion for Punto_comillas {
    fn ejecutar(&self, pila: &mut Pila<i32>) -> Result<(), &'static str> {
        println!("{}", self.mensaje);
        Ok(())
    }
    fn getNombre(&self) -> &str {
        return &self.nombre;
    }
}
