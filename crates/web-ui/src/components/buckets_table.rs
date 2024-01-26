use crate::types::buckets::Buckets;
use leptos::{component, view, CollectView, IntoView, ReadSignal, SignalGet, SignalWith};
use leptos_use::{use_intl_number_format, Notation, NumberStyle, UseIntlNumberFormatOptions};

#[component]
pub(crate) fn BucketsTable(
    buckets: ReadSignal<Buckets>,
    unit: ReadSignal<String>,
) -> impl IntoView {
    let rounded_float_format = move || {
        let unit = Some(unit.get()).filter(|v| !v.is_empty() && v != "a");
        let mut options = UseIntlNumberFormatOptions::default()
            .minimum_fraction_digits(1)
            .maximum_fraction_digits(3);

        if let Some(unit) = unit {
            options = options.style(NumberStyle::Unit).unit(unit);
        } else {
            options = options.notation(Notation::Compact);
        }

        use_intl_number_format(options)
    };

    let buckets_view = move || {
        buckets.with(|buckets| {
            buckets
                .iter()
                .map(|bucket| {
                    view! {
                        <tr>
                            <th class="text-start" scope="row">{bucket.number()}</th>
                            <td class="text-start" scope="col" title={bucket.le()}>{rounded_float_format().format(bucket.le())}</td>
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
