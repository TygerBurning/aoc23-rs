pub fn day06() {
    println!("Part A answer is: {:?}", 0);

    let mut valid = 0;
    for i in 0..53717880 {
        if (53717880 - i)*i as i64 > 275118112151524 {
            valid += 1;
        }
    }
    println!("Part B answer is: {:?}", valid);
}