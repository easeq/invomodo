use super::*;
use leptos::prelude::*;

#[component]
pub fn TotalsCard(
    line_items: RwSignal<Vec<LineItem>>,
    taxes: ReadSignal<Vec<TaxItem>>,
    discounts: ReadSignal<Vec<DiscountItem>>,
    charges: ReadSignal<Vec<ChargeItem>>,
) -> impl IntoView {
    let totals = Memo::new(move |_| {
        let items = line_items.get();

        // Subtotal calculation
        let mut subtotal = 0.0;
        for item in &items {
            subtotal += item.unit_price * item.quantity;
        }

        // Line-item discounts
        let mut line_item_discounts = 0.0;
        for item in &items {
            for discount in &item.discounts {
                match discount.discount_type {
                    DiscountType::FixedAmount => {
                        line_item_discounts += discount.value;
                    }
                    DiscountType::Percentage => {
                        line_item_discounts +=
                            (item.unit_price * item.quantity) * (discount.value / 100.0);
                    }
                }
            }
        }

        // Global discounts
        let mut global_discounts = 0.0;
        for discount in discounts.get() {
            if discount.scope == DiscountScope::GlobalInvoice {
                match discount.discount_type {
                    DiscountType::FixedAmount => {
                        global_discounts += discount.value;
                    }
                    DiscountType::Percentage => {
                        global_discounts += subtotal * (discount.value / 100.0);
                    }
                }
            }
        }

        let total_discounts = line_item_discounts + global_discounts;

        // Charges
        let mut line_item_charges = 0.0;
        for item in &items {
            for charge in &item.charges {
                line_item_charges += charge.amount;
            }
        }

        let mut global_charges = 0.0;
        for charge in charges.get() {
            if charge.scope == ChargeScope::GlobalInvoice {
                global_charges += charge.amount;
            }
        }

        let total_charges = line_item_charges + global_charges;

        // Tax calculation
        let mut tax_total = 0.0;
        for item in &items {
            let item_total = item.unit_price * item.quantity;
            let item_discount = item.discounts.iter().fold(0.0, |acc, d| {
                acc + match d.discount_type {
                    DiscountType::FixedAmount => d.value,
                    DiscountType::Percentage => item_total * (d.value / 100.0),
                }
            });
            let taxable_amount = item_total - item_discount;

            for tax in &item.taxes {
                tax_total += taxable_amount * (tax.rate / 100.0);
            }
        }

        let final_total = subtotal - total_discounts + total_charges + tax_total;

        (
            subtotal,
            total_discounts,
            total_charges,
            tax_total,
            final_total,
        )
    });

    view! {
        <div class="bg-white shadow rounded-lg p-4">
            <h2 class="text-lg font-semibold mb-4">"Totals"</h2>

            <div class="flex justify-between text-sm mb-1">
                <span>"Subtotal"</span>
                <span>{format!("{:.2}", totals.get().0)}</span>
            </div>

            <div class="flex justify-between text-sm mb-1">
                <span>"Discounts"</span>
                <span class="text-red-600">- {format!("{:.2}", totals.get().1)}</span>
            </div>

            <div class="flex justify-between text-sm mb-1">
                <span>"Charges"</span>
                <span class="text-green-600">+ {format!("{:.2}", totals.get().2)}</span>
            </div>

            <div class="flex justify-between text-sm mb-1">
                <span>"Taxes"</span>
                <span class="text-yellow-600">+ {format!("{:.2}", totals.get().3)}</span>
            </div>

            <div class="border-t mt-2 pt-2 flex justify-between font-semibold text-base">
                <span>"Total"</span>
                <span>{format!("{:.2}", totals.get().4)}</span>
            </div>
        </div>
    }
}
