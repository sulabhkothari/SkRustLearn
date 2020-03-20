fn diverges() -> ! {
    panic!("This function never returns!");
}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
fn nToStr(i:i32) -> i32 {
    i + 1
}


fn main(){
    let f:fn(i32) -> i32;
    f = nToStr;
    println!("{}", f(90));
}