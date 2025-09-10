use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::window;

use super::types::{Breakpoint, DeviceType, FormState, GridActions, ItemState, ValidationResult};
use super::validation::FormValidation;
use super::{FormData, ItemData};

/// Responsive hook that tracks screen size changes
pub fn use_responsive() -> (ReadSignal<Breakpoint>, ReadSignal<DeviceType>) {
    let (breakpoint, set_breakpoint) = signal(Breakpoint::Desktop);
    let (device_type, set_device_type) = signal(DeviceType::Desktop);

    let update_breakpoint = move || {
        if let Some(window) = window() {
            let width = window.inner_width().unwrap().as_f64().unwrap_or(1024.0);

            let new_breakpoint = if width < 640.0 {
                Breakpoint::Mobile
            } else if width < 1024.0 {
                Breakpoint::Tablet
            } else {
                Breakpoint::Desktop
            };

            let new_device_type = if width < 1024.0 {
                DeviceType::Mobile
            } else {
                DeviceType::Desktop
            };

            set_breakpoint.set(new_breakpoint);
            set_device_type.set(new_device_type);
        }
    };

    // Initial setup
    Effect::new(move |_| {
        update_breakpoint();
    });

    // Set up media query listeners for responsive changes
    Effect::new(move |_| {
        if let Some(_window) = window() {
            setup_media_query_listener("(max-width: 639px)", update_breakpoint);
            setup_media_query_listener(
                "(min-width: 640px) and (max-width: 1023px)",
                update_breakpoint,
            );
            setup_media_query_listener("(min-width: 1024px)", update_breakpoint);
        }
    });

    (breakpoint, device_type)
}

fn setup_media_query_listener<F>(query_str: &str, callback: F)
where
    F: Fn() + 'static,
{
    if let Some(window) = window() {
        if let Ok(Some(query)) = window.match_media(query_str) {
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                callback();
            }) as Box<dyn Fn(_)>);

            let _ =
                query.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref());
            closure.forget(); // Keep the closure alive
        }
    }
}

/// Hook that provides reactive access to grid state components
pub struct UseEditableGridReturn<T, P>
where
    T: Send + Sync + 'static,
    P: Send + Sync + 'static,
{
    pub items: Memo<Vec<ItemState<T>>>,
    pub form_state: Memo<FormState<P>>,
    pub device_type: ReadSignal<DeviceType>,
    pub breakpoint: ReadSignal<Breakpoint>,
    pub is_empty: Memo<bool>,
    pub item_count: Memo<usize>,
    pub actions: GridActions<P>,
}

/// Main headless hook for editable grid functionality
pub fn use_editable_grid<T, P>(initial_data: Vec<T>) -> UseEditableGridReturn<T, P>
where
    T: FormData<FormProps = P>
        + ItemData
        + Clone
        + PartialEq
        + Send
        + Sync
        + std::fmt::Debug
        + 'static,
    P: Default + Clone + PartialEq + Send + Sync + 'static,
{
    let (items, set_items) = signal(initial_data);
    let (current_form, set_current_form) = signal(P::default());
    let (editing_index, set_editing_index) = signal(None::<usize>);

    // Use responsive hook
    let (breakpoint, device_type) = use_responsive();

    // Create derived signals for item states
    let item_states = Memo::new(move |_| {
        items
            .get()
            .into_iter()
            .enumerate()
            .map(|(index, data)| ItemState::new(data, index))
            .collect::<Vec<_>>()
    });

    // Create derived signals for computed values
    let is_empty = Memo::new(move |_| item_states.get().is_empty());
    let item_count = Memo::new(move |_| item_states.get().len());

    // Create form state signal
    let form_state = Memo::new(move |_| FormState::new(current_form.get(), editing_index.get()));

    // Create actions
    let actions = create_grid_actions(
        items,
        set_items,
        set_current_form,
        editing_index,
        set_editing_index,
    );

    UseEditableGridReturn {
        items: item_states,
        form_state,
        device_type,
        breakpoint,
        is_empty,
        item_count,
        actions,
    }
}

fn create_grid_actions<T, P>(
    items: ReadSignal<Vec<T>>,
    set_items: WriteSignal<Vec<T>>,
    set_current_form: WriteSignal<P>,
    editing_index: ReadSignal<Option<usize>>,
    set_editing_index: WriteSignal<Option<usize>>,
) -> GridActions<P>
where
    T: FormData<FormProps = P> + Clone + PartialEq + Send + Sync + std::fmt::Debug + 'static,
    P: Default + Clone + PartialEq + Send + Sync + 'static,
{
    let submit_form = Callback::new(move |form_data: P| {
        let new_item = T::from_form_props(&form_data);
        log::debug!("new_item: {new_item:#?}");

        let mut current_items = items.get_untracked();

        match editing_index.get_untracked() {
            Some(index) => {
                if index < current_items.len() {
                    current_items[index] = new_item;
                }
            }
            None => {
                current_items.push(new_item);
            }
        }

        set_items.set(current_items);

        log::debug!("new items {:#?}", items.get().clone());

        set_editing_index.set(None);
        set_current_form.set(P::default());
    });

    let edit_item = Callback::new(move |index: usize| {
        // Use items signal directly instead of set_items.with_untracked
        let items = items.get_untracked();
        if let Some(item) = items.get(index) {
            set_current_form.set(item.to_form_props());
            set_editing_index.set(Some(index));
        }
    });

    let delete_item = Callback::new(move |index: usize| {
        set_items.update(|items| {
            if index < items.len() {
                items.remove(index);
            }
        });

        if editing_index.get_untracked() == Some(index) {
            set_current_form.set(P::default());
            set_editing_index.set(None);
        }
    });

    let cancel_edit = Callback::new(move |_| {
        set_current_form.set(P::default());
        set_editing_index.set(None);
    });

    let update_form = Callback::new(move |form_data: P| {
        set_current_form.set(form_data);
    });

    let clear_form = Callback::new(move |_| {
        set_current_form.set(P::default());
        if editing_index.get_untracked().is_some() {
            set_editing_index.set(None);
        }
    });

    GridActions {
        submit_form,
        edit_item,
        delete_item,
        cancel_edit,
        update_form,
        clear_form,
    }
}

/// Helper hook for form field management
pub fn use_form_field<T>(initial_value: T, on_change: Callback<T>) -> (ReadSignal<T>, Callback<T>)
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    let (value, set_value) = signal(initial_value);

    let update_value = Callback::new(move |new_value: T| {
        set_value.set(new_value.clone());
        on_change.run(new_value);
    });

    (value, update_value)
}

/// Hook for form validation
pub fn use_form_validation<T>(form_data: ReadSignal<T>) -> Memo<ValidationResult>
where
    T: FormValidation + Clone + PartialEq + Send + Sync + 'static,
{
    Memo::new(move |_| form_data.get().validate())
}
