<p align="center"><img alt="exponential-buckets-explorer" src="https://github.com/DDtKey/exponential-buckets-explorer/raw/main/assets/images/logo.png"></p>

A simple web page and desktop application for calculating buckets based on the exponential function.

Available at [buckets-exp.ddtkey.com](https://buckets-exp.ddtkey.com/), the desktop version can be downloaded from the release page: [latest](https://github.com/DDtKey/exponential-buckets-explorer/releases/latest)

Can be useful for checking parameters when configuring histograms in Prometheus, for example:
- rust: [exponential_buckets](https://docs.rs/prometheus/0.13.3/prometheus/fn.exponential_buckets.html)
- go: [ExponentialBuckets](https://pkg.go.dev/github.com/prometheus/client_golang/prometheus#ExponentialBuckets)

The application is built on [Leptos] and [Tauri].


[Leptos]: https://github.com/leptos-rs/leptos
[Tauri]: https://github.com/tauri-apps/tauri
