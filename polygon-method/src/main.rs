use std::io::Write;

fn main() {
    print!("Number of polygon sides: ");
    std::io::stdout().flush().unwrap();

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let polygon_sides_amount = line.trim().parse::<f64>().unwrap();

    if polygon_sides_amount < 3. {
        println!("The minimum sides amount is 3!");
        std::process::exit(1);
    }

    let smaller_polygon_area =
        0.5 * polygon_sides_amount * (360. / polygon_sides_amount).to_radians().sin();
    let larger_polygon_area =
        polygon_sides_amount * (360. / (polygon_sides_amount * 2.)).to_radians().tan();

    println!("{} < π < {}", smaller_polygon_area, larger_polygon_area);
    println!();
    println!(
        "Calculated PI: {:.50}",
        (smaller_polygon_area + larger_polygon_area) / 2.
    );
    println!("Actual PI    : {:.50}", std::f64::consts::PI);
    println!(
        "Difference   : {:.50}",
        (std::f64::consts::PI - (smaller_polygon_area + larger_polygon_area) / 2.).abs()
    );
}
