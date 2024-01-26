use leptos::ev::Event;
use leptos::{
    component, event_target_value, view, Callable, Callback, CollectView, IntoView, ReadSignal,
    SignalSet, SignalWith, WriteSignal,
};
use std::fmt::Display;
use std::str::FromStr;

pub(crate) trait SelectOption {
    fn label(&self) -> &'static str;
}

#[component]
pub(crate) fn SelectInput<T>(
    #[prop(into)] get: ReadSignal<T>,
    #[prop(into)] set: WriteSignal<T>,
    #[prop(into)] options: Vec<T>,
    #[prop(into, optional)] on_change: Option<Callback<Event>>,
    #[prop(into)] label: String,
) -> impl IntoView
where
    T: SelectOption + Display + PartialEq + FromStr + Copy + 'static,
{
    let on_change = move |ev| {
        let Ok(new_value) = event_target_value(&ev).parse::<T>() else {
            // impossible until somebody changes selector in their browser or wrong impl of `T: FromStr`
            log::debug!("[select-input] failed to parse value");
            return;
        };
        log::debug!("[select-input] new value is: {new_value}");

        set.set(new_value);
        if let Some(callback) = on_change {
            callback.call(ev);
        }
    };

    let options_view = options
        .into_iter()
        .map(|option| {
            let is_selected = move || get.with(|v| v == &option);
            view! {
                <option value=option.to_string() selected=is_selected>
                  { option.label() }
                </option>
            }
        })
        .collect_view();

    view! {
        <label class="form-label">
            {label}
            <select class="form-select" on:change=on_change >
                {options_view}
            </select>
        </label>
    }
}
