use crate::Pila;
use crate::operaciones::Operacion;

//al crear el struct if_then, tiene un atributo de instrucciones
//en caso de entrar en el if, ejecutamos las operaciones una por una
pub struct If_then{
    //uso la estructura de Box para que no se consuman los elementos del vector en ejecucion
    instrucciones: Vec<Box<dyn Operacion>>,
    nombre: String,
}

impl If_then{
    pub fn new(instrucciones: Vec<Box<dyn Operacion>>)->Self{
        If_then{instrucciones, nombre: "IF_THEN".to_string()}
    }
}

//Los struct condicionales tienen atributos propios, al crear el condicional if_then, este tiene como atributo el bloque de datos
//de instrucciones que viene luego del if

//se desapila el tope, si este es true, entramos al if, sino no hacemos nada para esta instruccion
//si no entramos al if, el conjunto de instrucciones sigue normalmente
impl Operacion for If_then {
    fn ejecutar(&self, pila: &mut Pila<i32>) -> Result<(), &'static str> {
        let e1 = pila.desapilar().ok_or("Error desapilando")?;

        //si el tope de la pila es true, ejecutamos las instrucciones
        if e1!=0{
            for inst in &self.instrucciones{
                inst.ejecutar(pila)?;
            }
        }

        //si no se entra en el if, esta instruccion no hace nada
        Ok(())
        
    }
    pub fn getNombre()->String{ return self.nombre;}

}