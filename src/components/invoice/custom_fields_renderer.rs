use crate::components::invoice::{
    FieldGroup, FieldItem, FieldItemValue, FieldType, FieldValue, GenerationContext,
};
use leptos::prelude::*;
use std::collections::HashMap;
use time::OffsetDateTime;

/// Field render data passed to custom renderers
#[derive(Clone, Debug, PartialEq)]
pub struct FieldRenderData {
    pub field: FieldItem,
    pub value: FieldValue,
    pub validation_errors: Vec<String>,
    pub is_focused: bool,
    pub is_dirty: bool,
}

/// Group render data passed to custom group renderers
#[derive(Clone, Debug, PartialEq)]
pub struct GroupRenderData {
    pub group: FieldGroup,
    pub fields: Vec<FieldRenderData>,
    pub is_expanded: bool,
}

/// Container render data for the entire form
#[derive(Clone, Debug, PartialEq)]
pub struct ContainerRenderData {
    pub groups: Vec<GroupRenderData>,
    pub ungrouped_fields: Vec<FieldRenderData>,
    pub total_fields: usize,
    pub total_errors: usize,
    pub is_valid: bool,
}

/// Field interaction callbacks
#[derive(Clone)]
pub struct FieldCallbacks {
    pub on_value_change: Callback<(String, FieldValue)>, // (field_id, new_value)
    pub on_focus: Option<Callback<String>>,              // field_id
    pub on_blur: Option<Callback<String>>,               // field_id
    pub on_validation_trigger: Option<Callback<String>>, // field_id
}

/// Group interaction callbacks
#[derive(Clone)]
pub struct GroupCallbacks {
    pub on_expand_toggle: Option<Callback<String>>, // group_id
    pub on_add_repetition: Option<Callback<String>>, // group_id for repeatable groups
    pub on_remove_repetition: Option<Callback<(String, usize)>>, // (group_id, index)
}

/// Container interaction callbacks
#[derive(Clone)]
pub struct ContainerCallbacks {
    pub on_form_submit: Option<Callback<HashMap<String, FieldItemValue>>>,
    pub on_form_reset: Option<Callback<()>>,
    pub on_form_validate: Option<Callback<()>>,
}

/// Render function type definitions - completely headless
pub type FieldRenderer = Box<dyn Fn(FieldRenderData, FieldCallbacks) -> AnyView + Send + Sync>;
pub type GroupRenderer =
    Box<dyn Fn(GroupRenderData, GroupCallbacks, AnyView) -> AnyView + Send + Sync>;
pub type ContainerRenderer =
    Box<dyn Fn(ContainerRenderData, ContainerCallbacks, AnyView) -> AnyView + Send + Sync>;

/// Filter function type for custom field filtering
pub type FieldFilter = Box<dyn Fn(&FieldItem) -> bool + Send + Sync>;

/// Auto-value generator registry trait
pub trait AutoValueGeneratorRegistry: Send + Sync {
    fn generate(
        &self,
        generator_id: &str,
        parameters: &HashMap<String, String>,
        context: &GenerationContext,
    ) -> Result<String, String>;
    fn list_generators(&self) -> Vec<(String, String)>; // (id, description) pairs
}

/// Field validation trait
pub trait FieldValidator: Send + Sync {
    fn validate(&self, field: &FieldItem, value: &FieldValue) -> Vec<String>;
}

/// Render context for field rendering and value management
#[derive(Default)]
pub struct FieldRenderContext {
    pub current_values: HashMap<String, FieldItemValue>,
    pub validation_errors: HashMap<String, Vec<String>>,
    pub focused_field: Option<String>,
    pub dirty_fields: std::collections::HashSet<String>,
    pub expanded_groups: std::collections::HashSet<String>,
    pub counter_states: HashMap<String, i64>,
}

/// Configuration for the headless renderer - only behavior, no presentation
#[derive(Clone)]
pub struct HeadlessRendererConfig {
    pub auto_generation_enabled: bool,
    pub validation_enabled: bool,
    pub real_time_validation: bool,
    pub auto_save_enabled: bool,
    pub auto_save_debounce_ms: u64,
}

impl Default for HeadlessRendererConfig {
    fn default() -> Self {
        Self {
            auto_generation_enabled: true,
            validation_enabled: true,
            real_time_validation: false,
            auto_save_enabled: false,
            auto_save_debounce_ms: 1000,
        }
    }
}

/// The completely headless renderer component
#[component]
pub fn HeadlessFieldsRenderer(
    #[prop()] fields: Memo<Vec<FieldItem>>,
    #[prop()] groups: ReadSignal<Vec<FieldGroup>>,
    #[prop()] form_values: RwSignal<HashMap<String, FieldItemValue>>,
    #[prop()] field_renderer: RwSignal<FieldRenderer>,
    #[prop(optional)] group_renderer: Option<RwSignal<GroupRenderer>>,
    #[prop(optional)] container_renderer: Option<RwSignal<ContainerRenderer>>,
    #[prop(optional)] config: Option<HeadlessRendererConfig>,
    #[prop(optional)] filter: Option<FieldFilter>,
    #[prop(optional)] context: Option<RwSignal<FieldRenderContext>>,
    #[prop(optional)] auto_generator_registry: Option<Box<dyn AutoValueGeneratorRegistry>>,
    #[prop(optional)] field_validator: Option<Box<dyn FieldValidator>>,
) -> impl IntoView {
    let config = config.unwrap_or_default();
    let render_context = context.unwrap_or_else(|| RwSignal::new(FieldRenderContext::default()));

    // Apply filtering
    let filtered_fields = Memo::new({
        let filter = filter;
        move |_| {
            let all_fields = fields.get();
            if let Some(ref filter_fn) = filter {
                all_fields
                    .into_iter()
                    .filter(|f| filter_fn(f))
                    .collect::<Vec<_>>()
            } else {
                all_fields
            }
        }
    });

    // Create field lookup map
    let fields_map = Memo::new(move |_| {
        filtered_fields
            .get()
            .into_iter()
            .map(|f| (f.id.clone(), f))
            .collect::<HashMap<String, FieldItem>>()
    });

    // Create group lookup map
    let groups_map = Memo::new(move |_| {
        groups
            .get()
            .into_iter()
            .map(|g| (g.id.clone(), g))
            .collect::<HashMap<String, FieldGroup>>()
    });

    // Auto-value generation effect
    Effect::new({
        let config = config.clone();
        move |_| {
            if !config.auto_generation_enabled {
                return;
            }

            let fields_map = fields_map.get();

            for field in fields_map.values() {
                if matches!(field.field_type, FieldType::AutoGenerated) {
                    if let Some(ref auto_gen) = field.auto_generation {
                        let (current_values, counter_states) = render_context.with(|context| {
                            let current_values = context
                                .current_values
                                .clone()
                                .into_iter()
                                .map(|(k, v)| (k, v.value))
                                .collect();

                            let counter_states = context.counter_states.clone();

                            (current_values, counter_states)
                        });

                        let generation_context = GenerationContext {
                            current_values,
                            counter_states,
                            current_date: OffsetDateTime::now_utc(),
                        };

                        match auto_gen.generate(&generation_context, &field.id) {
                            Ok(generated_value) => {
                                form_values.update(|values| {
                                    values.insert(
                                        field.id.clone(),
                                        FieldItemValue {
                                            id: field.id.clone(),
                                            label: field.name.clone(),
                                            value: FieldValue::AutoGenerated(generated_value),
                                        },
                                    );
                                });
                            }
                            Err(error) => {
                                render_context.update(|ctx| {
                                    ctx.validation_errors.insert(field.id.clone(), vec![error]);
                                });
                            }
                        }
                    }
                }

                // Handle date defaults
                if matches!(field.field_type, FieldType::Date) {
                    if let Some(ref date_default) = field.date_default {
                        if !form_values.get().contains_key(&field.id) {
                            if let Ok(default_date) = date_default.resolve() {
                                form_values.update(|values| {
                                    values.insert(
                                        field.id.clone(),
                                        FieldItemValue {
                                            id: field.id.clone(),
                                            label: field.name.clone(),
                                            value: FieldValue::Date(default_date),
                                        },
                                    );
                                });
                            }
                        }
                    }
                }
            }
        }
    });

    // Validation effect
    Effect::new({
        let config = config.clone();
        move |_| {
            if !config.validation_enabled {
                return;
            }

            if let Some(validator) = &field_validator {
                let fields_map = fields_map.get();
                let form_vals = form_values.get();

                render_context.update(|ctx| {
                    ctx.validation_errors.clear();

                    for field in fields_map.values() {
                        if let Some(field_value) = form_vals.get(&field.id) {
                            let errors = validator.validate(field, &field_value.value);
                            if !errors.is_empty() {
                                ctx.validation_errors.insert(field.id.clone(), errors);
                            }
                        }
                    }
                });
            }
        }
    });

    // Function to get current container data (not memoized due to complexity)
    let get_container_data = move || -> ContainerRenderData {
        let fields_map = fields_map.get();
        let groups_map = groups_map.get();
        let form_vals = form_values.get();

        let mut groups_data = Vec::new();
        let mut used_fields = std::collections::HashSet::new();
        let mut total_errors = 0;

        // Process defined groups
        for group in groups_map.values() {
            let mut group_fields = Vec::new();

            for field_id in &group.field_ids {
                if let Some(field) = fields_map.get(field_id) {
                    if !field.is_hidden {
                        let field_value = form_vals
                            .get(field_id)
                            .map(|v| v.value.clone())
                            .unwrap_or_else(|| match field.field_type {
                                FieldType::Number => FieldValue::Number(0.0),
                                FieldType::Checkbox => FieldValue::Checkbox(field.default_checked),
                                _ => FieldValue::Text(field.default_value.clone()),
                            });

                        let (validation_errors, is_focused, is_dirty) =
                            render_context.with(|context| {
                                let validation_errors = context
                                    .validation_errors
                                    .get(field_id)
                                    .cloned()
                                    .unwrap_or_default();
                                let is_focused = context.focused_field.as_ref() == Some(field_id);
                                let is_dirty = context.dirty_fields.contains(field_id);

                                (validation_errors, is_focused, is_dirty)
                            });
                        total_errors += validation_errors.len();

                        group_fields.push(FieldRenderData {
                            field: field.clone(),
                            value: field_value,
                            validation_errors,
                            is_focused,
                            is_dirty,
                        });

                        used_fields.insert(field_id.clone());
                    }
                }
            }

            if !group_fields.is_empty() {
                // Sort by order_in_group if specified
                group_fields.sort_by_key(|f| f.field.order_in_group.unwrap_or(0));
                groups_data.push(GroupRenderData {
                    group: group.clone(),
                    fields: group_fields,
                    is_expanded: render_context
                        .with(|context| context.expanded_groups.contains(&group.id)),
                });
            }
        }

        // Process ungrouped fields
        let ungrouped_fields: Vec<FieldRenderData> = fields_map
            .values()
            .filter(|field| !used_fields.contains(&field.id) && !field.is_hidden)
            .map(|field| {
                let field_value = form_vals
                    .get(&field.id)
                    .map(|v| v.value.clone())
                    .unwrap_or_else(|| match field.field_type {
                        FieldType::Number => FieldValue::Number(0.0),
                        FieldType::Checkbox => FieldValue::Checkbox(field.default_checked),
                        _ => FieldValue::Text(field.default_value.clone()),
                    });

                let (validation_errors, is_focused, is_dirty) = render_context.with(|context| {
                    let validation_errors = context
                        .validation_errors
                        .get(&field.id)
                        .cloned()
                        .unwrap_or_default();
                    let is_focused = context.focused_field.as_ref() == Some(&field.id);
                    let is_dirty = context.dirty_fields.contains(&field.id);

                    (validation_errors, is_focused, is_dirty)
                });

                total_errors += validation_errors.len();

                FieldRenderData {
                    field: field.clone(),
                    value: field_value,
                    validation_errors,
                    is_focused,
                    is_dirty,
                }
            })
            .collect();

        let total_fields =
            groups_data.iter().map(|g| g.fields.len()).sum::<usize>() + ungrouped_fields.len();

        ContainerRenderData {
            groups: groups_data,
            ungrouped_fields,
            total_fields,
            total_errors,
            is_valid: total_errors == 0,
        }
    };

    // Field callbacks
    let field_callbacks = FieldCallbacks {
        on_value_change: Callback::new({
            move |(field_id, new_value): (String, FieldValue)| {
                // Update form values
                form_values.update(|values| {
                    if let Some(field_item) = values.get(&field_id) {
                        let mut updated_item = field_item.clone();
                        updated_item.value = new_value;
                        values.insert(field_id.clone(), updated_item);
                    } else {
                        // Find field name from fields
                        let field_name = fields
                            .get()
                            .iter()
                            .find(|f| f.id == field_id)
                            .map(|f| f.name.clone())
                            .unwrap_or_else(|| field_id.clone());

                        values.insert(
                            field_id.clone(),
                            FieldItemValue {
                                id: field_id.clone(),
                                label: field_name,
                                value: new_value,
                            },
                        );
                    }
                });

                // Mark field as dirty
                render_context.update(|ctx| {
                    ctx.dirty_fields.insert(field_id);
                });
            }
        }),
        on_focus: Some(Callback::new({
            move |field_id: String| {
                render_context.update(|ctx| {
                    ctx.focused_field = Some(field_id);
                });
            }
        })),
        on_blur: Some(Callback::new({
            move |_field_id: String| {
                render_context.update(|ctx| {
                    ctx.focused_field = None;
                });
            }
        })),
        on_validation_trigger: Some(Callback::new({
            let config = config.clone();
            move |_field_id: String| {
                if config.real_time_validation {
                    // Trigger validation - implementation depends on validation system
                }
            }
        })),
    };

    // Group callbacks
    let group_callbacks = GroupCallbacks {
        on_expand_toggle: Some(Callback::new({
            move |group_id: String| {
                render_context.update(|ctx| {
                    if ctx.expanded_groups.contains(&group_id) {
                        ctx.expanded_groups.remove(&group_id);
                    } else {
                        ctx.expanded_groups.insert(group_id);
                    }
                });
            }
        })),
        on_add_repetition: None,    // Implement based on your needs
        on_remove_repetition: None, // Implement based on your needs
    };

    // Container callbacks
    let container_callbacks = ContainerCallbacks {
        on_form_submit: None, // Implement based on your needs
        on_form_reset: Some(Callback::new(move |_| {
            form_values.update(|values| values.clear());
            render_context.update(|ctx| {
                ctx.validation_errors.clear();
                ctx.dirty_fields.clear();
                ctx.focused_field = None;
            });
        })),
        on_form_validate: None, // Implement based on your needs
    };

    // Create the reactive view that renders fields
    view! {
        <div>
            {move || {
                let container_data = get_container_data();
                let field_callbacks = field_callbacks.clone();
                let mut all_field_views = Vec::new();
                for group_data in &container_data.groups {
                    let group_field_views: Vec<AnyView> = group_data
                        .fields
                        .iter()
                        .map(|field_data| {
                            field_renderer
                                .with(|field_renderer_fn| {
                                    field_renderer_fn(field_data.clone(), field_callbacks.clone())
                                })
                        })
                        .collect();
                    let group_content = // Render grouped fields

                    view! { {group_field_views.into_iter().collect::<Vec<_>>()} }
                        .into_any();
                    let group_view = if let Some(group_renderer) = &group_renderer {
                        group_renderer
                            .with(|group_renderer_fn: _| {
                                group_renderer_fn(
                                    group_data.clone(),
                                    group_callbacks.clone(),
                                    group_content,
                                )
                            })
                    } else {
                        group_content
                    };
                    all_field_views.push(group_view);
                }
                for field_data in &container_data.ungrouped_fields {
                    all_field_views
                        .push(
                            field_renderer
                                .with(|field_renderer_fn| {
                                    field_renderer_fn(field_data.clone(), field_callbacks.clone())
                                }),
                        );
                }
                let fields_view = // Apply group renderer if provided

                // Render ungrouped fields

                view! { {all_field_views.into_iter().collect::<Vec<_>>()} }
                    .into_any();
                if let Some(container_renderer) = &container_renderer {
                    container_renderer
                        .with(|f| { f(container_data, container_callbacks.clone(), fields_view) })
                } else {
                    fields_view
                }
            }}
        </div>
    }
}
