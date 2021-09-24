fn main() {
    let iterates_amount = 1000000;

    let mut pi: f64 = 3.;
    let mut i: f64 = 2.;
    for iteration in 0..iterates_amount {
        if iteration % 2 == 0 {
            pi += 4. / (i * (i + 1.) * (i + 2.));
        } else {
            pi -= 4. / (i * (i + 1.) * (i + 2.));
        }

        i += 2.;
    }

    println!("Generated PI : {:.50}", pi);
    println!("Actual PI    : {:.50}", std::f64::consts::PI);
    println!("Difference PI: {:.50}", (std::f64::consts::PI - pi).abs());
}
