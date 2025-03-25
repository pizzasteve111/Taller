mod Error; 
use Error::Error;

struct Invalid_word{
    mensaje: String,
}

impl Invalid_word{
    pub fn new() -> Self{
        Invalid_word{mensaje:"word invalida"}
    }
}

impl Error for Invalid_word{
    

    fn imprimir(&self){
        println!("Error: {}", self.mensaje);
    }
}