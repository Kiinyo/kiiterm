struct Test {
    x: usize,
    y: usize,
}
fn change_x (test: &mut Test) {
    test.x += 5;
}
fn main() {
    let mut test_struct = Test {
        x: 25,
        y: 32
    };

    println!("Test x: {}", test_struct.x);

    change_x(&mut test_struct);

    println!("Test x after change: {}", test_struct.x);

}