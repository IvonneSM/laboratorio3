use std::io::stdin;

fn is_isbn10(cadena: &str) -> bool{
    // crear y comentar esta función
    // se va atravesando todos los digitos del numero limpio y se pasa a i64, luego se multiplica con el "i" que es un contador que va bajando
    //de uno en uno desde el 10 y el numero se va sumando a la variable resultado con cada iteracion, luego se pregunta si el numero resultado modulo 11 es igual a 0
    let mut resultado: i64 = 0;
    let mut i: i64 = 10;
    for num in cadena.chars() { 
        let temp_num: i64 = num.to_string().parse::<i64>().unwrap();
        resultado += temp_num * i;
        i -= 1;
        
    }
    if resultado % 11 == 0 {
        return true
    }else {
        return false
    }
        
}

fn is_isbn_format_valid(c: &str) -> bool { //funcion de validacion que entregara un valor tipo booleano 
 // comentar esta funcion
// en el if se esta iterando y dividiendo caracter por caracter con el chars y se esta creando una lista y retornara verdadero si no hay una x como digito verficador
//sino retornara un true si hay una x o X como numero verificador.
    if c.chars().next().unwrap().is_numeric() { 
        return true;
    } else if c == "X" || c == "x"{ 
        return true;
    }
    return false // si no esta el digito de 0 a 9 o la x, X retornara un false
}

fn main(){

    let mut isbn: String = String::new();
    let mut clean_isbn: String = String::new();
    stdin().read_line(&mut isbn).unwrap();
   
    // comentar este ciclo
//el el ciclo for la cadena de string se esta limpiando (/n) y dividiendo caracter por caracter.
//if se esta validando el numero de identificacion
// se la variable se le da un valor entre la suma de ella misma y el digito verificador
    for c in isbn.to_string().trim().chars(){
        if is_isbn_format_valid(&c.to_string()){
            clean_isbn = clean_isbn + &c.to_string(); 
        }
    }

    println!("{}", clean_isbn);
    // comentar esta sentencia
    //esta sentencia esta evaluando si en la funcion is_isbn10 esta el digito verificador correcto si es x o un numero de 0 a 9
    //para printear que es valido segun el numero verificador y el largo correcto y si no dira que este no lo es
    if is_isbn10(&clean_isbn){ 
        println!("{} es un ISBN10 valido", clean_isbn);
    } else {
        println!("No lo es");
    }

}