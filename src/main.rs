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
