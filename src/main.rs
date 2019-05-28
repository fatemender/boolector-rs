fn main() {
    let solver = boolector::Builder::new()
        .generate_model(boolector::GenerateModel::Asserted)
        .finish();

    let b8 = solver.sort(boolector::Sort::BitVec(8));

    let x = boolector::Unsigned(b8.var(Some("x")));
    let y = boolector::Unsigned(b8.var(Some("y")));

    let sum = x.clone() + y.clone();

    let sum_lt_x = sum.clone().lt(x.clone());
    let sum_lt_y = sum.clone().lt(y.clone());

    let both = sum_lt_x.clone() & sum_lt_y.clone();

    solver.assert(both);
}
