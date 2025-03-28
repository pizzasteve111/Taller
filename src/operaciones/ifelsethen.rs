use crate::operaciones::operacion_trait::Operacion; // use crate::Pila;
use crate::Pila::Pila;

//este struct recibe dos tipos de instrucciones
//uno para el caso donde el tope es true
//y otro para el caso donde es false
pub struct If_else_then<'a> {
    instrucciones_true: Vec<&'a dyn Operacion>,
    instrucciones_false: Vec<&'a dyn Operacion>,
    nombre: String,
}

impl<'a> If_else_then<'a> {
    pub fn new(
        instrucciones_true: Vec<&'a dyn Operacion>,
        instrucciones_false: Vec<&'a dyn Operacion>,
    ) -> Self {
        If_else_then {
            instrucciones_true,
            instrucciones_false,
            nombre: "IF_THEN_ELSE".to_string(),
        }
    }
}

impl<'a> Operacion for If_else_then<'a> {
    fn ejecutar(&self, pila: &mut Pila<i32>) -> Result<(), &'static str> {
        let e1 = pila.desapilar().ok_or("Error desapilando")?;

        if e1 != 0 {
            for inst in &self.instrucciones_true {
                inst.ejecutar(pila)?;
            }
        } else {
            for inst in &self.instrucciones_false {
                inst.ejecutar(pila)?;
            }
        }

        Ok(())
    }
    fn getNombre(&self) -> &str {
        return &self.nombre;
    }
}
