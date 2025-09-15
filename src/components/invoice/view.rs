use leptos::prelude::*;

use super::*;
use crate::components::ui::{Tab, TabPanel, TabsProvider};

#[component]
pub fn InternalTab(name: String, label: String) -> impl IntoView {
    view! {
        <Tab
            name=name.clone()
            render=move |is_sel, select| {
                view! {
                    <button
                        class=move || {
                            let mut classes = "px-4 py-3 text-sm font-semibold border-b-2 "
                                .to_string();
                            if is_sel.get() {
                                classes.push_str("border-indigo-600 text-gray-900");
                            } else {
                                classes
                                    .push_str(
                                        "border-transparent text-gray-600 hover:text-gray-900",
                                    );
                            }
                            classes
                        }
                        on:click=move |_| select()
                    >
                        {label.clone()}
                    </button>
                }
            }
        />
    }
}

#[component]
pub fn InvoiceBuilder() -> impl IntoView {
    let invoice_builder_state = InvoiceBuilderState::new();

    view! {
        <div class="lg:col-span-2 space-y-6">
            <TabsProvider default="builder".to_string()>
                <div class="bg-white rounded-md shadow-sm border border-gray-200">
                    <div class="border-b border-gray-200">
                        <div class="flex px-4">
                            <InternalTab name="builder".to_string() label="Builder".to_string() />
                            <InternalTab name="taxes".to_string() label="Taxes".to_string() />
                            <InternalTab
                                name="discounts".to_string()
                                label="Discounts".to_string()
                            />
                            <InternalTab
                                name="other_charges".to_string()
                                label="Other Charges".to_string()
                            />
                            <InternalTab
                                name="custom_fields".to_string()
                                label="Custom Fields".to_string()
                            />
                            <InternalTab name="preview".to_string() label="Preview".to_string() />
                        </div>
                    </div>
                    <TabPanel name="builder".to_string()>
                        <Builder
                            line_items=invoice_builder_state.line_items
                            taxes=invoice_builder_state.taxes.read_only()
                            discounts=invoice_builder_state.discounts.read_only()
                            charges=invoice_builder_state.charges.read_only()
                            custom_fields=invoice_builder_state.custom_fields.read_only()
                        />
                    </TabPanel>
                    <TabPanel name="taxes".to_string()>
                        <Taxes state=invoice_builder_state.taxes />
                    </TabPanel>
                    <TabPanel name="discounts".to_string()>
                        <Discounts state=invoice_builder_state.discounts />
                    </TabPanel>
                    <TabPanel name="other_charges".to_string()>
                        <OtherCharges state=invoice_builder_state.charges />
                    </TabPanel>
                    <TabPanel name="custom_fields".to_string()>
                        <Fields state=invoice_builder_state.custom_fields />
                    </TabPanel>
                    <TabPanel name="preview".to_string()>
                        <section class="p-6 text-gray-700">preview</section>
                    </TabPanel>
                </div>
            </TabsProvider>
        </div>
    }
}
