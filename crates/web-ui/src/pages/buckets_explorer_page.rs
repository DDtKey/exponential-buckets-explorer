use crate::components::buckets_chart::BucketsChart;
use crate::components::buckets_table::BucketsTable;
use crate::components::input::{NumberInput, SelectInput, SelectOption};
use crate::types::buckets::Buckets;
use crate::types::units::Unit;
use leptos::{component, create_signal, view, IntoView, SignalGet, SignalGetUntracked, SignalSet};
use leptos_router::use_query;
use leptos_router::Params;
use strum::IntoEnumIterator;
use wasm_bindgen_futures::JsFuture;

const DEFAULT_INITIAL_VALUE: f64 = 1.0;
const DEFAULT_FACTOR: f64 = 1.5;
const DEFAULT_BUCKETS_NUM: u32 = 15;

#[derive(leptos::Params, Default, Debug, Copy, Clone, PartialEq)]
struct BucketsQueryParams {
    initial_value: Option<f64>,
    factor: Option<f64>,
    buckets_num: Option<u32>,
    unit: Option<Unit>,
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
    let (unit, set_unit) = create_signal(Unit::Number);

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

    let copy_link_callback = move |_| {
        let mut base_url = Some("http://127.0.0.1:8080") // todo: something like option_env!("TRUNK_PUBLIC_URL")?
            .map(url::Url::parse)
            .and_then(Result::ok)
            .expect("misconfiguration: failed to parse URL from Trunk");

        base_url
            .query_pairs_mut()
            .append_pair("initial_value", &initial_value.get().to_string())
            .append_pair("factor", &factor.get().to_string())
            .append_pair("buckets_num", &buckets_num.get().to_string())
            .append_pair("unit", &unit.get().to_string());

        if let Some(clipboard) = leptos::window().navigator().clipboard() {
            use wasm_bindgen_futures::spawn_local;
            spawn_local(async move {
                let promise = clipboard.write_text(base_url.as_ref());
                match JsFuture::from(promise).await {
                    Ok(_) => {
                        log::debug!("successfully copied the link to the clipboard");
                    }
                    Err(err) => {
                        log::warn!("failed to copy the link to the clipboard: {err:?}");
                        let _ = leptos::window()
                            .alert_with_message("failed to copy the link to the clipboard");
                    }
                }
            });
        } else {
            log::warn!("failed to copy the link to the clipboard: clipboard is `None`");
            let _ = leptos::window().alert_with_message("failed to copy the link to the clipboard");
        }
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
                <div class="col-md-3 col-sm-auto">
                    <button type="button" class="btn btn-secondary" on:click=copy_link_callback
                        title="Copy the link to the clipboard to the page with the current settings">
                        Share
                    </button>
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
