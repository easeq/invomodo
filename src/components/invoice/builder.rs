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
    biller_info: RwSignal<HashMap<String, FieldItemValue>>,
    client_info: RwSignal<HashMap<String, FieldItemValue>>,
) -> impl IntoView {
    view! {
        <div class="container mx-auto p-4">
            <div class="bg-white p-6 rounded-lg shadow-sm border border-gray-200">
                <h3 class="text-lg font-medium leading-6 text-gray-900 mb-6">"Invoice Details"</h3>
                <div class="space-y-6">
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <div>
                            <label class="form-label" for="template">
                                "Template"
                            </label>
                            <select class="form-select" id="template" name="template">
                                <option>"Default Template"</option>
                                <option>"Modern"</option>
                                <option>"Classic"</option>
                            </select>
                        </div>
                        <div>
                            <label class="form-label" for="locale">
                                "Locale"
                            </label>
                            <select class="form-select" id="locale" name="locale">
                                <option>"English (US)"</option>
                                <option>"Français (FR)"</option>
                                <option>"Español (ES)"</option>
                            </select>
                        </div>
                    </div>
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                        <div>
                            <label class="form-label" for="invoice-number">
                                "Invoice Number"
                            </label>
                            <input
                                class="form-input"
                                id="invoice-number"
                                name="invoice-number"
                                type="text"
                                value="INV-2024-001"
                            />
                        </div>
                        <div>
                            <label class="form-label" for="issue-date">
                                "Issue Date"
                            </label>
                            <input
                                class="form-input"
                                id="issue-date"
                                name="issue-date"
                                type="date"
                                value="2024-07-28"
                            />
                        </div>
                        <div>
                            <label class="form-label" for="due-date">
                                "Due Date"
                            </label>
                            <input
                                class="form-input"
                                id="due-date"
                                name="due-date"
                                type="date"
                                value="2024-08-27"
                            />
                        </div>
                    </div>
                    <div>
                        <label class="form-label" for="reference-number">
                            "Reference Number"
                        </label>
                        <input
                            class="form-input"
                            id="reference-number"
                            name="reference-number"
                            placeholder="Optional project code or PO number"
                            type="text"
                        />
                    </div>
                    <div class="mt-6 flex justify-end">
                        <button
                            class="inline-flex justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
                            type="submit"
                        >
                            "Save Details"
                        </button>
                    </div>
                </div>
            </div>
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
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
            <ExtraInfoFields fields=custom_fields form_values=extra_info />
        </div>
    }
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
            container_class: "mb-6".to_string(),
            grid_class: "grid grid-cols-1 lg:grid-cols-3 gap-6".to_string(),
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
            container_class: "mb-6".to_string(),
            grid_class: "grid grid-cols-1 lg:grid-cols-3 gap-6".to_string(),
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
