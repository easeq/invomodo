use super::*;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct InvoiceBuilderState {
    pub taxes: RwSignal<Vec<TaxItem>>,
    pub discounts: RwSignal<Vec<DiscountItem>>,
    pub charges: RwSignal<Vec<ChargeItem>>,
    pub custom_fields: RwSignal<Vec<CustomFieldItem>>,
    pub line_items: RwSignal<Vec<LineItem>>,
    // pub global_discounts: Vec<String>, // Applied discount IDs
    // pub global_charges: Vec<String>,   // Applied charge IDs
    // pub last_updated: String,
}

impl InvoiceBuilderState {
    pub fn new() -> Self {
        Self {
            taxes: RwSignal::new(vec![
                TaxItem {
                    id: uuid::Uuid::new_v4().to_string(),
                    name: "VAT".to_string(),
                    tax_type: TaxType::Percentage,
                    rate: 10.00,
                },
                TaxItem {
                    id: uuid::Uuid::new_v4().to_string(),
                    name: "Service Tax".to_string(),
                    tax_type: TaxType::Percentage,
                    rate: 5.00,
                },
                TaxItem {
                    id: uuid::Uuid::new_v4().to_string(),
                    name: "Flat Fee Tax".to_string(),
                    tax_type: TaxType::FixedAmount,
                    rate: 25.00,
                },
            ]),
            discounts: RwSignal::new(vec![
                DiscountItem {
                    id: uuid::Uuid::new_v4().to_string(),
                    name: "Early Bird Discount".to_string(),
                    description: "10% discount for early payment".to_string(),
                    discount_type: DiscountType::Percentage,
                    value: 10.0,
                    scope: DiscountScope::GlobalInvoice,
                    is_default: true,
                },
                DiscountItem {
                    id: uuid::Uuid::new_v4().to_string(),
                    name: "Bulk Order Discount".to_string(),
                    description: "$50 off for bulk orders".to_string(),
                    discount_type: DiscountType::FixedAmount,
                    value: 50.0,
                    scope: DiscountScope::LineItem,
                    is_default: false,
                },
                DiscountItem {
                    id: uuid::Uuid::new_v4().to_string(),
                    name: "Loyalty Discount".to_string(),
                    description: "20% off for loyal customers".to_string(),
                    discount_type: DiscountType::Percentage,
                    value: 20.0,
                    scope: DiscountScope::GlobalInvoice,
                    is_default: false,
                },
            ]),
            charges: RwSignal::new(vec![
                ChargeItem {
                    id: uuid::Uuid::new_v4().to_string(),
                    name: "Shipping Fee".to_string(),
                    description: "Standard shipping charge".to_string(),
                    amount: 5.00,
                    scope: ChargeScope::GlobalInvoice,
                    is_default: true,
                },
                ChargeItem {
                    id: uuid::Uuid::new_v4().to_string(),
                    name: "Handling Fee".to_string(),
                    description: "Fee for handling fragile items".to_string(),
                    amount: 10.00,
                    scope: ChargeScope::LineItem,
                    is_default: false,
                },
                ChargeItem {
                    id: uuid::Uuid::new_v4().to_string(),
                    name: "Rush Order Fee".to_string(),
                    description: "Fee for expedited order processing".to_string(),
                    amount: 25.00,
                    scope: ChargeScope::GlobalInvoice,
                    is_default: false,
                },
            ]),
            custom_fields: RwSignal::new(vec![
                CustomFieldItem {
                    id: "1".to_string(),
                    name: "Due Date".to_string(),
                    field_type: FieldType::Date,
                    category: FieldCategory::GlobalInvoice,
                    default_value: "30 days".to_string(),
                    required: true,
                    is_default: true,
                },
                CustomFieldItem {
                    id: "2".to_string(),
                    name: "PO Number".to_string(),
                    field_type: FieldType::Number,
                    category: FieldCategory::GlobalInvoice,
                    default_value: String::new(),
                    required: false,
                    is_default: false,
                },
                CustomFieldItem {
                    id: "3".to_string(),
                    name: "Client ID".to_string(),
                    field_type: FieldType::Text,
                    category: FieldCategory::ClientAddress,
                    default_value: "CUST-".to_string(),
                    required: false,
                    is_default: false,
                },
            ]),
            line_items: RwSignal::new(vec![]),
        }
    }
}

// Context provider
#[derive(Clone)]
pub struct InvoiceBuilderContext {
    pub state: InvoiceBuilderState,
}

pub fn provide_invoice_builder_context() -> InvoiceBuilderContext {
    let state = InvoiceBuilderState::new();
    InvoiceBuilderContext { state }
}

// Hook to use the context
pub fn use_invoice_builder() -> InvoiceBuilderContext {
    use_context::<InvoiceBuilderContext>().expect("InvoiceBuilderContext must be provided")
}
