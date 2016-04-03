pub fn run(until: u32) -> u32 {
    let mut sum = 0;

    for i in 1..until {
        if i % 3 == 0 || i % 5 == 0 {
            sum = sum + i;
        }
    }

    return sum;
}

#[test]
fn sum_until_10_is_23() {
    assert_eq!(23, run(10));
}
