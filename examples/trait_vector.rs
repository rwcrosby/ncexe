

struct MyStruct<'s> {
    s: String,
    ss: &'s str,
}
    
impl<'ms> MyStruct<'ms> {
    
    fn new(s: &'ms str) -> Box<dyn MyTrait + 'ms> {
        Box::new(Self{s: s.to_string() + " as string",
            ss: s
        })
    }
    
}

trait MyTrait: {
    fn some_fn(&self) {
        println!("In SomeFn")
    }
}

impl MyTrait for MyStruct<'_> {
    fn some_fn(&self) {
        println!("In SomeFn for MyStuct s:{}, ss:{}", self.s, self.ss)
    }
}

fn receiver<'t>(my_trait: &'t dyn MyTrait) {
    my_trait.some_fn();
}

// type MyTraitRef<'t> = dyn MyTrait;

fn main() {
    
    let ms = Box::new(MyStruct{s: "ms".to_string(), ss: "sliceme"});
    let ms2 = MyStruct::new("Blah");
    
    let trait_ref = ms2.as_ref();
    receiver(trait_ref);

    ms.some_fn();
    ms2.some_fn();

    let msv: Vec<Box<dyn MyTrait>> = vec![ms, ms2];
 
    receiver(msv[0].as_ref());
    
    println!("Do something else");
}