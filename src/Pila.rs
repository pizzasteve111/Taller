const CAP_MAX: usize = 128 * 1024;

pub struct Pila<T> {
    elementos: Vec<T>,
    //Luego agregar la capacidad que es de 128 kb
}

impl<T> Pila<T> {
    pub fn new() -> Self {
        Pila {
            elementos: Vec::new(),
        }
    }
    pub fn apilar(&mut self, item: T) -> Result<(), &'static str> {
        if self.elementos.len() > CAP_MAX {
            return Err("cap max excedida");
        }
        self.elementos.push(item);
        Ok(())
    }
    pub fn desapilar(&mut self) -> Option<T> {
        self.elementos.pop()
    }
    pub fn ver_primero(&self) -> Option<&T> {
        self.elementos.last()
    }
    pub fn largo(&self) -> usize {
        self.elementos.len()
    }
    pub fn esta_vacia(&self) -> bool {
        self.elementos.is_empty()
    }
}
