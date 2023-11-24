#![allow(unused)]

#[test]
pub fn align() {
    
    let bytes: [u8; 7] = [1, 2, 3, 4, 5, 6, 7];
    
    let (prefix, shorts, suffix) = 
    unsafe {
        bytes.align_to::<u16>()
    };
    println!("{:?} - {:?} - {:?}", prefix, shorts, suffix);
    
    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]   
    struct MyStruct {
        foo: u16,
        bar: u8,
    }    
    
    let (head, body, tail) = unsafe { bytes.align_to::<MyStruct>() };
    assert!(head.is_empty(), "Data was not aligned");
    println!("{:?} - {:?} - {:?}", head, body, tail);
    let my_struct = &body[0];
    
    println!("{:?}", my_struct);
    
}