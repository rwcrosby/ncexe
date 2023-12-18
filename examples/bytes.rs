#![allow(unused)]

// https://stackoverflow.com/questions/59289331/how-do-i-map-data-from-a-vector-of-bytes-to-a-structure-in-rust

#[test]
// align_to doesn't move the data, just sets up another slice
pub fn align() {
    
    let bytes: [u8; 7] = [1, 2, 3, 4, 5, 6, 7];
    
    let (prefix, shorts, suffix) = 
    unsafe {
        bytes.align_to::<u16>()
    };
    println!("Align: {:?} - {:?} - {:?}", prefix, shorts, suffix);
    
    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]   
    struct MyStruct {
        foo: u16,
        bar: u8,
    }    
    
    let (head, body, tail) = unsafe { bytes.align_to::<MyStruct>() };
    assert!(head.is_empty(), "Data was not aligned");
    println!("Align: {:?} - {:?} - {:?}", head, body, tail);
    let my_struct = &body[0];
    
    println!("Align: {:?}", my_struct);
    
}

#[test]
// transmute creats a copy of the dataq
pub fn transmute() {
    
    let bytes: [u8; 7] = [1, 2, 3, 4, 5, 6, 7];
    
    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]   
    struct MyStruct {
        foo: u16,
        bar: u8,
        baz: [u8; 4],
    }    
    
    let my_struct = unsafe {
        std::mem::transmute::<[u8; 7], MyStruct> (bytes)
    };

    println!("Transmute: {:?}", my_struct);
    
}

#[test]
// raw_pointer seem to be the way to go, no data movement, just reference to the memory
// read_unaligned moves the data
pub fn raw_pointer() {
    
    let bytes: [u8; 7] = [1, 2, 3, 4, 5, 6, 7];
    
    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]   
    struct MyStruct {
        foo: u16,
        bar: u8,
        baz: [u8; 4],
    }    
    
    let my_struct = unsafe {
        (bytes.as_ptr() as *const MyStruct).read_unaligned() 
    };

    println!("Read_unaligned: {:?}", my_struct);
    
    let my_struct_ptr = bytes.as_ptr() as *const MyStruct;
    
    let my_struct_ptr2 = unsafe{ *my_struct_ptr };
    let my_struct_ptr3 = unsafe{ *(bytes.as_ptr() as *const MyStruct) };

    println!("Read_unaligned: {:?}", my_struct_ptr);
    println!("Read_unaligned: {:?}", my_struct_ptr2);
    println!("Read_unaligned: {:?}", my_struct_ptr3);

}