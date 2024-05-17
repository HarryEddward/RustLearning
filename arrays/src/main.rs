fn main() {
    
                // 0, 1, 2, 3, 4 // -> INDICES
    let _numeros = [1, 2, 3, 4, 5];
    
    let mut numeros: [i32; 5] = [1, 2, 3, 4, 5];
    
    println!("{:?}", numeros);

    let valores = [5.5; 10];
    println!("{:?}", valores);

    let primer_elemento = numeros[0];
    let ultimo_elemento = numeros[ numeros.len() - 1];

    numeros[2] = 30;

    println!("{:?}", primer_elemento);
    println!("{:?}", ultimo_elemento);

}
