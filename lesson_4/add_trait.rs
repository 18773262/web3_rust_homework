#![allow(dead_code)]

/*
搜索相关文档，为你自己定义的一个类型或多个类型实现加法运算（用符号 +），并构思使用Trait Object实现类型方法的调用。
提示：

实现 Add trait

实现一个函数，接受Trait Object作为参数
 */
use std::ops::Add;

#[derive(Debug)]
struct Complex {
    a: f64,
    b: f64,
}

#[derive(Debug)]
struct Real {
    x: f64,
}

impl Add for Real {
    type Output = Real;
    fn add(self, other: Real) -> Real {
        Real {x: self.x+other.x}
    }
}

impl Add for Complex {
    type Output = Complex;
    fn add(self, other: Complex) -> Complex {
        Complex {a: self.a+other.a, b: self.b+other.b}
    }
}

fn main() {
    let cp1 = Complex{a: 1f64, b: 2.0};
    let cp2 = Complex{a: 5.0, b:8.1};
    let cp3 = cp1 + cp2;
    print!("{:?}", cp3);
}