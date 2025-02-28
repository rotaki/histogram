use hdrhistogram::Histogram;
use std::time::{Duration, Instant};

// This is the function we want to measure.
// Replace the body with your actual computation.
fn function_to_measure() {
    // Example work: a trivial calculation
    let _ = (0..10).sum::<u64>();
}

fn main() {
    // Create a histogram that records values in the range [1, 1_000_000_000] nanoseconds,
    // with 3 significant figures of precision.
    let mut histogram =
        Histogram::<u64>::new_with_bounds(1, 1_000_000_000, 3).expect("Failed to create histogram");

    let iterations = 1_000_000;

    for _ in 0..iterations {
        let start = Instant::now();
        function_to_measure();
        let elapsed = start.elapsed();

        // Convert elapsed time to nanoseconds
        let nanos = elapsed.as_nanos() as u64;
        histogram
            .record(nanos)
            .expect("Failed to record measurement");
    }

    // Print out key statistics from the histogram.
    // println!("Histogram statistics for {} iterations:", iterations);
    // println!("Mean: {:.2} ns", histogram.mean());
    // println!(
    //     "Median (50th percentile): {} ns",
    //     histogram.value_at_quantile(0.5)
    // );
    // println!("90th percentile: {} ns", histogram.value_at_quantile(0.9));
    // println!("99th percentile: {} ns", histogram.value_at_quantile(0.99));

    // for v in histogram.iter_recorded() {
    //     println!(
    //         "{}'th percentile of data is {} with {} samples",
    //         v.percentile(),
    //         v.value_iterated_to(),
    //         v.count_at_value()
    //     );
    // }

    plot_histogram(&histogram, 100, Some(0), Some(1000)); // 1000 ns = 1 us
}

pub fn print_histogram_data(histogram: &Histogram<u64>) {
    // Print header for the formatted table.
    println!(
        "{:>10} {:>15} {:>10} {:>16}",
        "Value", "Percentile", "TotalCount", "1/(1-Percentile)"
    );

    let mut cumulative = 0;
    // Iterate over each recorded bucket.
    for v in histogram.iter_recorded() {
        cumulative += v.count_at_value();
        // The iterator’s percentile is given as a percentage (0.0 to 100.0);
        // we divide by 100 to get a fraction.
        let pct = v.quantile_iterated_to();
        if pct < 1.0 {
            println!(
                "{:10.3} {:15.12} {:10} {:16.2}",
                v.value_iterated_to(),
                pct,
                cumulative,
                1.0 / (1.0 - pct)
            );
        } else {
            // For the final bucket (where percentile == 1), we omit the 1/(1-Percentile) column.
            println!(
                "{:10.3} {:15.12} {:10}",
                v.value_iterated_to(),
                pct,
                cumulative
            );
        }
    }

    // Print a summary similar to your sample.
    println!(
        "#[Mean    = {:10.3}, StdDeviation   = {:10.3}]",
        histogram.mean(),
        histogram.stdev()
    );
    println!(
        "#[Max     = {:10.3}, Total count    = {:10}]",
        histogram.max(),
        histogram.len()
    );
    println!("#[Buckets = {:10}]", histogram.buckets());
    // println!(
    //     "#[Buckets = {:10}, SubBuckets     = {:10}]",
    //     histogram.buckets(),
    //     histogram.sub_bucket_count()
    // );
}

pub fn plot_histogram(
    histogram: &Histogram<u64>,
    step: u64,
    min_threshold: Option<u64>,
    max_threshold: Option<u64>,
) {
    let max_bar_width = 50;

    // Determine the maximum count in any bucket to scale the bars.
    let max_count = histogram.len();

    // Print header.
    println!("{:>10} | {:<50} | {}", "Value", "Histogram", "Count(%)");
    println!("{}", "-".repeat(10 + 3 + 50 + 3 + 5 + 1 + 10)); // Extra last 10 for the percentage

    // Print smaller than min_threshold
    if let Some(min) = min_threshold {
        let percentage = histogram.percentile_below(min);
        let bar_len = ((percentage / 100.0) * max_bar_width as f64).round() as usize;
        let bar = "*".repeat(bar_len);
        println!(
            "< {:>8} | {:<50} | {:10}({:.2})",
            min,
            bar,
            (percentage / 100.0 * max_count as f64) as usize,
            percentage / 100.0
        );
    }

    // Iterate through the histogram in linear steps.
    for iv in histogram.iter_linear(step) {
        // Skip buckets that are below the minimum threshold.
        if let Some(min) = min_threshold {
            if iv.value_iterated_to() < min {
                continue;
            }
        }
        // Stop when we reach the maximum threshold.
        if let Some(max) = max_threshold {
            if iv.value_iterated_to() > max {
                break;
            }
        }
        let count = iv.count_since_last_iteration();
        // Scale the bar length relative to the maximum count.
        let bar_len = ((count as f64 / max_count as f64) * max_bar_width as f64).round() as usize;
        let bar = "*".repeat(bar_len);
        println!(
            "{:>10} | {:<50} | {:10}({:.2})",
            iv.value_iterated_to(),
            bar,
            count,
            count as f64 / max_count as f64
        );
    }

    // Print larger than max_threshold
    if let Some(max) = max_threshold {
        let percentage = 100.0 - histogram.percentile_below(max);
        let bar_len = ((percentage / 100.0) * max_bar_width as f64).round() as usize;
        let bar = "*".repeat(bar_len);
        println!(
            "> {:>8} | {:<50} | {:10}({:.2})",
            max,
            bar,
            (percentage / 100.0 * max_count as f64) as usize,
            percentage / 100.0
        );
    }
}
