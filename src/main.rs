fn main() {
    println!("Hello, world!");
    println!("Start LP");

    let c = Canonical::sample();
    let mut d = Dict::from_canonical(&c);
    d.solve();
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
            unbase: (0..unbase_num).collect(),
            base: (unbase_num..(base_num+unbase_num)).collect(),
            a: can.a.clone(),
            b: can.b.clone(),
            c: can.c.clone(),
            max : 0.0,
        }
    }
    pub fn solve(&mut self) -> Option<()>{
        loop {
            let i  = match self.choose_column(){
                None=> break,
                Some(i) => i
            };
            self.pivot(i)?;
        }
        Some(())
    }
    fn choose_column(&self) -> Option<usize>{
        argmax(&self.c)
    }
    fn pivot(&mut self,piv: usize) -> Option<()>{ 
        let row = self.ratio_test(piv)?;
        println!("{}",row );
        for j in 0..self.b.len(){
            if j==row {continue};
            let ratio = self.a[j][piv] / self.a[row][piv];

            for k in 0..self.c.len(){
                if k==piv {continue};
                self.a[j][k] -= ratio * self.a[row][k];
            }
            self.b[j] -= ratio * self.b[row];

            self.a[j][piv] = -ratio;
        }
        let ratio = -self.c[piv] / self.a[row][piv];
        for k in 0..self.c.len(){
            if k==piv {continue};
            self.c[k] += ratio * self.a[row][k];
        }
        self.c[piv] = ratio;
        self.max -= ratio * self.b[row];
        self.b[row] = self.b[row]/self.a[row][piv];
        self.a[row][piv] = 1.0 / self.a[row][piv];

        let temp = self.base[row];
        self.base[row]  = self.unbase[piv];
        self.unbase[piv] = temp;
        Some(())
    }
    fn ratio_test(&self, i: usize) -> Option<usize>{
        let mut index = 0;
        let mut min =  f64::INFINITY;
        for j in 0..self.base.len(){
            let ratio = self.b[j] / self.a[j][i];
            min = if ratio < min {
                index = j;
                ratio
            } else {min};
        }        
        if min == f64::INFINITY {None} else {Some(index)}
    }
}

#[test]
fn test_pivot(){
    let mut d = Dict::from_canonical(&Canonical::sample());
    println!("{:?}",d );
    let res = d.pivot(0);
    println!("{:?}",res);
    println!("{:?}",d );
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