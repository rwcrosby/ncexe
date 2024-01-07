fn main() {
    
    let b1 = Base{a: 11};
    
    Base::fn1(&b1);
    
    t1(Base::fn2);
    
    let b3 = Base{a: 33};
    t2(&b3, Base::fn2);
    
    let b4 = Base{a: 44};
    t3(&b4, Base::fn1);

    let b4 = Base{a: 44};
    t3(&b4, Base::fn1);

    let b5 = Base2{b: 55};
    t3(&b5, Base2::fn3);

    println!("Test Traits");
    test_trait();

}

struct Base {
    a: usize,
}

impl Base {

    fn fn1(&self) {
        println!("Fn1 {}", self.a * 10);
    }

    fn fn2(&self) {
        println!("Fn2 {}", self.a * 100);
    }

}

struct Base2 {
    b: usize,
}

impl Base2 {

    fn fn3(&self) {
        println!("Fn3 {}", self.b * 3);
    }

    fn _fn4(&self) {
        println!("Fn4 {}", self.b * 33);
    }

}

fn t1(fnp: fn(b: &Base)) {
    let b2 = Base{a: 22};
    fnp(&b2);
}

fn t2(base: &Base, fnp: fn(b: &Base)) {
    fnp(base);
}

fn t3<T>(base: &T, fnp: fn(b: &T)) {
    fnp(base);
}

// -----------------------------------------------------

trait Trait1 {
    fn on_enter(&self);
}

struct Base3<'a > {
    b: &'a Base,
}

struct Base4<'a> {
    b: &'a Base2,
}

impl<'a> Trait1 for Base3<'a> {
    fn on_enter(&self) {
        self.b.fn1();
    }
}

impl<'a> Trait1 for Base4<'a> {
    fn on_enter(&self) {
        self.b.fn3()
    }
}

fn test_trait() {

    let b1 = Base{a: 100};
    let b2 = Base2{b: 100};

    let b3 = Base3{b: &b1};

    let t1: &dyn Trait1 = &b3;
    t1.on_enter();
    
    let t2 = &Base4{b: &b2} as &dyn Trait1;
    t2.on_enter();
}