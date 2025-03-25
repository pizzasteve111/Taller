use crate::Pila;
use crate::operaciones::Operacion;

struct Suma{
    nombre: String,
}
impl Suma{
    pub fn new()->Self{
        Suma{ nombre: "+".to_string(),}
    }
}

impl Operacion for Suma{
    fn ejecutar(&self,pila: &mut Pila<i32>)->Result<(), &'static str>{
        let e1=pila.desapilar().ok_or("Error desapilando")?;
        let e2=pila.desapilar().ok_or("Error desapilando")?;
        pila.apilar(e1+e2);


        Ok(())
    }
    pub fn getNombre()->String{ return self.nombre;}

}