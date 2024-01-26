use crate::types::Buckets;
use leptos::{component, view, CollectView, IntoView, ReadSignal, SignalWith};
use leptos_use::{use_intl_number_format, Notation, UseIntlNumberFormatOptions};

#[component]
pub(crate) fn BucketsTable(buckets: ReadSignal<Buckets>) -> impl IntoView {
    let rounded_float_format = use_intl_number_format(
        UseIntlNumberFormatOptions::default()
            .notation(Notation::Compact)
            .minimum_fraction_digits(1)
            .maximum_fraction_digits(3),
    );

    let buckets_view = move || {
        buckets.with(|buckets| {
            buckets
                .iter()
                .map(|bucket| {
                    view! {
                        <tr>
                            <td class="text-start" scope="col">{bucket.number()}</td>
                            <td class="text-start" scope="col" title={bucket.le()}>{rounded_float_format.format(bucket.le())}</td>
                        </tr>
                    }
                })
                .collect_view()
        })
    };

    view! {
        <div class="table-responsive">
            <table class="table table-hover">
                <thead>
                    <tr>
                        <th class="text-start" scope="col">#</th>
                        <th class="text-start" scope="col">Less than or equal to</th>
                    </tr>
                </thead>
                {buckets_view}
            </table>
        </div>
    }
}
