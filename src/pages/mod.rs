use crate::components::buckets_chart::BucketsChart;
use crate::components::buckets_table::BucketsTable;
use crate::components::input::NumberInput;
use crate::types::Buckets;
use leptos::{component, create_signal, view, IntoView, SignalGet, SignalSet};

const DEFAULT_INITIAL_VALUE: f64 = 1.0;
const DEFAULT_FACTOR: f64 = 1.5;
const DEFAULT_BUCKETS_NUM: u32 = 15;

#[component]
pub(crate) fn BucketsExplorerPage() -> impl IntoView {
    let (initial_value, set_initial_value) = create_signal(DEFAULT_INITIAL_VALUE);
    let (factor, set_factor) = create_signal(DEFAULT_FACTOR);
    let (buckets_num, set_buckets_num) = create_signal(DEFAULT_BUCKETS_NUM);

    let (buckets, set_buckets) = create_signal(Buckets::calculate(
        DEFAULT_INITIAL_VALUE,
        DEFAULT_FACTOR,
        DEFAULT_BUCKETS_NUM,
    ));

    let update_buckets_callback = move |_| {
        set_buckets.set(Buckets::calculate(
            initial_value.get(),
            factor.get(),
            buckets_num.get(),
        ))
    };

    let table_header = ["#", "From", "To (exclusive)"]
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    view! {
        <div class="container mt-4">
            <h1 class="mb-4">Exponential Buckets Calculator</h1>

            {/* Input Parameters Form */}
            <div class="row mb-4">
                <div class="col-sm-auto">
                    <NumberInput
                        get=initial_value set=set_initial_value on_change=update_buckets_callback
                        label="Initial value" min=0.0 step=0.1 />
                </div>
                <div class="col-sm-auto">
                    <NumberInput
                        get=factor set=set_factor on_change=update_buckets_callback
                        label="Factor" min=1.0 step=0.1 />
                </div>
                <div class="col-sm-auto">
                    <NumberInput
                        get=buckets_num set=set_buckets_num on_change=update_buckets_callback
                        label="Number of Buckets" min=1_u32 max=100_u32 />
                </div>
            </div>

            <div class="mb-4 row">
                {/* Bucket Distribution Table */}
                <div class="col-sm-6 text-center">
                    <h2>Bucket Distribution Table</h2>

                    <BucketsTable buckets=buckets headers=table_header />
                </div>
                {/* Chart */}
                <div class="col-sm-6 text-center">
                    <div>
                        <h2>Chart</h2>

                        <BucketsChart buckets=buckets />
                    </div>
                </div>
            </div>

        </div>

    }
}
