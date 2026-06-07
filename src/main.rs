//Curso rust

//constantes que nunca cambian su valor y se ponen fuera de las funciones:
const PI: f64 = 3.1416;
//las constantes siempre van en mayusculas
const MENSAJE: &str = "Que mal :(";
fn main() {
    //estas variables son inmutables
    let entero: i64 = 19216800;
    let decimal: f64 = 128.01111;
    let boleano: bool = true;
    let caracter: char = 'A';
    let mistring: &str = "hola"; //Usa &str para string o String
    //let mistring2: String = "putos"; asi no es
    println!("Hello, world!");
    println!("entero: {}", entero);
    println!("decimal: {}", decimal);
    println!("boleano: {}", boleano);
    println!("caracter: {}", caracter);
    println!("mi string: {}", mistring);

    //tupla recuerda las posiciones: 0, 1, 2, 3; todo inicia con 0
    let mitupla: (i32, f64, char, &str) = (72, 6.46, 'A', "holi");
    println!("mi tupla: {}", mitupla.3);

    //array, aqui definimos numeros de 64 bits y recordamos las posiciones: 0,1,2,3,4;
    let miarray: [i64; 5] = [1,2,3,4,5];
    println!("mi numero favorito: {}", miarray[2]);

    //variables mutables
    let mut contadores: i32 = 0;
    contadores = 12;
    println!("contadores: {}", contadores);

    //imrpimir PI
    println!("Valor de PI: {}", PI);

    //Operaciones aritmeticas RUST!!!!!!!!!!;
    let num1: i64 = 12;
    let num2: i64 = 10;
    let mut resultado: i64 = num1+num2;
    println!("Resultado: {}", resultado);

    //Operadores logicos: comparar con Falso && Verdadero = Falso, !Verdadero o !Falso lo pasa a viceversa: !V = F, el || pasa todo a verdadero.
    let jk: bool = true;
    let kj: bool= false;
    println!("prueba1: {}", jk && kj); //da negativo
    println!("prueba 2: {}", !jk); //da negativo
    println!("prueba 3: {}", jk || kj); //da positivo siempre

    //comparacion: 10 == 10 es igual, != desigual de 10 != 10, 10 > mayor 10, 10 < menor que 10, se puede como >= o <=


    //if y else: con ==
    let numerocorto: i16 = 12;
    if numerocorto == 12 {
        println!("muy bien");
    } else {
        println!("{}", MENSAJE);
    }
    //con diferencia
    if numerocorto != 12 {
        println!("muy bien");
    } else {
        println!("{}", MENSAJE);
    }
    //con menor y igual
    if numerocorto <= 12 {
        println!("muy bien");
    } else {
        println!("{}", MENSAJE);
    }
    //menor
    if numerocorto < 12 {
        println!("muy bien");
    } else {
        println!("{}", MENSAJE);
    }

    //con else if:

    if numerocorto != 12 {
        println!("muy bien");
    } else if numerocorto < 50 {
        println!("si, es menor");
    } else {
        println!("{}", MENSAJE);
    }

    //usando la variable numerocorto usaremos match para comparar:
    let numerocorto4 = 4;
    match numerocorto4 {
        1 => println!("no"),
        2 => println!("no"),
        3 => println!("no"),
        4 => println!("ok"),
        _ => println!("no hay nada"),
    }

    
}
