
fn fibonacci(n: i32)-> i32{
    if n == 0{
        return 0
    } else if n == 1 {
        return 1
    }

    let mut a = 0;
    let mut b = 1;
    let mut c;

    for _i in 2..=n{
        c = a + b;
        a = b;
        b = c;
    }
    b
}

fn main(){
    let n = 10;
    println!("The nth number of {} is {}", n, fibonacci(n))
}