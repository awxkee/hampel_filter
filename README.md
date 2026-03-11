# Hampel Filter

[Hampel filter](https://www.mathworks.com/help/dsp/ref/hampelfilter.html) for outlier detection in streaming float data.

Replaces spikes and outliers in a signal with the local window median, based on the MAD (Median Absolute Deviation) estimator.

## Features

- Zero heap allocation — window and scratch buffer are stack-allocated via const generics
- Iterator adapter — plugs into any float iterator pipeline
- Fast fixed-size median for small windows (3, 4, 5) via sorting networks
- Falls back to quickselect for larger windows

## Usage
```rust
use hampel_filter::HampelExt;

let signal = vec![1.0f32, 1.1, 1.0, 999.0, 1.0, 1.1, 1.0];

let cleaned: Vec<f32> = signal
    .iter()
    .copied()
    .hampel::<7>(3.0) // window size 7, 3-sigma threshold
    .collect();
```

This project is licensed under either of

- BSD-3-Clause License (see [LICENSE](LICENSE.md))
- Apache License, Version 2.0 (see [LICENSE](LICENSE-APACHE.md))

at your option.
