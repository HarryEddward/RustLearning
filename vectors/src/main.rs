fn main() {

    let mut vector = vec![1, 2, 3];
    let mut vector: Vec<i32> = Vec::new();

    vector.push(4);

    vector.insert(0, -1);
    vector.remove(vector.len() - 1);

    vector[1] = 10;

    let primer_elemento = vector[0];
    let ultimo_elemento = vector.pop().unwrap();


    println!("{:?}", vector);
    println!("{:?}", primer_elemento);
    println!("{:?}", ultimo_elemento);
}
