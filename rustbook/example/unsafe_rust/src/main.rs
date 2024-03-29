use core::slice;

// Declaring and calling an extern function defined in another langauge
extern "C" {
    fn abs(input: i32) -> i32; 
}

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    // Create raw pointers
    let mut num = 5;

    let _r1 = &num as *const i32;
    let _r2 = &mut num as *mut i32;

    // creating a raw pointer to an arbitrary memory adress
    let address = 0x012345usize;
    let _r = address as *const i32;

    // dereferencing raw pointers within an unsafe block
    let mut num = 5;
    
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    unsafe {
        dangerous();
    }

    // using the safe split_at_mut function
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a,b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4,5,6]);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(abs(-3)));
    }

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

}

unsafe fn dangerous() {}

fn _split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}


