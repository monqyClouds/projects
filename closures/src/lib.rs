use std::{collections::HashMap, thread, time::Duration};

pub struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    pub calculation: T,
    pub value: Option<HashMap<u32, u32>>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    pub fn value(&mut self, arg: u32) -> u32 {
        match &mut self.value {
            Some(v) => *v.entry(arg).or_insert_with(|| (self.calculation)(arg)),
            None => {
                let mut values_map = HashMap::new();
                let value = (self.calculation)(arg);

                values_map.insert(arg, value);
                self.value = Some(values_map);

                value
            }
        }
    }
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups", expensive_closure.value(intensity));
        println!("Next, do {} situps", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes",
                expensive_closure.value(intensity)
            );
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let _v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2)
    }

    #[test]
    fn closure_trait() {
        let x = vec![1, 2, 3, 4];
        let equal_to_x = move |z| z == x;
        let y = vec![1, 2, 3, 4];

        assert!(equal_to_x(y));
    }
}
