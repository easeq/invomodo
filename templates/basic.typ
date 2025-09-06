// This invoice template is designed to accept all data from a single
// JSON-like object, making it highly modular and easy to reuse.

// --- 1. CONFIGURATION DATA OBJECT ---
// All invoice data is now in this single object. Simply update this block
// to create a new invoice.
#let invoice-data = (
  meta: (
    title: "INVOICE",
    invoice-number: "SDI-2024-001",
    date: "2024-08-16",
    due-date: "2024-08-30",
  ),
  sender: (
    name: "Tech Solutions, LLC",
    address: "123 Silicon Alley",
    city: "San Jose, CA 95113",
    country: "USA",
    email: "billing@techsolutions.com",
    phone: "+1 (555) 789-0123",
  ),
  client: (
    name: "Startup Innovations Co.",
    address: "456 Innovation Drive",
    city: "Palo Alto, CA 94301",
    country: "USA",
    email: "accounts@startupinnovations.com",
  ),
  project: (
    name: "Mobile App Development",
  ),
  financial: (
    currency: "$",
    tax-rate: 0.08,
    payment-instructions: "Payment is due upon receipt. We accept bank transfer or PayPal.",
    payment-details: [
      Bank: Innovation Bank \
      Account No.: 555566667777 \
      Routing No.: 987654321
    ],
  ),
  items: (
    ("Frontend Development (React)", 45, 120, 5400),
    ("Backend API Development (Node.js)", 30, 150, 4500),
    ("Database Design & Integration", 15, 150, 2250),
    ("Frontend Development (React)", 45, 120, 5400),
    ("Frontend Development (React)", 45, 120, 5400),
    ("Frontend Development (React)", 45, 120, 5400),
    ("Frontend Development (React)", 45, 120, 5400),
    ("Frontend Development (React)", 45, 120, 5400),
    ("Frontend Development (React)", 45, 120, 5400),
    ("Frontend Development (React)", 45, 120, 5400),
    ("Backend API Development (Node.js)", 30, 150, 4500),
    ("Database Design & Integration", 15, 150, 2250),
    ("Backend API Development (Node.js)", 30, 150, 4500),
    ("Database Design & Integration", 15, 150, 2250),
    ("Backend API Development (Node.js)", 30, 150, 4500),
    ("Database Design & Integration", 15, 150, 2250),
    ("Backend API Development (Node.js)", 30, 150, 4500),
    ("Database Design & Integration", 15, 150, 2250),
    ("Backend API Development (Node.js)", 30, 150, 4500),
    ("Database Design & Integration", 15, 150, 2250),
    ("Backend API Development (Node.js)", 30, 150, 4500),
    ("Database Design & Integration", 15, 150, 2250),
    ("Backend API Development (Node.js)", 30, 150, 4500),
    ("Database Design & Integration", 15, 150, 2250),
  ),
)

// --- 2. LAYOUT & STYLING ---
#set page(
  paper: "a4",
  margin: (top: 1in, bottom: 1in, left: 1in, right: 1in),
)

#set text(
  font: "Arial",
  size: 11pt,
  fill: rgb("#333333")
)

// --- 3. TEMPLATE STRUCTURE ---

// Header
#align(right)[
  #text(30pt, weight: "bold", fill: rgb("#004080"))[#invoice-data.meta.title]
]

#v(1em)

#grid(
  columns: (1fr, 1fr),
  column-gutter: 1em,
  row-gutter: 1em,
  align: (left, right),
  [
    #text(12pt, weight: "bold")[#invoice-data.sender.name] \
    #invoice-data.sender.address \
    #invoice-data.sender.city \
    #invoice-data.sender.country \
    #link("mailto:" + invoice-data.sender.email)[#invoice-data.sender.email] \
    #invoice-data.sender.phone
  ],
  [
    #text(weight: "bold")[Invoice No.]: #invoice-data.meta.invoice-number \
    #text(weight: "bold")[Date]: #invoice-data.meta.date \
    #text(weight: "bold")[Due Date]: #invoice-data.meta.due-date
  ],
)

#line(length: 100%, stroke: 1pt + rgb("#CCCCCC"))
#v(1.5em)

#grid(
  columns: (1fr, 1fr),
  column-gutter: 1em,
  row-gutter: 1em,
  align: (left, right),
  [
    #text(12pt, weight: "bold", fill: rgb("#004080"))[BILL TO] \ 
    #text(11pt, weight: "bold")[#invoice-data.client.name] \
    #invoice-data.client.address \
    #invoice-data.client.city \
    #invoice-data.client.country \ 
    #link("mailto:" + invoice-data.client.email)[#invoice-data.client.email]
  ],
  [
    #text(10pt)[Project: #invoice-data.project.name]
  ],
)

// Define the item data
#let item-table-data = invoice-data.items

// Table with header and styled body
#table(
  columns: (3fr, 1fr, 1.5fr, 1.5fr),
  align: (left, right, right, right),
  inset: 10pt,
  fill: (col, row) => if calc.rem(row, 2) == 1 { rgb("F0F8FF") } else { rgb("FFFFFF") },
  stroke: (col, row) => {
    let stroke = rgb("004080")
    if col == 0 and row != 0 {
      return (
        left: 1.5pt + stroke,
      )
    }
    if col == item-table-data.first().len() - 1 and row != 0 {
      return (
        right: 1.5pt + stroke,
      )
    }
    if row == 0 {
      return 1.5pt + stroke
    } else {
      return none
    }
  },

  // Header row
  table.header(
    repeat: true,
    [*Description*],
    [*Hours*],
    [*Rate/hr*],
    table.cell(align: right)[*Total*]
  ),

  // Body rows
  ..for (row, (desc, hours, rate, total)) in item-table-data.enumerate() {
    ([#desc], [#hours], [#rate], [#total])
  },
  
  // Totals Section
  let subtotal = item-table-data.map(row => row.at(3)).sum(),
  let tax-amount = subtotal * invoice-data.financial.tax-rate,
  let total = subtotal + tax-amount,
  let tax-rate-percent = invoice-data.financial.tax-rate * 100,

  let footer_top_stroke = (
    top: 1.5pt + rgb("004080"),
  ),
  let footer_top_left_stroke = (
    top: 1.5pt + rgb("004080"),
    left: 1.5pt + rgb("004080"),
  ),
  let footer_left_stroke = (
    left: 1.5pt + rgb("004080"),
  ),
  let footer_bottom_stroke = (
    bottom: 1.5pt + rgb("004080"),
  ),
  let footer_bottom_left_stroke = (
    bottom: 1.5pt + rgb("004080"),
    left: 1.5pt + rgb("004080"),
  ),
  
  // Footer row
  table.footer(
    repeat: true,

    // Subtotal
    table.cell(stroke: footer_top_stroke)[],
    table.cell(stroke: footer_top_stroke)[],
    table.cell(align:right, stroke: footer_top_left_stroke)[*Subtotal:*],
    table.cell(align:right, stroke: footer_top_stroke)[#invoice-data.financial.currency #subtotal.2],

    // Taxes
    [],
    [],
    table.cell(align:right, stroke: footer_left_stroke)[*Tax \(#tax-rate-percent%\):*],
    table.cell(align:right)[#invoice-data.financial.currency #tax-amount.2],

    // Total Due
    table.cell(stroke: footer_bottom_stroke)[],
    table.cell(stroke: footer_bottom_stroke)[],
    table.cell(align:right, stroke: footer_bottom_left_stroke)[*Total Due:*],
    table.cell(align:right, stroke: footer_bottom_stroke)[#invoice-data.financial.currency #total.2],
  ),
)

#v(3em)

// Footer
#line(length: 100%, stroke: 1pt + rgb("#CCCCCC"))
#v(1em)

#box(align(left)[
  #text(8pt)[
    *Payment Instructions:* #invoice-data.financial.payment-instructions #linebreak()
    #invoice-data.financial.payment-details
  ]
])
