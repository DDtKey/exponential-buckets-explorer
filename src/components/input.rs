use leptos::ev::Event;
use leptos::wasm_bindgen::JsValue;
use leptos::{
    component, create_signal, event_target_value, view, Callable, Callback, IntoAttribute,
    IntoView, ReadSignal, SignalSet, SignalWith, WriteSignal,
};
use std::fmt::{Debug, Display};
use std::str::FromStr;

#[component]
pub(crate) fn NumberInput<T>(
    #[prop(into)] get: ReadSignal<T>,
    #[prop(into)] set: WriteSignal<T>,
    #[prop(into, optional)] on_change: Option<Callback<Event>>,
    #[prop(into)] label: String,
    #[prop(optional)] min: Option<T>,
    #[prop(optional)] max: Option<T>,
    #[prop(optional)] step: Option<T>,
) -> impl IntoView
where
    T: Into<JsValue> + Copy + FromStr + PartialOrd + Display + Debug + IntoAttribute + 'static,
    <T as FromStr>::Err: Clone + std::error::Error + 'static,
{
    let (error, set_error) = create_signal(None);
    let is_wrong = move || error.with(|e| e.is_some());
    let error_message =
        move || error.with(|e| e.as_ref().map(ToString::to_string).unwrap_or_default());

    let on_change = move |ev| {
        let result = T::from_str(&event_target_value(&ev))
            .map_err(|err| err.to_string())
            .and_then(|value| validate_value(value, min, max));
        log::debug!("parsed result of `on_change`: {:?}", result);

        match result {
            Ok(new_value) => {
                set.set(new_value);
                if let Some(callback) = on_change {
                    callback.call(ev);
                }
                set_error.set(None);
            }
            Err(err) => set_error.set(Some(err)),
        };
    };

    view! {
        <label class="form-label">
            {label}
            <input type="number" min=min max=max step=step class="form-control" class:is-invalid=is_wrong prop:value=get on:change=on_change />
            <div class="invalid-feedback" class:d-block=is_wrong >
                { error_message }
            </div>
        </label>
    }
}

fn validate_value<T: PartialOrd + Display + Copy>(
    value: T,
    min: Option<T>,
    max: Option<T>,
) -> Result<T, String> {
    let min = min.unwrap_or(value);
    let max = max.unwrap_or(value);
    if value < min {
        Err(format!("Value must be greater than or equal to {}", min))
    } else if value > max {
        Err(format!("Value must be less than or equal to {}", max))
    } else {
        Ok(value)
    }
}
