use crate::Pila;
use crate::operaciones::Operacion;


struct Emit{
    nombre: String,
}
impl Emit{
    pub fn new()->Self{
        Emit{ nombre: "EMIT".to_string(),}
    }
}

//imprimir el numero de la pila, pero traduciendolo segun el codigo ascii
impl Operacion for Punto{
    fn ejecutar(&self,pila: &mut Pila<i32>)->Result<(), &'static str>{
        let e1=pila.desapilar().ok_or("Error desapilando")?;
        if let Ok(caracter) = char::from_u32(e1 as u32) {
            println!("{}", caracter); 
            Ok(())
        } else {
            Err("El valor no es un carácter ASCII válido")
        }
        Ok(())
    }
    pub fn getNombre()->String{ return self.nombre;}

}