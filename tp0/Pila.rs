const CAP_MAX: usize= 128*1024

struct Pila<T>{
    elementos: Vec<T>,
    //Luego agregar la capacidad que es de 128 kb
}

impl<T> Pila<T> {
    fn new() -> Self{
        Pila {elementos: Vec::new()}
    }
    fn apilar(&mut self, item:T){
        if self.elementos.len()>CAP_MAX{
            return Err("cap max excedida")
        }
        self.elementos.push(item);
        Ok(())
    }
    fn desapilar(&mut self) -> Option<T>{
        self.elementos.pop();
    }
    fn ver_primero(&self) -> Option<%T>{
        self.elementos.last();
    }
    fn largo(&self) -> usize{
        self.elementos.len()
    }
    fn esta_vacia(&self) -> bool{
        self.elementos.is_empty()
    }

}