pub fn rolling_finite_mean(series: Vec<f64>, window: u32) -> Vec<f64> {
    if window == 0 {
        println!("window=0, nothing to moving average. Return the original series");
        return series;
    }

    let series_len = series.len();
    if series_len == 0 {
        println!("series_length=0, nothing to moving average. Return the original series");
        return series;
    }

    let mut series_mva: Vec<f64> = vec![];
    let mut prev_roll_fin_count: i32 = 0;
    let mut prev_roll_fin_sum: f64 = 0.;
    for i in 0..series_len {
        let curr_roll_fin_count: i32;
        let curr_roll_fin_sum: f64;
        let is_curr_val_fin: i32 = series[i].is_finite() as i32;
        let curr_adj_val: f64 = if is_curr_val_fin == 1 { series[i] } else { 0. };

        if i == 0 {
            curr_roll_fin_count = is_curr_val_fin;
            curr_roll_fin_sum = series[i];
        } else if i < window as usize {
            curr_roll_fin_count = prev_roll_fin_count + is_curr_val_fin;
            curr_roll_fin_sum = (if prev_roll_fin_sum.is_finite() {
                prev_roll_fin_sum
            } else {
                0.
            }) + curr_adj_val;
        } else {
            let out_val: f64 = series[i - (window as usize)];
            let is_out_val_fin: i32 = out_val.is_finite() as i32;
            let out_adj_val: f64 = if is_out_val_fin == 1 { out_val } else { 0. };
            curr_roll_fin_count = prev_roll_fin_count + is_curr_val_fin - is_out_val_fin;
            curr_roll_fin_sum = prev_roll_fin_sum + curr_adj_val - out_adj_val;
        }

        // moving average
        series_mva.push(curr_roll_fin_sum / (curr_roll_fin_count as f64));

        // update
        prev_roll_fin_count = curr_roll_fin_count;
        prev_roll_fin_sum = curr_roll_fin_sum
    }
    series_mva
}
