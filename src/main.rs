fn main() {
    let solver = boolector::Builder::new()
        .generate_model(boolector::GenerateModel::Asserted)
        .finish();

    let b8 = solver.sort(boolector::Sort::BitVec(8));

    let x = boolector::Unsigned(b8.var(Some("x")));
    let y = boolector::Unsigned(b8.var(Some("y")));

    let sum = &x + &y;

    let sum_lt_x = sum.lt(&x);
    let sum_lt_y = sum.lt(&y);

    let both = &sum_lt_x & &sum_lt_y;

    solver.assert(&both);

    if let boolector::SolveResult::Sat(Some(model)) = solver.solve() {
        println!("yay, sat!");
        let x_val = model.bit_vec(&x);
        let y_val = model.bit_vec(&y);
        let sum_val = model.bit_vec(&sum);
        println!("x = {}", x_val.to_str());
        println!("y = {}", y_val.to_str());
        println!("x + y = {}", sum_val.to_str());
    }
}
