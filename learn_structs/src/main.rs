use std::io;
#[derive(Debug, Default)]
struct Car {
    engine: String,
    year: i32,
}

impl Car {
    fn print_car(&self) {
        println!("Here and bmwcar is : {} and {}", self.engine, self.year);
    }
    fn get_newer(&self, other: &Car) {
        if self.year < other.year {
            println!("{} is older ", self.engine)
        } else {
            println!("We rock new car baby !");
        }
    }
}

fn main() {
    let mut bmwcar = Car::default();
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    if choice.contains("BMW") {
        println!("What engine is your car ? ");
        io::stdin()
            .read_line(&mut bmwcar.engine)
            .expect("Failed to read line");
        bmwcar.engine = bmwcar.engine.trim().to_string();
        println!("And what year was it made ?");
        choice.clear();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        bmwcar.year = choice.trim().parse().expect("Please write a number !");
    }
    let ford_car = Car {
        engine: "Ford".to_string(),
        year: 2025,
    };
    bmwcar.print_car();
    bmwcar.get_newer(&ford_car);
}
