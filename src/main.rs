// ============================================================
//  CURSO RUST
// ============================================================


// ────────────────────────────────────────────────────────────
//  CONSTANTES GLOBALES
// ────────────────────────────────────────────────────────────

const PI: f64 = 3.1416;
//las constantes siempre van en mayusculas
const MENSAJE: &str = "Que mal :(";


// ────────────────────────────────────────────────────────────
//  FUNCIONES
// ────────────────────────────────────────────────────────────

//Funciones, como ya sabemos que en fn main(Aqui va parametros para la funion) -> es el retorno de la funcion.

fn suma(numerochafa1: i32, numerochafa2: i32) -> i32 {
    return numerochafa1 + numerochafa2;
}

//funcion que nomas:

fn holaxd(){
    println!("Holii");
}


// ────────────────────────────────────────────────────────────
//  MAIN
// ────────────────────────────────────────────────────────────

fn main() {

    // ── Llamadas a funciones ─────────────────────────────────
    let resultadoo:i32  = suma(30, 50);
    println!("{}", resultadoo);

    holaxd();


    // ── Tipos de datos y variables ───────────────────────────
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


    // ── Tuplas y arrays ──────────────────────────────────────
    //tupla recuerda las posiciones: 0, 1, 2, 3; todo inicia con 0
    let mitupla: (i32, f64, char, &str) = (72, 6.46, 'A', "holi");
    println!("mi tupla: {}", mitupla.3);

    //array, aqui definimos numeros de 64 bits y recordamos las posiciones: 0,1,2,3,4;
    let miarray: [i64; 5] = [1,2,3,4,5];
    println!("mi numero favorito: {}", miarray[2]);


    // ── Variables mutables ───────────────────────────────────
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


    // ── Operadores lógicos ───────────────────────────────────
    //Operadores logicos: comparar con Falso && Verdadero = Falso, !Verdadero o !Falso lo pasa a viceversa: !V = F, el || pasa todo a verdadero.
    let jk: bool = true;
    let kj: bool= false;
    println!("prueba1: {}", jk && kj); //da negativo
    println!("prueba 2: {}", !jk); //da negativo
    println!("prueba 3: {}", jk || kj); //da positivo siempre

    //comparacion: 10 == 10 es igual, != desigual de 10 != 10, 10 > mayor 10, 10 < menor que 10, se puede como >= o <=


    // ── Condicionales — if / else if / match ─────────────────
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


    // ── Ciclos — loop / while / for ──────────────────────────
    //usando loop
    let mut numerorandom: i8 = 0;
    loop {
        //se imprime Holaaaa 5 veces porque si
        println!("Holaaaaa");
        numerorandom += 1; //se suma numerorandom mas 1 y cuando numero random sea a 5 se cancela todo
        if numerorandom == 5 {
            println!("fin");
            break;
        }
    }

    //usando while
    while numerorandom < 10 {
        println!("Que");
        numerorandom += 1;
        //solo se imprimar hasta 10 veces si es menor que 10
    }

    //uso de for
    for numerokrazy in 1..10 {
        println!("numerokrazy: {numerokrazy}");
    }
    for elemento in [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] {
        println!("elemento: {elemento}");  //imprime los numeros que estan en [] tanto en numerokrazy y elemento
    }


    // ── Bloques ──────────────────────────────────────────────
    let x: i32 = 5;
    let mibloque= {
        let z = 5;
        6+z
    };
    println!("suma: {}", mibloque);


    /*
    Ownership --------- Apartado
    */
}
