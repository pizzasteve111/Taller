use crate::Pila;
use crate::operaciones::Operacion;

struct Over{
    nombre: String,
}
impl Over{
    pub fn new()->Self{
        Over{ nombre: "OVER".to_string(),}
    }
}

impl Operacion for Over{
    fn ejecutar(&self,pila: &mut Pila<i32>)->Result<(), &'static str>{
        if pila.largo()<2{
            return Err("La pila no tiene suficientes elementos para realizar swap");
    
        }
        
        let e1=pila.desapilar().ok_or("Pila vacia")?;
        let e2=pila.desapilar().ok_or("Pila vacia")?;
    
        pila.apilar(e2);
        pila.apilar(e1);
        pila.apilar(e2);
    
        Ok(())

    }
    pub fn getNombre()->String{ return self.nombre;}

}