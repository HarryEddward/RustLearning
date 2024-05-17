fn main() {
    

    let tupla = (10, true, 5.5);

    println!("{:?}", tupla);


    let mut tupla: (u8, bool, f64, i32) = (2, true, 5.5, -10);
    println!("{:?}", tupla);

    let primer_elemento = tupla.0;
    let ultimo_elemento = tupla.3;

    tupla.1 = true;

    println!("{}", primer_elemento);
    println!("{}", ultimo_elemento);
    println!("{}", tupla.1)

}
