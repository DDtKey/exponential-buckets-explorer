use crate::types::buckets::Buckets;
use crate::types::units::Unit;
use leptos::{
    component, view, CollectView, IntoView, ReadSignal, SignalGet, SignalGetUntracked, SignalWith,
};
use leptos_use::{use_intl_number_format, Notation, UseIntlNumberFormatOptions};
use std::time::Duration;
use ubyte::ToByteUnit;

#[component]
pub(crate) fn BucketsTable(buckets: ReadSignal<Buckets>, unit: ReadSignal<Unit>) -> impl IntoView {
    let intl_float_format = use_intl_number_format(
        UseIntlNumberFormatOptions::default()
            .notation(Notation::Compact)
            .minimum_fraction_digits(1)
            .maximum_fraction_digits(3),
    );

    let rounded_float_format = move |val: f64| {
        if val.is_infinite() {
            return "∞".to_string();
        }

        match unit.get() {
            Unit::Number => intl_float_format.format(val).get_untracked(),
            Unit::Bytes => (val.round() as u64).bytes().to_string(),
            Unit::Seconds => humantime::format_duration(Duration::from_secs_f64(val)).to_string(),
        }
    };

    let buckets_view = move || {
        buckets.with(|buckets| {
            buckets
                .iter()
                .map(|bucket| {
                    view! {
                        <tr>
                            <th class="text-start" scope="row">{bucket.number()}</th>
                            <td class="text-start" scope="col" title={bucket.le()}>{rounded_float_format(bucket.le())}</td>
                        </tr>
                    }
                })
                .collect_view()
        })
    };

    view! {
        <div class="table-responsive">
            <table class="table table-striped table-sm">
                <thead>
                    <tr>
                        <th class="text-start" scope="col">#</th>
                        <th class="text-start" scope="col">Less than or equal to</th>
                    </tr>
                </thead>
                <tbody>
                    {buckets_view}
                </tbody>
            </table>
        </div>
    }
}
