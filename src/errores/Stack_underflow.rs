mod Error; 
use Error::Error;

struct Stack_underflow{
    mensaje: String,
}

impl Stack_overflow{
    pub fn new() -> Self{
        Stack_underflow{mensaje:"Una operación intenta popear un elemento de una pila vacía"}
    }
}

impl Error for Stack_underflow{
    
    fn imprimir(&self){
        println!("Error: {}", self.mensaje);
    }
}