use crate::components::ui::{
    AutocompleteConfig, AutocompleteGroup, AutocompleteItem, GroupedAutocomplete, StaticDataSource,
};

use leptos::prelude::*;
use phosphor_leptos::{Icon, MAGNIFYING_GLASS, X};

#[derive(Clone, Debug, PartialEq)]
pub struct LineChargeItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub rate: Option<f64>,
}

impl AutocompleteItem for LineChargeItem {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.id.clone()
    }

    fn search_text(&self) -> String {
        format!("{} {}", self.name, self.description)
    }

    fn display_text(&self) -> String {
        if let Some(rate) = self.rate {
            format!("{} ({}%)", self.name, rate)
        } else {
            self.name.clone()
        }
    }
}

/// Example component using the autocomplete
#[component]
pub fn LineItemCharges() -> impl IntoView {
    // Sample data
    let tax_items = vec![
        LineChargeItem {
            id: "vat-10".to_string(),
            name: "VAT".to_string(),
            description: "Value Added Tax".to_string(),
            rate: Some(10.0),
        },
        LineChargeItem {
            id: "gst-18".to_string(),
            name: "GST".to_string(),
            description: "Goods and Services Tax".to_string(),
            rate: Some(18.0),
        },
        LineChargeItem {
            id: "sales-tax".to_string(),
            name: "Sales Tax".to_string(),
            description: "State Sales Tax".to_string(),
            rate: Some(8.5),
        },
    ];

    let discount_items = vec![
        LineChargeItem {
            id: "early-bird".to_string(),
            name: "Early Bird Discount".to_string(),
            description: "Early booking discount".to_string(),
            rate: Some(15.0),
        },
        LineChargeItem {
            id: "loyalty".to_string(),
            name: "Loyalty Discount".to_string(),
            description: "Customer loyalty discount".to_string(),
            rate: Some(10.0),
        },
    ];

    let other_charges = vec![
        LineChargeItem {
            id: "shipping".to_string(),
            name: "Shipping Fee".to_string(),
            description: "Standard shipping charges".to_string(),
            rate: None,
        },
        LineChargeItem {
            id: "handling".to_string(),
            name: "Handling Fee".to_string(),
            description: "Processing and handling".to_string(),
            rate: None,
        },
    ];

    let groups = vec![
        AutocompleteGroup {
            id: "taxes".to_string(),
            name: "Taxes".to_string(),
            items: tax_items,
        },
        AutocompleteGroup {
            id: "discounts".to_string(),
            name: "Discounts".to_string(),
            items: discount_items,
        },
        AutocompleteGroup {
            id: "other".to_string(),
            name: "Other Charges".to_string(),
            items: other_charges,
        },
    ];

    let data_source = StaticDataSource::new(groups, false);
    let config = AutocompleteConfig {
        min_query_length: 1,
        max_results_per_group: Some(5),
        debounce_ms: 200.0,
        case_sensitive: false,
    };

    view! {
        <GroupedAutocomplete
            data_source=data_source
            config=config
            render=move |props, actions| {
                view! {
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1" for="charges">
                            "Taxes, Discounts, Other Charges"
                        </label>
                        <div class="relative">
                            <input
                                class="form-input"
                                id="charges"
                                placeholder="Search for taxes, discounts, or other charges"
                                type="text"
                                prop:value=props.query
                                on:input=move |ev| {
                                    actions.set_query.set(event_target_value(&ev));
                                }
                                on:focus=move |_| actions.set_open.set(true)
                                on:keydown=move |ev| actions.handle_key_down.run(ev)
                            />
                            <div class="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none">
                                <span class="material-symbols-outlined text-gray-400">
                                    <Icon icon=MAGNIFYING_GLASS />
                                </span>
                            </div>

                            // Dropdown results
                            <Show when=move || {
                                props.is_open.get() && !props.results.get().is_empty()
                            }>
                                <div class="absolute z-10 mt-1 w-full bg-white shadow-lg max-h-60 rounded-md py-1 text-base ring-1 ring-black ring-opacity-5 overflow-auto focus:outline-none">
                                    <For
                                        each=move || props.results.get().into_iter().enumerate()
                                        key=|(_, group)| group.id.clone()
                                        children=move |(group_idx, group)| {
                                            view! {
                                                <div>
                                                    <div class="px-4 py-2 text-xs font-semibold text-gray-500 uppercase tracking-wide bg-gray-50">
                                                        {group.name.clone()}
                                                    </div>
                                                    <For
                                                        each={
                                                            let group_items = group.items.clone();
                                                            move || group_items.clone().into_iter().enumerate()
                                                        }
                                                        key=|(_, item)| item.id()
                                                        children=move |(item_idx, item)| {
                                                            let is_highlighted = props.highlighted_index
                                                                == Some((group_idx, item_idx));
                                                            let item_clone = item.clone();

                                                            view! {
                                                                <div
                                                                    class=format!(
                                                                        "px-4 py-2 cursor-pointer hover:bg-gray-100 {}",
                                                                        if is_highlighted {
                                                                            "bg-indigo-100 text-indigo-900"
                                                                        } else {
                                                                            "text-gray-900"
                                                                        },
                                                                    )
                                                                    on:click=move |_| {
                                                                        actions.select_item.run(item_clone.clone())
                                                                    }
                                                                >
                                                                    <div class="font-medium">{item.display_text()}</div>
                                                                    <div class="text-sm text-gray-500">
                                                                        {item.description.clone()}
                                                                    </div>
                                                                </div>
                                                            }
                                                        }
                                                    />
                                                </div>
                                            }
                                        }
                                    />
                                </div>
                            </Show>

                            // Loading indicator
                            <Show when=move || props.is_loading.get()>
                                <div class="absolute right-3 top-1/2 transform -translate-y-1/2">
                                    <div class="animate-spin h-4 w-4 border-2 border-indigo-500 border-t-transparent rounded-full"></div>
                                </div>
                            </Show>
                        </div>

                        // Selected items (tags)
                        <div class="mt-2 flex flex-wrap gap-2">
                            <For
                                each=move || props.selected_items.get()
                                key=|item| item.id()
                                children=move |item| {
                                    let item_id = item.id();

                                    view! {
                                        <span class="bg-indigo-100 text-indigo-800 text-sm font-medium px-3 py-1 rounded-full flex items-center">
                                            {item.display_text()}
                                            <button
                                                class="ml-2 text-indigo-800 hover:text-indigo-900"
                                                on:click=move |_| actions.remove_item.run(item_id.clone())
                                            >
                                                <span class="material-symbols-outlined text-sm">
                                                    <Icon icon=X />
                                                </span>
                                            </button>
                                        </span>
                                    }
                                }
                            />
                        </div>
                    </div>
                }
                    .into_any()
            }
        />
    }
}
