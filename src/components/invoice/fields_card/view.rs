use super::{create_container_renderer, create_field_renderer, create_group_renderer};
use crate::components::invoice::{
    FieldGroup, FieldItem, FieldItemValue, FieldRenderContext, HeadlessFieldsRenderer,
    HeadlessRendererConfig,
};
use leptos::prelude::*;
use std::collections::HashMap;

type FieldFilter = Box<dyn Fn(&FieldItem) -> bool + Send + Sync>;

#[component]
pub fn TailwindCollapsibleFieldCard(
    #[prop()] fields: ReadSignal<Vec<FieldItem>>,
    #[prop()] groups: ReadSignal<Vec<FieldGroup>>,
    #[prop()] form_values: RwSignal<HashMap<String, FieldItemValue>>,
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] initially_collapsed: Option<bool>,
    #[prop(optional)] on_submit: Option<Callback<HashMap<String, FieldItemValue>>>,
    #[prop(optional)] field_filter: Option<FieldFilter>,
) -> impl IntoView {
    let is_collapsed = RwSignal::new(initially_collapsed.unwrap_or(false));
    let context = RwSignal::new(FieldRenderContext::default());

    // Filter fields if filter is provided
    let filtered_fields = Memo::new(move |_| {
        let all_fields = fields.get();
        if let Some(filter) = &field_filter {
            all_fields.into_iter().filter(|f| filter(f)).collect()
        } else {
            all_fields
        }
    });

    let config = HeadlessRendererConfig {
        auto_generation_enabled: true,
        validation_enabled: true,
        real_time_validation: true,
        auto_save_enabled: false,
        auto_save_debounce_ms: 1000,
    };

    // Create the renderers using the factory functions
    let field_renderer = RwSignal::new(create_field_renderer());
    let group_renderer = RwSignal::new(create_group_renderer());
    let container_renderer = RwSignal::new(create_container_renderer(
        title.clone(),
        is_collapsed,
        form_values,
        on_submit,
    ));

    view! {
        <HeadlessFieldsRenderer
            fields=filtered_fields
            groups=groups
            form_values=form_values
            field_renderer=field_renderer
            group_renderer=group_renderer
            container_renderer=container_renderer
            config=config
            context=context
        />
    }
}
