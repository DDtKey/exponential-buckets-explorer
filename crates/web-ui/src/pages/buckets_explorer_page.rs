use crate::components::buckets_chart::BucketsChart;
use crate::components::buckets_table::BucketsTable;
use crate::components::input::NumberInput;
use crate::types::Buckets;
use leptos::{component, create_signal, view, IntoView, SignalGet, SignalGetUntracked, SignalSet};
use leptos_router::use_query;
use leptos_router::Params;

const DEFAULT_INITIAL_VALUE: f64 = 1.0;
const DEFAULT_FACTOR: f64 = 1.5;
const DEFAULT_BUCKETS_NUM: u32 = 15;

#[derive(leptos::Params, Default, Debug, Copy, Clone, PartialEq)]
struct BucketsQueryParams {
    initial_value: Option<f64>,
    factor: Option<f64>,
    buckets_num: Option<u32>,
}

#[component]
pub(crate) fn BucketsExplorerPage() -> impl IntoView {
    let query = use_query::<BucketsQueryParams>()
        .get_untracked()
        .unwrap_or_else(|e| {
            log::debug!("Failed to parse query params: {e}, using default values instead");
            Default::default()
        });

    let (initial_value, set_initial_value) =
        create_signal(query.initial_value.unwrap_or(DEFAULT_INITIAL_VALUE));
    let (factor, set_factor) = create_signal(query.factor.unwrap_or(DEFAULT_FACTOR));
    let (buckets_num, set_buckets_num) =
        create_signal(query.buckets_num.unwrap_or(DEFAULT_BUCKETS_NUM));

    let (buckets, set_buckets) = create_signal(Buckets::calculate(
        initial_value.get_untracked(),
        factor.get_untracked(),
        buckets_num.get_untracked(),
    ));

    let update_buckets_callback = move |_| {
        set_buckets.set(Buckets::calculate(
            initial_value.get(),
            factor.get(),
            buckets_num.get(),
        ))
    };

    view! {
        <div class="container mt-4">
            <h1 class="mb-4">Exponential Buckets Explorer</h1>

            {/* Input Parameters Form */}
            <div class="row mb-4 align-items-end">
                <div class="col-md-3 col-sm-auto">
                    <NumberInput
                        get=initial_value set=set_initial_value on_change=update_buckets_callback
                        label="Initial value" min=0.0 step=0.1 />
                </div>
                <div class="col-md-3 col-sm-auto">
                    <NumberInput
                        get=factor set=set_factor on_change=update_buckets_callback
                        label="Factor" min=1.0 step=0.1 />
                </div>
                <div class="col-md-3 col-sm-auto">
                    <NumberInput
                        get=buckets_num set=set_buckets_num on_change=update_buckets_callback
                        label="Number of buckets" min=1_u32 max=100_u32 />
                </div>
            </div>

            <div class="mb-4 row">
                {/* Bucket Distribution Table */}
                <div class="col-md-3 col-sm-auto text-center">
                    <h2>Buckets</h2>

                    <BucketsTable buckets=buckets />
                </div>
                {/* Chart */}
                <div class="col-md-6 col-sm-auto text-center">
                    <div>
                        <h2>Chart</h2>

                        <BucketsChart buckets=buckets />
                    </div>
                </div>
            </div>

        </div>

    }
}
