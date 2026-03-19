fn main() {
    let pizza_diameters: Vec<i32> = Vec::<i32>::new();
    println!("{pizza_diameters:?}");

    let pastas: Vec<&str> = Vec::new();
    let pastas = Vec::<&str>::new();
    let pastas = vec!["Rigatoni", "Angel hair", "Fettucine"];
    println!("{pastas:?}");

    let mut pizza_diameters = vec![8, 10, 12, 14];
    pizza_diameters.push(16);
    pizza_diameters.push(18);

    println!("{pizza_diameters:?}");

    pizza_diameters.insert(0, 4);
    println!("pizza insert: {pizza_diameters:?}");

    let last_pizza_diameter = pizza_diameters.pop();
    println!("last element: {last_pizza_diameter:?}");
    println!("array after pop: {pizza_diameters:?}");

    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");
    let pizza_topping = vec![pepperoni, mushroom, sausage];

    let value = &pizza_topping[2];

    let pizza_slice = &pizza_diameters[1..3];
    println!("{pizza_slice:?}");

    let option = pizza_topping.get(2);
    match option {
        Some(topping) => println!("The topping is {topping}"),
        None => println!("No value at the index position"),
    }
}
