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
    println!("Histogram statistics for {} iterations:", iterations);
    println!("Mean: {:.2} ns", histogram.mean());
    println!(
        "Median (50th percentile): {} ns",
        histogram.value_at_quantile(0.5)
    );
    println!("90th percentile: {} ns", histogram.value_at_quantile(0.9));
    println!("99th percentile: {} ns", histogram.value_at_quantile(0.99));

    for v in histogram.iter_recorded() {
        println!(
            "{}'th percentile of data is {} with {} samples",
            v.percentile(),
            v.value_iterated_to(),
            v.count_at_value()
        );
    }
}
