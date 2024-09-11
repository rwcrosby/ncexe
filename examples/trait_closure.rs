// Experiment with closure containing references

struct s1<'a> {

    str1: &'a String,

}

struct s2<'c> {

    tref: &'c dyn Fn (u32) -> u32,
}

trait t1 {

    fn callit(&self, y: u32) -> u32;

}

impl<'b> t1 for s2<'b> {

    fn callit(&self, y: u32) -> u32 {
        (self.tref)(y)
    }

}

impl<'b> t1 for s1<'b> {

    fn callit(&self, y :u32) -> u32 {
        self.str1.len() as u32
    }

}

fn main() {

    let sv1 = | x:u32 | -> u32 { x * x }; 
    
    let b1 = Box::new(sv1);
    
    let b2 = Box::new(| x:u32 | -> u32 { x * x }); 

    let str1 = "Hello World";

    let b3 = | x: u32 | -> u32 { 

        x + str1.len() as u32

      };

    let sv3 = s2{tref: &b1};
    let sv4 = s2{tref: &b2};
    let sv5 = s2{tref: &b3};

    println!("{}", (sv5.tref)(32));
    
    to_trait(&sv3);
    to_trait(&sv4);
    to_trait(&sv5);

    let s1a = s1{str1: &"blah".to_string()};

    let tv: &dyn t1 = &s1a;
    println!("{}", tv.callit(32));



}

fn to_trait<'a>(s_in: &'a s2<'a>) -> &'a dyn t1 {

    let tv: &dyn t1 = s_in;
    println!("{}", tv.callit(32));

    tv

}
