use leptos::ev::Event;
use leptos::{
    component, event_target_value, view, Callable, Callback, CollectView, IntoView, ReadSignal,
    SignalGet, SignalSet, WriteSignal,
};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub(crate) struct SelectOption {
    pub(crate) value: String,
    pub(crate) label: String,
}

#[component]
pub(crate) fn SelectInput(
    #[prop(into)] get: ReadSignal<String>,
    #[prop(into)] set: WriteSignal<String>,
    #[prop(into)] options: Vec<SelectOption>,
    #[prop(into, optional)] on_change: Option<Callback<Event>>,
    #[prop(into)] label: String,
) -> impl IntoView {
    let on_change = move |ev| {
        let new_value = event_target_value(&ev);
        log::debug!("[select-input] new value is: {new_value}");

        set.set(new_value);
        if let Some(callback) = on_change {
            callback.call(ev);
        }
    };

    let options_view = options
        .into_iter()
        .map(|option| {
            let value = option.value.clone();
            let is_selected = move || get.get() == value;
            view! {
                <option value=option.value selected=is_selected>
                  { option.label }
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
