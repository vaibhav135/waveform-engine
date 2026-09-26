use std::f64::consts::PI;

/**
*
*  first calculate the sampling time interval
*
*  delta t = 1 / sampling frequency
*
*  say we are sampling at 10000 smaples/s
*
*
* Another thing is N = D * sampling frequency
* for this case let's take D = 10
*/

fn main() {
    let duration = 10;
    let sampling_freq = 10_000;
    let total_sample = sampling_freq * duration;

    // delta_t = sample interval
    let delta_t = 1.0 / (sampling_freq as f64);

    // We assume amplitude as 2. Meaning max +2 and min = -2
    let ampl = 2;

    // We assume wave frequency as 50hz. This is the wave frequency.
    let freq = 50;

    // Idea is to get a time interval for each sample index.
    for sample_idx in 0..total_sample {
        let cur_time = (sample_idx as f64) * delta_t;
        let discreet_sample = (ampl as f64) * f64::sin(2.0 * PI * (freq as f64) * cur_time);

        println!("{discreet_sample}");
    }
}
