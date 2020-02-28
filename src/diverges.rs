fn diverges() -> ! {
    panic!("This function never returns!");
}

fn nToStr(i:i32) -> i32 {
    i + 1
}

fn main(){
    let f:fn(i32) -> i32;
    f = nToStr;
    println!("{}", f(90));
}