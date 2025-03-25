mod Error; 
use Error::Error;


struct Division_zero{
    mensaje: String,
}

impl Division_zero{
    pub fn new() -> Self{
        Division_zero{mensaje:"se trata de dividir por cero"}
    }
}

impl Error for Division_zero{
    
    fn imprimir(&self){
        println!("Error: {}", self.mensaje);
    }
}