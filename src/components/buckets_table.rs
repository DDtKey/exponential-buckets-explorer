use crate::types::Buckets;
use leptos::{component, view, CollectView, IntoView, ReadSignal, SignalWith};
use leptos_use::{use_intl_number_format, Notation, UseIntlNumberFormatOptions};

#[component]
pub(crate) fn BucketsTable(
    buckets: ReadSignal<Buckets>,
    #[prop(into)] headers: Vec<String>,
) -> impl IntoView {
    let head_columns = headers
        .iter()
        .map(|header| view! { <th class="text-start" scope="col">{header}</th> })
        .collect_view();

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
                            <td class="text-start" scope="col" title={bucket.start()}>{rounded_float_format.format(bucket.start())}</td>
                            <td class="text-start" scope="col" title={bucket.end()}>{rounded_float_format.format(bucket.end())}</td>
                        </tr>
                    }
                })
                .collect_view()
        })
    };

    view! {
        <div class="table-responsive">
            <table class="table">
                <thead>
                    <tr>
                        {head_columns}
                    </tr>
                </thead>
                {buckets_view}
            </table>
        </div>
    }
}
