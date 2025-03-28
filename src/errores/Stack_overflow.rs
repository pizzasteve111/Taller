mod Error; 
use Error::Error;

struct Stack_overflow{
    mensaje: String,
}

impl Stack_overflow{
    pub fn new() -> Self{
        Stack_overflow{mensaje:"una operación intenta pushear un elemento a una pila que se encuentra en su capacidad máxima de memoria."}
    }
}


impl Error for Stack_overflow{
    

    fn imprimir(&self){
        println!("Error: {}", self.mensaje);
    }
}