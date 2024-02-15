use crate::components::buckets_chart::BucketsChart;
use crate::components::buckets_table::BucketsTable;
use crate::components::input::{NumberInput, SelectInput, SelectOption};
use crate::types::buckets::Buckets;
use crate::types::units::Unit;
use leptos::{
    component, create_signal, view, IntoView, Signal, SignalGet, SignalGetUntracked, SignalSet,
};
use leptos_router::create_query_signal;
use strum::IntoEnumIterator;

const DEFAULT_INITIAL_VALUE: f64 = 1.0;
const DEFAULT_FACTOR: f64 = 1.5;
const DEFAULT_BUCKETS_NUM: u32 = 15;

#[component]
pub(crate) fn BucketsExplorerPage() -> impl IntoView {
    let (initial_value, set_initial_value) = create_query_signal("initial_value");
    let (factor, set_factor) = create_query_signal("factor");
    let (buckets_num, set_buckets_num) = create_query_signal("buckets_num");
    let (unit, set_unit) = create_query_signal("unit");

    // map optional query parameters to default values
    let initial_value = Signal::from(move || initial_value.get().unwrap_or(DEFAULT_INITIAL_VALUE));
    let factor = Signal::from(move || factor.get().unwrap_or(DEFAULT_FACTOR));
    let buckets_num = Signal::from(move || buckets_num.get().unwrap_or(DEFAULT_BUCKETS_NUM));
    let unit = Signal::from(move || unit.get().unwrap_or_default());

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

    let options: Vec<_> = Unit::iter().collect();

    view! {
        <div class="container mt-4">
            <h1 class="mb-4">Exponential Buckets Explorer</h1>

            {/* Input Parameters Form */}
            <div class="row mb-4">
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
                <div class="col-md-3 col-sm-auto">
                    <SelectInput
                        get=unit set=set_unit options=options
                        on_change=update_buckets_callback
                        label="Unit" />
                </div>
            </div>

            <div class="mb-4 row">
                {/* Bucket Distribution Table */}
                <div class="col-md-3 col-sm-auto text-center">
                    <h2>Buckets</h2>

                    <BucketsTable buckets=buckets unit=unit />
                </div>
                {/* Chart */}
                <div class="col-md-9 col-sm-auto text-center">
                    <div>
                        <h2>Chart</h2>

                        <BucketsChart buckets=buckets />
                    </div>
                </div>
            </div>

        </div>

    }
}

impl SelectOption for Unit {
    fn label(&self) -> &'static str {
        match self {
            Unit::Number => "Number",
            Unit::Bytes => "Bytes",
            Unit::Seconds => "Seconds",
        }
    }
}
