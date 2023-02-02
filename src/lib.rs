pub fn simple_moving_average(arr: Vec<f64>, w: u64) -> Vec<f64> {
    if w == 0 {
        println!("window=0, nothing to moving average. Return the original series");
        return arr
    }
    let n = arr.len() as u64;
    let mut mva_arr: Vec<f64> = vec![];
    let mut sum_win = 0.;
    let mut v = 0.;
    let mut w_dyn = 0;  // dynamic window size
    for i in 0..n {
        sum_win = 0.;
        w_dyn = 0;
        for j in i-w+1..i+1 {  // j = i, i-1, ..., i-w+1
            v = arr[j as usize];

            if f64::is_finite(v) { // ignore non-finite values (f64::INFINITY, f64::NEG_INFINITY, f64::NAN)
                sum_win += v;
                w_dyn += 1;
            }
        }
        mva_arr.push(sum_win / (w_dyn as f64));
    }
    mva_arr
}
