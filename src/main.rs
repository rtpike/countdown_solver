use itertools::Itertools;
use std::fmt;
use std::error;
#[macro_use] extern crate itertools;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Ops {
    Add,
    Sub,
    Mult,
    Div,
    Num(i32),
}

#[derive(PartialEq, Debug)]
enum RpnErr {
    DivZero,
    DivRem,
    Stack,
}

impl fmt::Display for RpnErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            RpnErr::DivZero => "Divide by zero",
            RpnErr::DivRem => "Divide remainder",
            RpnErr::Stack => "Stack size",
        };
        f.write_str(description)
    }
}

impl error::Error for RpnErr {}

impl fmt::Display for Ops {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Ops::Add => {
                write!(f, "+")
            }
            Ops::Sub => {
                write!(f, "-")
            }
            Ops::Mult => {
                write!(f, "*")
            }
            Ops::Div => {
                write!(f, "/")
            }
            Ops::Num(n) => {
                write!(f, "{}",*n)
            }
        }
    }
}

fn product_ops(vector: &[Ops], n: usize) -> Vec<Vec<Ops>> {
    let mut result: Vec<Vec<Ops>> = vec![vec![]];

    for _ in 0..n {
        result = iproduct!(result.iter(), vector.iter())
            .map(|(v, x)| {
                let mut v1 = v.clone();
                v1.push(*x);
                v1
            })
            .collect();
    }
    result
}


fn product(vector: &[i32], n: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![vec![]];

    for _ in 0..n {
        result = iproduct!(result.iter(), vector.iter())
            .map(|(v, x)| {
                let mut v1 = v.clone();
                v1.push(*x);
                v1
            })
            .collect();
    }
    result
}


/// Reverse Polish Notation from vector
fn rpn_vec(rvec: &Vec<Ops>) -> Result<i32,RpnErr> {
    let mut stack: Vec<i32> = vec![];
    let debug = false;

    for token in rvec.iter() {
        if debug { print!("{} ", token.to_string()); }
        if debug { println!("{:?} ", token); }
        match token {
            Ops::Add => {
                if debug { print!("+ "); }
                let b = stack.pop().expect("missing first operand");
                let a = stack.pop().expect("missing second operand");
                stack.push(a + b);
            }
            Ops::Sub => {
                if debug { print!("- "); }
                let b = stack.pop().expect("missing first operand");
                let a = stack.pop().expect("missing second operand");
                stack.push(a - b);
            }
            Ops::Mult => {
                if debug { print!("* "); }
                let b = stack.pop().expect("missing first operand");
                let a = stack.pop().expect("missing second operand");
                stack.push(a * b);
            }
            Ops::Div => {
                if debug { print!("/ "); }
                let b = stack.pop().expect("missing first operand");
                let a = stack.pop().expect("missing second operand");
                if b == 0 {
                    if debug {println!("ERROR: div by zero {}/{}", a, b);}
                    //return -9900;
                    return Err(RpnErr::DivZero)
                }  else if a % b != 0 {
                    if debug {println!("ERROR: div remainder {}/{} = {} rem {}", a, b, a / b , a % b );}
                    //return -9901;
                    return Err(RpnErr::DivRem)
                } else {
                    stack.push(a / b);
                }
            }
            Ops::Num(n) => {
                if debug { print!("push {:?} ", n); }
                stack.push(*n);
            }
            //_ => panic!("unknown operator {}", token),
        }
        if debug { println!("calculate {:?}", stack); }
    }
    if stack.len() == 1 {
        Ok(stack.pop().unwrap())
    } else {
        println!(" ERROR: extra values in stack {:?}", stack);
        //return -9999
        Err(RpnErr::Stack)
    }
}


/// Reverse Polish Notation from string
#[warn(dead_code)]
fn rpn(text: &str) -> f64 {
    let tokens = text.split_whitespace();
    let mut stack: Vec<f64> = vec![];
    println!("input operation stack");

    for token in tokens {
        print!("{:^5} ", token);
        match token.parse() {
            Ok(num) => {
                stack.push(num);
                println!("push      {:?}", stack);
            }
            Err(_) => {
                match token {
                    "+" => {
                        let b = stack.pop().expect("missing first operand");
                        let a = stack.pop().expect("missing second operand");
                        stack.push(a + b);
                    }
                    "-" => {
                        let b = stack.pop().expect("missing first operand");
                        let a = stack.pop().expect("missing second operand");
                        stack.push(a - b);
                    }
                    "*" => {
                        let b = stack.pop().expect("missing first operand");
                        let a = stack.pop().expect("missing second operand");
                        stack.push(a * b);
                    }
                    "/" => {
                        let b = stack.pop().expect("missing first operand");
                        let a = stack.pop().expect("missing second operand");
                        stack.push(a / b);
                    }
                    "^" => {
                        let b = stack.pop().expect("missing first operand");
                        let a = stack.pop().expect("missing second operand");
                        stack.push(a.powf(b));
                    }
                    _ => panic!("unknown operator {}", token),
                }
                println!("calculate {:?}", stack);
            }
        }
    }
    stack.pop().unwrap_or(0.0)
}

/// generator that produces a RPN vector using the given numbers
fn gen_rpn(nums:&[i32], ans: i32) -> i32 { //Vec<Sdata> {
    println!("nums {:?} ans: {}", nums, ans);
    let add = Ops::Add; // commutative
    let sub = Ops::Sub;
    let mult = Ops::Mult; // commutative
    let div = Ops::Div;
    let ops = [add, sub, mult, div];
    let debug = false;

    // TOD: use a tree instead of vect
    for i in 0..(nums.len()) {
        let num_perms = nums.into_iter().permutations(i + 1);
        //let num_ops = ops.iter().permutations(v.len()-1);
        for v in num_perms {
            if debug { println!("v:{:?}", v); }
            if v.len() == 1 {
                if debug { println!("{}", *v[0]); }
            } else {
                let num_ops = product_ops(&ops,v.len()-1);
                for optv in num_ops {
                    if debug {println!("v:{:?} optv:{:?}", v, optv);}
                    let mut rvect: Vec<Ops> = Vec::new();
                    let mut y = 0;
                    for x in 0..optv.len() {
                        if y < v.len() {
                            rvect.push(Ops::Num(*v[y]));
                            y += 1;
                        }
                        if y < v.len() {
                            rvect.push(Ops::Num(*v[y]));
                            y +=1;
                        }
                        rvect.push(optv[x].clone());
                    }
                    let num = rpn_vec(&rvect);
                    //println!("= {:?}", num);
                    if num == Ok(ans) || debug {
                        for token in rvect.iter() {
                            print!("{} ", token.to_string());
                        }
                        println!("= {}", num.unwrap())
                    }
                }
            }
        }
    }
    return 9999; // ERROR
}


fn main() {
    /*
    println!("Hello, world!");
    rpn("2 3 3 + *");
    rpn("2 3 3 * +");
    rpn("2 3 3 + /");

    let add = Ops::Add;
    let num = Ops::Num(11);

    let sub = Sdata{opt: Ops::Sub, data:200};
    let mult = Sdata{opt: Ops::Mult, data:300};
    let div = Sdata{opt: Ops::Div, data:400};

    let mut rvect: Vec<Ops> = Vec::new();
    rvect.push(Ops::Num(2));
    rvect.push(Ops::Num(3));
    rvect.push(Ops::Add);
    rvect.push(Ops::Num(3));
    rvect.push(Ops::Num(1));
    rvect.push(Ops::Sub);
    rvect.push(Ops::Div);
    println!("= {:?}", rpn_vec(&rvect));


    let list= [1,2,3,4];
    println!("Permutations:");
    let mut n = 0;
    for i in list.iter().permutations(2) {
        println!("{}: {:?}", n,i);
        n += 1;
    }
    n = 0;
    println!("Product:");
    for i in product(&list, 3) {
        println!("{}: {:?}", n,i);
        n += 1;
    }
    */

    gen_rpn(&[3,6,25,50,75,100], 352);

    // https://www.mirror.co.uk/news/weird-news/countdown-reveals-ultimate-maths-wizard-12227740
    gen_rpn(&[3,6,25,50,75,100], 952);

}