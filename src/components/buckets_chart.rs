use crate::types::Buckets;
use charming::component::Axis;
use charming::element::AxisType;
use charming::series::Line;
use charming::{Chart, WasmRenderer};

use leptos::{component, create_effect, view, IntoView, ReadSignal, SignalWith};

#[component]
pub(crate) fn BucketsChart(buckets: ReadSignal<Buckets>) -> impl IntoView {
    let id = "buckets-chart";

    // re-render chart on `buckets` change
    create_effect(move |_| {
        buckets.with(|buckets| {
            log::debug!("re-rendering buckets chart");
            let chart = Chart::new()
                .x_axis(Axis::new().type_(AxisType::Value))
                .y_axis(Axis::new().type_(AxisType::Value))
                .series(
                    Line::new().data(
                        buckets
                            .iter()
                            .map(|b| vec![f64::from(b.number()), b.start()])
                            .collect::<Vec<_>>(),
                    ),
                );

            let renderer = WasmRenderer::new(600, 400);
            renderer.render(id, &chart).unwrap();
        })
    });

    view! {
        <div id={id}></div>
    }
}
