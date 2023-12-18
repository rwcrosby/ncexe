#![allow(unused)]

fn main() {
}

struct Cl1 {}

impl Cl1 {
    fn format(&self) -> &str {
        "Cl1.format"
    }    
}
struct Cl2 {}

impl Cl2 {
    fn format(&self) -> &str {
        "Cl2.format"
    }    
}

trait T1{
    fn format(&self) -> &str;
}

impl T1 for Cl1 {
    fn format(&self) -> &str {
        "T1.Cl1.format"
    }
}

impl T1 for Cl2 {
    fn format(&self) -> &str {
        "T1.Cl2.format"
    }
}

enum Types {
    Type1(Cl1),
    Type2(Cl1),
    Type3(Cl2)
}

impl Types {
    fn to_string(&self) -> &str {
        match self {
            Types::Type1(v) => v.format(),
            Types::Type2(v) => v.format(),
            Types::Type3(v) => v.format(),
        }
    }

    fn to_t1(&self) -> &dyn T1 {
        match self {
            Types::Type1(v) => v as &dyn T1,
            Types::Type2(v) => v as &dyn T1,
            Types::Type3(v) => v as &dyn T1,
        }

    }

}

#[cfg(test)]
mod tests {

    use super::*;

    fn testit(t: Types) {
        match t {
            Types::Type1(v) => assert!(v.format() == "Cl1.format"),
            Types::Type2(v) => assert!(v.format() == "Cl1.format"),
            Types::Type3(v) => assert!(v.format() == "Cl2.format")
        }
    }

    #[test]
    pub fn cl1() {

        let v1 = Types::Type1(Cl1{});
        testit(v1);
    }
        
    #[test]
    pub fn cl2() {
        
        let v2 = Types::Type2(Cl1{});
        testit(v2);
    }
    
    #[test]
    pub fn cl3() {
        
        let v3 = Types::Type3(Cl2{});
        testit(v3);

    }

    #[test]
    pub fn enum_cl3() {
        
        let v3 = Types::Type3(Cl2{});
        assert!(v3.to_string() == "Cl2.format");

    }

    #[test]
    pub fn t1_cl() {
        
        let v = Types::Type1(Cl1{});
        assert!(v.to_t1().format() == "T1.Cl1.format");

        let v = Types::Type2(Cl1{});
        assert!(v.to_t1().format() == "T1.Cl1.format");

        let v = Types::Type3(Cl2{});
        assert!(v.to_t1().format() == "T1.Cl2.format");

    }


}
