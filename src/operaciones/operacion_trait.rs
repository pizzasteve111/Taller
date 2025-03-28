use crate::Pila::Pila;

pub trait Operacion {
    fn ejecutar(&self, pila: &mut Pila<i32>) -> Result<(), &'static str>;
    fn getNombre(&self) -> &str;
}
