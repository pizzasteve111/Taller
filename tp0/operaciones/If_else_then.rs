use crate::Pila;
use crate::operaciones::Operacion;

//este struct recibe dos tipos de instrucciones
//uno para el caso donde el tope es true
//y otro para el caso donde es false
pub struct If_else_then{
    
    instrucciones_true: Vec<Box<dyn Operacion>>,
    instrucciones_false: Vec<Box<dyn Operacion>>,
    nombre: String,
}

impl If_else_then{
    pub fn new(instrucciones_true: Vec<Box<dyn Operacion>>,instrucciones_false: Vec<Box<dyn Operacion>>)->Self{
        If_then{instrucciones_true,instrucciones_false,nombre: "IF_THEN_ELSE".to_string()}
    }
}

impl Operacion for If_else_then {
    fn ejecutar(&self, pila: &mut Pila<i32>) -> Result<(), &'static str> {
        let e1 = pila.desapilar().ok_or("Error desapilando")?;

        
        if e1!=0{
            for inst in &self.instrucciones_true{
                inst.ejecutar(pila)?;
            }
        } else{
            for inst in &self.instrucciones_false{
                inst.ejecutar(pila)?;
            }
        }

        Ok(())
        
    }
    pub fn getNombre()->String{ return self.nombre;}

}