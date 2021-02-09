pub fn resize (width: usize, height: usize) {
    println!("\u{001B}[8;{};{}t",height, width);
}