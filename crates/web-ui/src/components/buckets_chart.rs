use crate::types::buckets::Buckets;
use chart_js_rs::scatter::Scatter;
use chart_js_rs::{
    ChartOptions, ChartScale, Dataset, DatasetDataExt, NoAnnotations, XYDataset, XYPoint,
};
use std::collections::HashMap;

use leptos::{component, create_effect, view, IntoView, MaybeSignal, SignalWith};

#[component]
pub(crate) fn BucketsChart(#[prop(into)] buckets: MaybeSignal<Buckets>) -> impl IntoView {
    const ID: &str = "buckets-chart";

    // re-render chart on `buckets` change
    create_effect(move |_| {
        buckets.with(|buckets| {
            log::debug!("re-rendering buckets chart");
            let chart: Scatter<NoAnnotations> = Scatter {
                id: ID.to_string(),
                options: ChartOptions {
                    scales: Some(HashMap::from([
                        (
                            "x".into(),
                            ChartScale {
                                r#type: "linear".into(),
                                ..Default::default()
                            },
                        ),
                        (
                            "y".into(),
                            ChartScale {
                                r#type: "linear".into(),
                                ..Default::default()
                            },
                        ),
                    ])),
                    ..Default::default()
                },
                data: Dataset {
                    labels: None,
                    datasets: Vec::from([XYDataset {
                        r#type: "line".into(),
                        label: "Buckets distribution".into(),
                        data: buckets
                            .iter()
                            .map(|b| XYPoint {
                                x: b.number().into(),
                                y: b.le().into(),
                                description: String::new(),
                            })
                            .collect::<Vec<_>>()
                            .to_dataset_data(),
                        ..Default::default()
                    }]),
                },
                ..Default::default()
            };

            chart.to_chart().render();
            log::debug!("re-rendering finished");
        })
    });

    view! {
        <canvas id={ID}></canvas>
    }
}
