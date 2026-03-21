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
    let mut pizza_topping = vec![pepperoni, mushroom, sausage];
    pizza_topping[1] = String::from("Olives");
    println!("{pizza_topping:?}");

    let target_topiing = &mut pizza_topping[2];
    target_topiing.push_str(" and Meatballs");
    let another_topping = &pizza_topping[2];
    println!("{pizza_topping:?}");
    // let mut delicous_toppings = pizza_topping;

    // let topping_reference = &delicous_toppings[1];

    // delicous_toppings.push(String::from("Olives"));

    //println!("The topping is {topping_reference}");

    //let value = &pizza_topping[2];

    let pizza_slice = &pizza_diameters[1..3];
    println!("{pizza_slice:?}");

    // let option = pizza_topping.get(2);
    // match option {
    //     Some(topping) => println!("The topping is {topping}"),
    //     None => println!("No value at the index position"),
    // }

    let mut seasons: Vec<&str> = Vec::with_capacity(4);
    println!(
        "Length : {}, Capacity: {}",
        seasons.len(),
        seasons.capacity()
    );

    seasons.push("Summer");
    seasons.push("Fall");
    seasons.push("Spring");
    seasons.push("Winter");

    println!(
        "Length : {}, Capacity: {}",
        seasons.len(),
        seasons.capacity()
    );

    seasons.push("Rainy");
        println!(
        "Length : {}, Capacity: {}",
        seasons.len(),
        seasons.capacity()
    );
}
