fn main() {
    println!("Start LP");

    let c = Canonical::sample();
    println!("{:?}", c);
    let mut d = Dict::from_canonical(&c);
    d.solve();
    println!("{:?}", d);
}

#[derive(Debug)]
struct Dict {
    unbase: Vec<i64>,
    base: Vec<i64>,
    a: Vec<Vec<f64>>,
    buf: Vec<f64>,
    x: usize,
    y: usize,
}

fn argmax(v: &[f64]) -> Option<usize> {
    assert!(v.len() > 0);
    let mut max = v[0];
    let mut index = 0;
    for (i, x) in v.into_iter().enumerate() {
        max = if max > *x {
            max
        } else {
            index = i;
            *x
        };
    }
    if max > 0.0 {
        Some(index)
    } else {
        None
    }
}

#[test]
fn test_argmax() {
    let v: Vec<f64> = vec![2.0, 3.0, 1.0];
    let i = argmax(&v).unwrap();
    assert_eq!(i, 1);
    let v: Vec<f64> = vec![-2.0, -3.0, -1.0];
    let i = argmax(&v);
    assert_eq!(i, None);
}

use std::f64;

impl Dict {
    fn from_canonical(can: &Canonical) -> Dict {
        let base_num = can.b.len() as i64;
        let unbase_num = can.c.len() as i64;
        let mut d = Dict {
            unbase: (0..unbase_num).collect(),
            base: (unbase_num..(base_num + unbase_num)).collect(),
            a: can
                .a
                .clone()
                .into_iter()
                .zip(can.b.iter())
                .map(|(vec, w)| {
                    let mut nv: Vec<f64> = vec.into_iter().map(|x| -x).collect();
                    nv.push(*w);
                    nv
                }).collect(),
            x: base_num as usize,
            y: unbase_num as usize,
            buf: vec![0.0; unbase_num as usize + 1],
        };
        let mut c = can.c.clone();
        c.push(0.0);
        d.a.push(c);
        d
    }
    pub fn solve(&mut self) -> Option<()> {
        loop {
            let i = match self.choose_column() {
                None => break,
                Some(i) => i,
            };
            self.pivot(i)?;
        }
        Some(())
    }
    fn choose_column(&self) -> Option<usize> {
        argmax(&self.a[self.x][0..self.y])
    }
    fn pivot(&mut self, piv: usize) -> Option<()> {
        let row = self.ratio_test(piv)?;
        for i in 0..self.y + 1 {
            self.buf[i] = 0.0;
        }
        self.buf[piv] = 1.0;
        std::mem::swap(&mut self.buf, &mut self.a[row]);
        let ratio_seed = 1.0 / self.buf[piv];
        self.buf[piv] = -1.0;
        for i in 0..self.x + 1 {
            let ratio = -self.a[i][piv] * ratio_seed;
            self.a[i][piv] = 0.0;
            for j in 0..self.y + 1 {
                self.a[i][j] += ratio * self.buf[j];
            }
        }
        std::mem::swap(&mut self.base[row], &mut self.unbase[piv]);
        Some(())
    }
    fn ratio_test(&self, j: usize) -> Option<usize> {
        let mut index = 0;
        let mut min = f64::INFINITY;
        for i in 0..self.x {
            let ratio = -self.a[i][self.y] / self.a[i][j];
            if ratio < 0.0 {
                continue;
            };
            min = if ratio < min {
                index = i;
                ratio
            } else {
                min
            };
        }
        if min == f64::INFINITY {
            None
        } else {
            Some(index)
        }
    }
}

#[test]
fn test_pivot() {
    let mut d = Dict::from_canonical(&Canonical::sample());
    println!("{:?}", d);
    let res = d.pivot(0);
    println!("{:?}", res);
    println!("{:?}", d);
}

#[derive(Debug)]
struct Canonical {
    c: Vec<f64>,
    a: Vec<Vec<f64>>,
    b: Vec<f64>,
}

impl Canonical {
    fn sample() -> Canonical {
        let c = vec![3.0, 2.0];
        let b = vec![4.0, 5.0, 7.0];
        let mut a = Vec::new();
        a.push(vec![1.0, 1.0]);
        a.push(vec![2.0, 0.0]);
        a.push(vec![2.0, 1.0]);
        Canonical { a: a, b: b, c: c }
    }
}
