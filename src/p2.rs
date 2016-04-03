fn fib(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

pub fn run(max: u32) -> u32 {
    let mut i = 0;
    let mut sum = 0;
    let mut result;

    loop {
        result = fib(i);

        if result > max {
            break;
        }

        if result % 2 == 1 {
            sum += result;
        }

        i += 1;
    }

    return sum;
}

#[test]
fn fib_first_is_1() {
    assert_eq!(1, fib(1));
}

#[test]
fn fib_second_is_1() {
    assert_eq!(1, fib(2));
}

#[test]
fn fib_third_is_2() {
    assert_eq!(2, fib(3));
}

#[test]
fn fib_fourth_is_3() {
    assert_eq!(3, fib(4));
}

#[test]
fn fib_fifth_is_5() {
    assert_eq!(5, fib(5));
}

#[test]
fn fib_sixth_is_8() {
    assert_eq!(8, fib(6));
}

#[test]
fn fib_seventh_is_13() {
    assert_eq!(13, fib(7));
}
