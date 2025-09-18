use super::*;
use leptos::prelude::*;
use std::collections::HashMap;

#[component]
pub fn Builder(
    line_items: RwSignal<Vec<LineItem>>,
    taxes: ReadSignal<Vec<TaxItem>>,
    discounts: ReadSignal<Vec<DiscountItem>>,
    charges: ReadSignal<Vec<ChargeItem>>,
    custom_fields: ReadSignal<Vec<FieldItem>>,
    extra_info: RwSignal<HashMap<String, FieldItemValue>>,
    invoice_info: RwSignal<HashMap<String, FieldItemValue>>,
    biller_info: RwSignal<HashMap<String, FieldItemValue>>,
    client_info: RwSignal<HashMap<String, FieldItemValue>>,
) -> impl IntoView {
    view! {
        <div class="w-full px-4 py-6">
            <div class="grid grid-cols-1 lg:grid-cols-[3fr,1fr] gap-8 w-full">
                <div class="space-y-6">
                    <InvoiceFields fields=custom_fields form_values=invoice_info />

                    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                        <CategoryFieldsCard
                            fields=custom_fields
                            form_values=biller_info
                            field_category=FieldCategory::Biller
                        />
                        <CategoryFieldsCard
                            fields=custom_fields
                            form_values=client_info
                            field_category=FieldCategory::Client
                        />
                    </div>

                    <LineItems
                        state=line_items
                        taxes=taxes
                        discounts=discounts
                        charges=charges
                        custom_fields=custom_fields
                    />

                </div>

                <div class="space-y-6">
                    // <div class="space-y-6">
                    // <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    // <div>
                    // <label class="form-label" for="template">
                    // "Template"
                    // </label>
                    // <select class="form-select" id="template" name="template">
                    // <option>"Default Template"</option>
                    // <option>"Modern"</option>
                    // <option>"Classic"</option>
                    // </select>
                    // </div>
                    // <div>
                    // <label class="form-label" for="locale">
                    // "Locale"
                    // </label>
                    // <select class="form-select" id="locale" name="locale">
                    // <option>"English (US)"</option>
                    // <option>"Français (FR)"</option>
                    // <option>"Español (ES)"</option>
                    // </select>
                    // </div>
                    // </div>
                    // </div>
                    <TotalsCharges fields=custom_fields form_values=client_info />
                    <TotalsCard
                        line_items=line_items
                        taxes=taxes
                        discounts=discounts
                        charges=charges
                    />
                    <ExtraInfoFields fields=custom_fields form_values=extra_info />
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn TotalsCharges(
    fields: ReadSignal<Vec<FieldItem>>,
    form_values: RwSignal<HashMap<String, FieldItemValue>>,
) -> impl IntoView {
    let config = FieldsConfig {
        render_mode: RenderMode::Card {
            title: Some("Totals Charges".to_string()),
            collapsible: true,
        },
        layout: LayoutConfig {
            container_class: "space-y-6 mb-6".to_string(),
            // grid_class: "grid grid-cols-1 lg:grid-cols-3 gap-6".to_string(),
            ..Default::default()
        },
        ..Default::default()
    };

    let filter = Box::new(|field: &FieldItem| field.category == FieldCategory::Invoice)
        as Box<dyn Fn(&FieldItem) -> bool + Send + Sync>;

    view! { <FieldsRenderer fields=fields form_values=form_values config=config filter=filter /> }
}

#[component]
pub fn InvoiceFields(
    fields: ReadSignal<Vec<FieldItem>>,
    form_values: RwSignal<HashMap<String, FieldItemValue>>,
) -> impl IntoView {
    let config = FieldsConfig {
        render_mode: RenderMode::Card {
            title: Some("General".to_string()),
            collapsible: true,
        },
        layout: LayoutConfig {
            container_class: "space-y-6 mb-6".to_string(),
            // grid_class: "grid grid-cols-1 lg:grid-cols-3 gap-6".to_string(),
            ..Default::default()
        },
        ..Default::default()
    };

    let filter = Box::new(|field: &FieldItem| field.category == FieldCategory::Invoice)
        as Box<dyn Fn(&FieldItem) -> bool + Send + Sync>;

    view! { <FieldsRenderer fields=fields form_values=form_values config=config filter=filter /> }
}

#[component]
pub fn ExtraInfoFields(
    fields: ReadSignal<Vec<FieldItem>>,
    form_values: RwSignal<HashMap<String, FieldItemValue>>,
) -> impl IntoView {
    let config = FieldsConfig {
        render_mode: RenderMode::Card {
            title: Some("Additional Information".to_string()),
            collapsible: true,
        },
        layout: LayoutConfig {
            container_class: "space-y-6 mb-6".to_string(),
            // grid_class: "grid grid-cols-1 lg:grid-cols-3 gap-6".to_string(),
            ..Default::default()
        },
        ..Default::default()
    };

    let filter = Box::new(|field: &FieldItem| field.category == FieldCategory::ExtraInfo)
        as Box<dyn Fn(&FieldItem) -> bool + Send + Sync>;

    view! { <FieldsRenderer fields=fields form_values=form_values config=config filter=filter /> }
}

#[component]
pub fn CategoryFieldsCard(
    fields: ReadSignal<Vec<FieldItem>>,
    form_values: RwSignal<HashMap<String, FieldItemValue>>,
    field_category: FieldCategory,
) -> impl IntoView {
    let config = FieldsConfig {
        render_mode: RenderMode::Card {
            title: Some("Billing Information".to_string()),
            collapsible: true,
        },
        layout: LayoutConfig {
            container_class: "space-y-6 mb-6".to_string(),
            // grid_class: "grid grid-cols-1 lg:grid-cols-3 gap-6".to_string(),
            ..Default::default()
        },
        ..Default::default()
    };

    let filter = Box::new({
        let field_category = field_category.clone();
        move |field: &FieldItem| field.category == field_category.clone()
    }) as Box<dyn Fn(&FieldItem) -> bool + Send + Sync>;

    view! { <FieldsRenderer fields=fields form_values=form_values config=config filter=filter /> }
}
