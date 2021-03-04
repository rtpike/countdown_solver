use itertools::Itertools;
use std::fmt;

#[derive(PartialEq, Clone, Debug)]
enum Ops {
    Add,
    Sub,
    Mult,
    Div,
    Num(i32),
}

enum RpnErr {
    DivZero,
    DivRem,
    Stack
}

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

/*
#[derive(Clone)]
struct Sdata {
    opt: Ops,
    data: i32,
}

impl fmt::Display for Sdata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.opt == Ops::Num {
            write!(f, "{}", self.data)
        } else {
            match self.opt {
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
                _ => {
                    write!(f, "NA")
                }
            }
        }
    }
}


impl fmt::Debug for Sdata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.opt == Ops::Num {
            f.debug_struct("Sdata")
                .field("data", &self.data)
                .finish()
        } else {
            f.debug_struct("Sdata")
                .field("opt", &self.opt)
                .finish()
        }
    }
}
*/


/// Reverse Polish Notation from vector
fn rpn_vec(rvec: &Vec<Ops>) -> i32 {
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
                    println!("ERROR: div by zero {}/{}", a, b);
                    return -9900;
                }  else if a % b != 0 {
                    if debug {println!("ERROR: div remainder {}/{} = {} rem {}", a, b, a / b , a % b );}
                    return -9901;
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
        stack.pop().unwrap()
    } else {
        println!(" ERROR: extra values in stack {:?}", stack);
        return -9999
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
        //let num_ops = ops.iter().permutations(i/2+1);
        for v in num_perms {
            if debug { println!("v:{:?}", v); }
            if v.len() == 1 {
                if debug { println!("{}", *v[0]); }
            } else {
                let num_ops = ops.iter().permutations(v.len()-1);
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
                    if num == ans || debug {
                        for token in rvect.iter() {
                            print!("{} ", token.to_string());
                        }
                        println!("= {}", num)
                    }
                }
            }

            /*
            println!("v {:?}", v);
            if v.len() == 1 {
                //return *v[0];
            } else if v.len() == 2 {
                for opt in ops.iter() {
                    let mut rvect: Vec<Ops> = Vec::new();
                    rvect.push(Ops::Num(*v[0]));
                    rvect.push(Ops::Num(*v[1]));
                    rvect.push(opt.clone());
                    //print!("    rpn_vec{:?}", rvect, );
                    let num = rpn_vec(rvect);
                    println!("= {:?}", num);
                }
            } else {
                let mut rvect: Vec<Ops> = Vec::new();
                num_ops
                for opt in ops.iter() {
                     rvect.push(Ops::Num(*v[0]));
                     rvect.push(Ops::Num(*v[1]));
                     rvect.push(opt.clone());
                     //print!("    rpn_vec{:?}", rvect, );
                     //let num = rpn_vec(rvect);
                     //println!("= {:?}", num);
                }
                let mut n = 2;
                while n < v.len() {
                    //for opt in ops.iter() {
                        rvect.push(Ops::Num(*v[n]));
                        n += 1;
                        rvect.push(Ops::Add);
                        //rvect.push(opt.clone());
                        //print!("    rpn_vec{:?}", rvect, );
                    //} // TODO: groupings
                }
                let num = rpn_vec(rvect);
                println!("= {:?}", num);
            }

             */
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
    */
    gen_rpn(&[2,3,4,20,100], 52);

}


