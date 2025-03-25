use crate::Pila;
use crate::operaciones::Operacion;

struct Division{
    nombre: String,
}
impl Division{
    pub fn new()->Self{
        Division{ nombre: "/".to_string(),}
    }
}

impl Operacion for Division{
    fn ejecutar(&self,pila: &mut Pila<i32>)-> Result<(), &'static str>{
        let e1=pila.desapilar().ok_or("Error desapilando")?;
        let e2=pila.desapilar().ok_or("Error desapilando")?;
        //haria e2/e1
        if e1==0 {
            return Err("Error: División por cero");
        }
        pila.apilar(e2/e1);
        Ok(())
    }
    pub fn getNombre()->String{ return self.nombre;}

}