use std::io;


fn main() {
    
    println!("Ingresa el nombre del usuario: ");
    let mut username = String::new();

    //Prestamos -> Read & Write -> &mut
    io::stdin().read_line(&mut username); //Ref


    let username = username.trim();

    println!("Ingresa la edad del usuario");
    let mut edad = String::new();

    io::stdin().read_line(&mut edad);
    let edad = edad.trim();

    let edad: i8 = edad.parse().unwrap();
    println!("El valor de el nombre es: {}", &username);
    println!("El valor de la edad es: {}", &edad);
}
