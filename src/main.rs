use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[route("/records")]
        Records {},
        #[route("/add-record")]
        AddRecord {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

// Define a simple structure for Farmer Records
#[derive(Debug, Clone, PartialEq)]
struct Record {
    id: usize,
    farmer_name: String,
    item_type: String, // e.g. Wheat Bran, Chaff/Chokar
    quantity_kg: f32,
    amount: f32,
    date: String,
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Provide a global state for records
    use_context_provider(|| {
        Signal::new(vec![
            Record {
                id: 1,
                farmer_name: "Ramesh Singh".to_string(),
                item_type: "Wheat Bran".to_string(),
                quantity_kg: 50.0,
                amount: 1500.0,
                date: "2026-09-29".to_string(),
            },
            Record {
                id: 2,
                farmer_name: "Suresh Kumar".to_string(),
                item_type: "Chaff (Chokar)".to_string(),
                quantity_kg: 30.0,
                amount: 900.0,
                date: "2026-09-28".to_string(),
            },
        ])
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
fn Navbar() -> Element {
    rsx! {
        nav {
            class: "fixed w-full z-50 glass-panel border-b border-white/10 px-6 py-4 flex justify-between items-center cursor-pointer",
            div {
                class: "flex items-center gap-2 cursor-pointer",
                Link {
                    to: Route::Home {},
                    class: "text-2xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-emerald-400 to-teal-300 cursor-pointer",
                    "Satyam Dairy"
                }
            }
            div {
                class: "flex gap-6 items-center",
                Link {
                    to: Route::Home {},
                    class: "text-gray-300 hover:text-emerald-400 transition-colors font-medium cursor-pointer",
                    "Dashboard"
                }
                Link {
                    to: Route::Records {},
                    class: "text-gray-300 hover:text-emerald-400 transition-colors font-medium cursor-pointer",
                    "Farmer Records"
                }
                Link {
                    to: Route::AddRecord {},
                    class: "btn-3d px-4 py-2 font-semibold cursor-pointer",
                    "+ Add Record"
                }
            }
        }
        div {
            class: "pt-24 min-h-screen",
            Outlet::<Route> {}
        }
    }
}

#[component]
fn Home() -> Element {
    let records = use_context::<Signal<Vec<Record>>>();

    let total_kg = records.read().iter().map(|r| r.quantity_kg).sum::<f32>();
    let total_amount = records.read().iter().map(|r| r.amount).sum::<f32>();

    rsx! {
        div {
            class: "max-w-7xl mx-auto px-6 page-transition",

            h1 { class: "text-4xl font-extrabold mb-8 text-white tracking-tight cursor-default", "Dashboard Overview" }

            div {
                class: "grid grid-cols-1 md:grid-cols-3 gap-6 mb-12 cursor-default",

                // Stat Card 1
                div {
                    class: "glass-panel p-6 rounded-2xl cursor-pointer hover:bg-white/10 transition duration-300 transform hover:-translate-y-1",
                    h3 { class: "text-gray-400 text-sm font-medium uppercase tracking-wider mb-2", "Total Farmers" }
                    p { class: "text-4xl font-bold text-emerald-400", "{records.read().len()}" }
                }

                // Stat Card 2
                div {
                    class: "glass-panel p-6 rounded-2xl cursor-pointer hover:bg-white/10 transition duration-300 transform hover:-translate-y-1",
                    h3 { class: "text-gray-400 text-sm font-medium uppercase tracking-wider mb-2", "Total Feed Supplied" }
                    p { class: "text-4xl font-bold text-emerald-400", "{total_kg} kg" }
                }

                // Stat Card 3
                div {
                    class: "glass-panel p-6 rounded-2xl cursor-pointer hover:bg-white/10 transition duration-300 transform hover:-translate-y-1",
                    h3 { class: "text-gray-400 text-sm font-medium uppercase tracking-wider mb-2", "Total Amount" }
                    p { class: "text-4xl font-bold text-emerald-400", "₹{total_amount}" }
                }
            }

            div {
                class: "glass-panel p-8 rounded-2xl cursor-pointer hover:shadow-2xl transition duration-500",
                h2 { class: "text-2xl font-bold mb-4", "Recent Transactions" }
                p { class: "text-gray-300 mb-6", "Monitor your recent wheat bran and chokar distributions." }
                Link {
                    to: Route::Records {},
                    class: "text-emerald-400 hover:text-emerald-300 font-medium flex items-center gap-2 cursor-pointer",
                    "View all records →"
                }
            }
        }
    }
}

#[component]
fn Records() -> Element {
    let mut records = use_context::<Signal<Vec<Record>>>();

    rsx! {
        div {
            class: "max-w-7xl mx-auto px-6 page-transition",

            div { class: "flex justify-between items-center mb-8",
                h1 { class: "text-4xl font-extrabold text-white tracking-tight cursor-default", "Farmer Records" }
                Link {
                    to: Route::AddRecord {},
                    class: "btn-3d px-5 py-2.5 font-semibold cursor-pointer",
                    "+ New Record"
                }
            }

            div {
                class: "glass-panel rounded-2xl overflow-hidden cursor-default",
                table {
                    class: "w-full text-left border-collapse",
                    thead {
                        class: "bg-black/40 text-gray-300 text-sm uppercase tracking-wider",
                        tr {
                            th { class: "p-4 font-medium", "Date" }
                            th { class: "p-4 font-medium", "Farmer Name" }
                            th { class: "p-4 font-medium", "Item Type" }
                            th { class: "p-4 font-medium", "Quantity (kg)" }
                            th { class: "p-4 font-medium", "Amount (₹)" }
                            th { class: "p-4 font-medium text-right", "Action" }
                        }
                    }
                    tbody {
                        class: "divide-y divide-white/10",
                        for record in records.read().clone() {
                            tr {
                                class: "hover:bg-white/5 transition-colors cursor-pointer group",
                                td { class: "p-4 text-gray-400", "{record.date}" }
                                td { class: "p-4 font-medium text-emerald-400 group-hover:text-emerald-300", "{record.farmer_name}" }
                                td {
                                    class: "p-4",
                                    span {
                                        class: "px-3 py-1 rounded-full text-xs font-semibold bg-emerald-500/20 text-emerald-300 border border-emerald-500/30 cursor-pointer",
                                        "{record.item_type}"
                                    }
                                }
                                td { class: "p-4 text-gray-300", "{record.quantity_kg}" }
                                td { class: "p-4 font-bold text-white", "₹{record.amount}" }
                                td {
                                    class: "p-4 text-right",
                                    button {
                                        class: "text-gray-400 hover:text-red-400 transition-colors cursor-pointer",
                                        onclick: move |_| {
                                            let id = record.id;
                                            records.write().retain(|r| r.id != id);
                                        },
                                        "Delete"
                                    }
                                }
                            }
                        }
                    }
                }

                if records.read().is_empty() {
                    div {
                        class: "p-8 text-center text-gray-400 cursor-default",
                        "No records found. Click '+ New Record' to add one."
                    }
                }
            }
        }
    }
}

#[component]
fn AddRecord() -> Element {
    let mut records = use_context::<Signal<Vec<Record>>>();

    let mut farmer_name = use_signal(|| "".to_string());
    let mut item_type = use_signal(|| "Wheat Bran".to_string());
    let mut quantity = use_signal(|| "".to_string());
    let mut amount = use_signal(|| "".to_string());
    let mut is_submitted = use_signal(|| false);

    let handle_submit = move |e: Event<FormData>| {
        e.prevent_default();
        let q: f32 = quantity.read().parse().unwrap_or(0.0);
        let a: f32 = amount.read().parse().unwrap_or(0.0);

        let new_id = records.read().iter().map(|r| r.id).max().unwrap_or(0) + 1;

        records.write().push(Record {
            id: new_id,
            farmer_name: farmer_name.read().clone(),
            item_type: item_type.read().clone(),
            quantity_kg: q,
            amount: a,
            date: "2026-09-29".to_string(),
        });

        farmer_name.write().clear();
        quantity.write().clear();
        amount.write().clear();
        *is_submitted.write() = true;
    };

    rsx! {
        div {
            class: "max-w-2xl mx-auto px-6 page-transition",

            h1 { class: "text-4xl font-extrabold text-white tracking-tight mb-8 cursor-default", "Add New Record" }

            div {
                class: "glass-panel p-8 rounded-2xl cursor-default",

                if *is_submitted.read() {
                    div {
                        class: "mb-6 p-4 bg-emerald-500/20 border border-emerald-500/50 rounded-lg text-emerald-300 font-medium flex items-center justify-between cursor-pointer",
                        "Record added successfully!"
                        button {
                            class: "text-emerald-400 hover:text-white cursor-pointer px-2",
                            onclick: move |_| *is_submitted.write() = false,
                            "×"
                        }
                    }
                }

                form {
                    onsubmit: handle_submit,
                    class: "space-y-6",

                    // Farmer Name
                    div {
                        label { class: "block text-sm font-medium text-gray-300 mb-2 cursor-pointer", "Farmer Name" }
                        input {
                            type: "text",
                            class: "w-full bg-black/30 border border-white/10 rounded-lg px-4 py-3 text-white placeholder-gray-500 focus:outline-none focus:border-emerald-500 focus:ring-1 focus:ring-emerald-500 transition-colors cursor-text",
                            placeholder: "Enter farmer's name",
                            value: "{farmer_name}",
                            oninput: move |e| *farmer_name.write() = e.value(),
                            required: true,
                        }
                    }

                    // Item Type
                    div {
                        label { class: "block text-sm font-medium text-gray-300 mb-2 cursor-pointer", "Feed Type" }
                        select {
                            class: "w-full bg-black/30 border border-white/10 rounded-lg px-4 py-3 text-white focus:outline-none focus:border-emerald-500 transition-colors cursor-pointer appearance-none",
                            value: "{item_type}",
                            onchange: move |e| *item_type.write() = e.value(),
                            option { value: "Wheat Bran", "Wheat Bran" }
                            option { value: "Chaff (Chokar)", "Chaff (Chokar)" }
                            option { value: "Compound Feed", "Compound Feed" }
                        }
                    }

                    // Quantity and Amount row
                    div {
                        class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                        div {
                            label { class: "block text-sm font-medium text-gray-300 mb-2 cursor-pointer", "Quantity (kg)" }
                            input {
                                type: "number",
                                step: "0.01",
                                class: "w-full bg-black/30 border border-white/10 rounded-lg px-4 py-3 text-white placeholder-gray-500 focus:outline-none focus:border-emerald-500 transition-colors cursor-text",
                                placeholder: "0.00",
                                value: "{quantity}",
                                oninput: move |e| *quantity.write() = e.value(),
                                required: true,
                            }
                        }
                        div {
                            label { class: "block text-sm font-medium text-gray-300 mb-2 cursor-pointer", "Total Amount (₹)" }
                            input {
                                type: "number",
                                step: "0.01",
                                class: "w-full bg-black/30 border border-white/10 rounded-lg px-4 py-3 text-white placeholder-gray-500 focus:outline-none focus:border-emerald-500 transition-colors cursor-text",
                                placeholder: "0.00",
                                value: "{amount}",
                                oninput: move |e| *amount.write() = e.value(),
                                required: true,
                            }
                        }
                    }

                    // Submit button
                    div {
                        class: "pt-4",
                        button {
                            type: "submit",
                            class: "w-full btn-3d py-4 text-lg font-bold cursor-pointer flex justify-center items-center gap-2",
                            "Save Record"
                        }
                    }
                }
            }
        }
    }
}
