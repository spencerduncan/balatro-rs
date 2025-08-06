// Add the necessary imports and code directly here to test
fn main() {
    println\!("Testing f64::NAN comparison:");

    let nan_val = f64::NAN;
    let is_equal = (nan_val - nan_val).abs() < f64::EPSILON;
    let both_nan = nan_val.is_nan() && nan_val.is_nan();
    let result = is_equal || both_nan;

    println\!("is_equal: {}", is_equal);
    println\!("both_nan: {}", both_nan);
    println\!("result: {}", result);

    // Test the actual comparison that's failing
    let diff = nan_val - nan_val;
    println\!("diff: {}", diff);
    println\!("diff.is_nan(): {}", diff.is_nan());
}
EOF < /dev/null
