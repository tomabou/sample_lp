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

fn argmax(v: &Vec<f64>) -> Option<usize>{
    assert!(v.len() >0);
    let mut max = v[0];
    let mut index = 0;
    for (i,x) in v.into_iter().enumerate() {
        max = if max > *x {max} else {index = i; *x};
    }
    if max > 0.0 {Some(index)} else {None}
}

#[test]
fn test_argmax(){
    let v: Vec<f64> = vec![2.0,3.0,1.0];
    let i = argmax(&v).unwrap();
    assert_eq!(i,1);
    let v: Vec<f64> = vec![-2.0,-3.0,-1.0];
    let i = argmax(&v);
    assert_eq!(i,None);
}


use std::f64;

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
    pub fn solve(&mut self){
        let i = self.choose_column().unwrap();
        self.pivot(i);
    }
    fn choose_column(&self) -> Option<usize>{
        argmax(&self.c)
    }
    fn pivot(&mut self,i: usize) -> Option<()>{ 
        let j = self.ratio_test(i)?;

        Some(())
    }
    fn ratio_test(&self, i: usize) -> Option<usize>{
        let mut index = 0;
        let mut min =  f64::INFINITY;
        for j in 0..self.base.len(){
            if self.b[j] < 0.0 {
                continue;
            }
            let ratio = self.b[j] / self.a[i][j];
            min = if ratio > min {
                index = i;
                ratio
            } else {min};
        }        
        if min == f64::INFINITY {None} else {Some(index)}
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