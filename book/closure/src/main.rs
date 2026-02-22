use std::{collections::HashMap, thread, time::Duration};

struct Cacher<T> {
    calculation: T,
    value: HashMap<u32, u32>,
}
// TODO now, improve cacher so that it uses a hasmap for args, right ? Currently if new arg no new
// calculation.
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }
    fn assign(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
    // more idiomatic
    fn assign_idiomatic(&mut self, arg: u32) -> u32 {
        *self
            .value
            .entry(arg)
            .or_insert_with(|| (self.calculation)(arg))
    }
}

fn main() {
    let simultated_intensity = 10;
    let simultated_random_number = 7;
    geenrate_workout(simultated_intensity, simultated_random_number);
}

fn _generate_workout_idiomatic(intensity: i32, random_number: i32) {
    let expensive_closure = |num: u32| {
        println!("Calculate expensive stuff!");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let mut cacher = Cacher::new(expensive_closure);

    if intensity < 25 {
        println!(
            "Today do {} push-ups!",
            cacher.assign_idiomatic(intensity as u32)
        );
        println!(
            "Also do {} pull ups!",
            cacher.assign_idiomatic(intensity as u32)
        );
    } else if random_number == 3 {
        println!("Azi pauza boss");
    } else {
        println!("Fugi {} minute!", cacher.assign_idiomatic(intensity as u32));
    }
}

fn geenrate_workout(intesity: i32, rand_nubmer: i32) {
    let expsnv_closure = |num| {
        println!("Calucate expensive stuff !");
        thread::sleep(Duration::from_secs(2));
        num
    };
    let mut new_cacher = Cacher::new(expsnv_closure);

    if intesity < 25 {
        println!("Today do {} push-ups!", new_cacher.assign(intesity as u32));
        println!("Also do {} pull ups ", new_cacher.assign(intesity as u32));
    } else if rand_nubmer == 3 {
        println!("Azi pauza boss");
    } else {
        println!("Fugi {} minute ", new_cacher.assign(intesity as u32));
    }
}
// deprecated after cacher ?
fn _simulated_expnsv(intensity: i32) -> i32 {
    println!("Calucate expensive stuff !");
    thread::sleep(Duration::from_secs(2));
    intensity
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn caches_per_argument() {
        let calls = Rc::new(Cell::new(0));
        let calls2 = Rc::clone(&calls);

        let closure = move |x: u32| {
            calls2.set(calls2.get() + 1);
            x + 1
        };

        let mut c = Cacher::new(closure);

        // 10 first time => computes
        let a = c.assign_idiomatic(10);
        // 10 second time => should NOT compute
        let b = c.assign_idiomatic(10);
        // 20 => computes
        let d = c.assign_idiomatic(20);

        assert_eq!(a, 11);
        assert_eq!(b, 11);
        assert_eq!(d, 21);

        assert_eq!(calls.get(), 2);
    }
}
