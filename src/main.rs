use std::io;

fn main() {
    println!("Hello, world!");
    println!("Start LP");

    let c = Canonical::sample();
    let d = Dict::from_canonical(&c);
    println!("{:?}",d);
}

#[derive(Debug)]
struct Dict{
    unbase: Vec<i64>,
    base: Vec<i64>,
    a: Vec<Vec<f64>>,    
    b: Vec<f64>,
    c: Vec<f64>,
    max: f64,
}

impl Dict{
    fn from_canonical (can: &Canonical) -> Dict{
        let base_num = can.b.len() as i64;
        let unbase_num = can.c.len() as i64;
        Dict{
            unbase: (0..base_num).collect(),
            base: (base_num..(base_num+unbase_num)).collect(),
            a: can.a.clone(),
            b: can.b.clone(),
            c: can.c.clone(),
            max : 0.0,
        }
    }
}

#[derive(Debug)]
struct Canonical {
    c: Vec<f64>,
    a: Vec<Vec<f64>>,
    b: Vec<f64>,
}

impl Canonical{
    fn sample() -> Canonical{
        let c = vec![3.0,2.0];
        let b = vec![4.0,5.0,7.0];
        let mut a = Vec::new();
        a.push(vec![1.0,1.0]);
        a.push(vec![2.0,0.0]);
        a.push(vec![2.0,1.0]);
        Canonical{
            a:a,
            b:b,
            c:c
        }
    }
}   