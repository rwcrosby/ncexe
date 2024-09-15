// Experiment with closure containing references

trait T1 {
    fn callit(&self, y: usize) -> usize;
}

struct S1<'a> {
    str1: &'a String,
}    

impl<'b> T1 for S1<'b> {
    fn callit(&self, y :usize) -> usize {
        y + self.str1.len()
    }
}

struct S2<'c> {
    tref: &'c dyn Fn (usize) -> usize,
}        

impl<'b> T1 for S2<'b> {
    fn callit(&self, y: usize) -> usize {
        (self.tref)(y)
    }    
}    

fn main() {
    
    let s1a = S1{str1: &"blah".to_string()};
    let tv: &dyn T1 = &s1a;
    println!("s1a: {}", tv.callit(32));
    
    let sv1 = | x:usize | -> usize { x * x }; 
    let b1 = Box::new(sv1);
    let sv3 = S2{tref: &b1};
    
    let b2 = Box::new(| x:usize | -> usize { x * x }); 
    let sv4 = S2{tref: &b2};
    
    let str1 = "Hello World";
    let b3 = | x: usize | -> usize { 
        x + str1.len() as usize
      };
    let sv5 = S2{tref: &b3};
    
    println!("sv5: {}", (sv5.tref)(32));
    
    let tv = to_trait(&sv3);
    println!("sv3: {}", tv.callit(10));
    
    let tv = to_trait(&sv4);
    println!("sv4: {}", tv.callit(20));
    
    let tv = to_trait(&sv5);
    println!("sv5: {}", tv.callit(30));

}

fn to_trait<'a>(s_in: &'a S2<'a>) -> &'a dyn T1 {
    s_in
}
