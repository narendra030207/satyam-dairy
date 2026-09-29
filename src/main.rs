use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Login {},

    #[layout(Navbar)]
        #[route("/dashboard")]
        Dashboard {},
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
    item_type: String,
    quantity_kg: f32,
    amount: f32,
    date: String,
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_effect(|| {
        let _ = dioxus::document::eval(
            r#"
            if ('serviceWorker' in navigator) {
                navigator.serviceWorker.register('/sw.js').then(function(registration) {
                    console.log('ServiceWorker registration successful with scope: ', registration.scope);
                }).catch(function(err) {
                    console.log('ServiceWorker registration failed: ', err);
                });
            }
        "#,
        );
    });

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

        // PWA Links
        document::Link { rel: "manifest", href: "/manifest.json" }
        document::Link { rel: "apple-touch-icon", href: "/icon.svg" }
        document::Meta { name: "theme-color", content: "#007aff" }

        // Modern SVG Watermark Background
        div {
            class: "fixed inset-0 w-full h-full -z-50 pointer-events-none flex justify-center items-center overflow-hidden",

            // Giant subtle cow and milk drop SVG watermark
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                class: "w-[120vw] md:w-[600px] h-auto text-black dark:text-white opacity-[0.03] dark:opacity-[0.02] transform -rotate-12",

                // Cow Path
                path {
                    fill: "currentColor",
                    d: "M10.5 18a.5.5 0 0 1 .5.5a.5.5 0 0 1-.5.5a.5.5 0 0 1-.5-.5a.5.5 0 0 1 .5-.5m3 0a.5.5 0 0 1 .5.5a.5.5 0 0 1-.5.5a.5.5 0 0 1-.5-.5a.5.5 0 0 1 .5-.5M10 11a1 1 0 0 1 1 1a1 1 0 0 1-1 1a1 1 0 0 1-1-1a1 1 0 0 1 1-1m4 0a1 1 0 0 1 1 1a1 1 0 0 1-1 1a1 1 0 0 1-1-1a1 1 0 0 1 1-1m4 7c0 2.21-2.69 4-6 4s-6-1.79-6-4c0-.9.45-1.73 1.2-2.4c-.75-1-1.2-2.25-1.2-3.6l.12-1.22c-.54.15-1.19.15-1.72 0c-1.02-.28-2.56-1.43-2.33-2.23s2.14-.95 3.16-.65c.59.17 1.22.6 1.59 1.06l.57-.81C6.79 7.05 7 4 10 3l-.09.14c-.28.44-1 1.83-.24 3.33a6.02 6.02 0 0 1 4.66 0c.76-1.5.04-2.89-.24-3.33L14 3c3 1 3.21 4.05 2.61 5.15l.57.81c.37-.46 1-.89 1.59-1.06c1.02-.3 2.93-.15 3.16.65s-1.31 1.95-2.33 2.23c-.53.15-1.18.15-1.72 0L18 12c0 1.35-.45 2.6-1.2 3.6c.75.67 1.2 1.5 1.2 2.4m-6-2c-2.21 0-4 .9-4 2s1.79 2 4 2s4-.9 4-2s-1.79-2-4-2m0-2c1.12 0 2.17.21 3.07.56c.58-.69.93-1.56.93-2.56a4 4 0 0 0-4-4a4 4 0 0 0-4 4c0 1 .35 1.87.93 2.56c.9-.35 1.95-.56 3.07-.56m2.09-10.86"
                }
            }

            // Second floating Milk drop SVG
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                class: "absolute top-1/4 right-1/4 w-[60vw] md:w-[300px] h-auto text-black dark:text-white opacity-[0.02] dark:opacity-[0.015] transform rotate-12",

                path {
                    fill: "currentColor",
                    d: "m12 3.77l-.75.84S9.97 6.06 8.68 7.94S6 12.07 6 14.23a6 6 0 0 0 6 6a6 6 0 0 0 6-6c0-2.16-1.39-4.41-2.68-6.29s-2.57-3.33-2.57-3.33zm0 3.13c.44.52.84.95 1.68 2.17c1.21 1.76 2.32 4 2.32 5.16c0 2.22-1.78 4-4 4s-4-1.78-4-4c0-1.16 1.11-3.4 2.32-5.16c.84-1.22 1.24-1.65 1.68-2.17"
                }
            }
        }

        Router::<Route> {}
    }
}

#[component]
fn Navbar() -> Element {
    rsx! {
        nav {
            class: "fixed w-full z-50 ios-nav px-4 h-11 flex justify-between items-center cursor-default",
            div {
                class: "w-1/3 flex justify-start",
                Link {
                    to: Route::Dashboard {},
                    class: "text-[#007aff] dark:text-[#0a84ff] text-[17px] active:opacity-50 transition-opacity flex items-center cursor-pointer",
                    "Dashboard"
                }
            }
            div {
                class: "w-1/3 flex justify-center",
                span {
                    class: "text-[17px] font-semibold text-black dark:text-white cursor-default",
                    "Satyam Dairy"
                }
            }
            div {
                class: "w-1/3 flex justify-end gap-3",
                Link {
                    to: Route::Records {},
                    class: "text-[#007aff] dark:text-[#0a84ff] text-[17px] active:opacity-50 transition-opacity cursor-pointer",
                    "Records"
                }
                Link {
                    to: Route::AddRecord {},
                    class: "text-[#007aff] dark:text-[#0a84ff] text-[17px] font-semibold active:opacity-50 transition-opacity cursor-pointer",
                    "Add"
                }
            }
        }
        div {
            class: "pt-[60px] pb-8 min-h-screen",
            Outlet::<Route> {}
        }
    }
}

#[component]
fn Dashboard() -> Element {
    let records = use_context::<Signal<Vec<Record>>>();

    let total_kg = records.read().iter().map(|r| r.quantity_kg).sum::<f32>();
    let total_kg = if total_kg.abs() < 0.001 {
        0.0
    } else {
        total_kg
    };

    let total_amount = records.read().iter().map(|r| r.amount).sum::<f32>();
    let total_amount = if total_amount.abs() < 0.001 {
        0.0
    } else {
        total_amount
    };
    let total_farmers = records.read().len();

    rsx! {
        div {
            class: "max-w-3xl mx-auto px-4 page-transition",

            h1 { class: "text-[34px] font-bold text-black dark:text-white mt-2 mb-6 tracking-tight cursor-default", "Overview" }

            // iOS Summary Widgets
            div {
                class: "grid grid-cols-2 gap-4 mb-8",
                div {
                    class: "ios-panel p-4 flex flex-col justify-between h-[110px] cursor-default",
                    span { class: "text-[15px] text-gray-500 font-medium flex items-center gap-1", "Farmers" }
                    span { class: "text-[28px] font-bold text-black dark:text-white", "{total_farmers}" }
                }
                div {
                    class: "ios-panel p-4 flex flex-col justify-between h-[110px] cursor-default",
                    span { class: "text-[15px] text-gray-500 font-medium flex items-center gap-1", "Feed (kg)" }
                    span { class: "text-[28px] font-bold text-[#007aff] dark:text-[#0a84ff]", "{total_kg}" }
                }
                div {
                    class: "ios-panel p-4 flex flex-col justify-between h-[110px] col-span-2 cursor-default",
                    span { class: "text-[15px] text-gray-500 font-medium flex items-center gap-1", "Total Revenue" }
                    span { class: "text-[34px] font-bold text-[#34c759] dark:text-[#32d74b]", "₹{total_amount}" }
                }
            }

            h2 { class: "text-[22px] font-bold text-black dark:text-white mb-3 cursor-default ml-2", "Recent Feed Distributions" }
            div {
                class: "ios-panel overflow-hidden",
                Link {
                    to: Route::Records {},
                    class: "flex justify-between items-center px-4 py-3 bg-transparent active:bg-gray-100 dark:active:bg-gray-800 transition-colors cursor-pointer",
                    span { class: "text-[17px] text-black dark:text-white", "View All Records" }
                    span { class: "text-gray-400 text-xl font-medium", "›" }
                }
            }
        }
    }
}

#[component]
fn Records() -> Element {
    let mut records = use_context::<Signal<Vec<Record>>>();
    let record_list = records.read().clone();
    let length = record_list.len();

    rsx! {
        div {
            class: "max-w-3xl mx-auto px-4 page-transition",

            h1 { class: "text-[34px] font-bold text-black dark:text-white mt-2 mb-6 tracking-tight cursor-default", "Records" }

            if length == 0 {
                div {
                    class: "text-center text-gray-500 mt-20 text-[17px] cursor-default",
                    "No records found."
                }
            } else {
                div {
                    class: "ios-panel overflow-hidden cursor-default",
                    for (i, record) in record_list.into_iter().enumerate() {
                        div {
                            class: "flex flex-col relative",
                            div {
                                class: "flex justify-between items-center px-4 py-2.5",
                                div {
                                    p { class: "text-[17px] font-semibold text-black dark:text-white", "{record.farmer_name}" }
                                    p { class: "text-[15px] text-gray-500", "{record.date} • {record.item_type}" }
                                }
                                div {
                                    class: "text-right flex items-center gap-4",
                                    div {
                                        p { class: "text-[17px] font-medium text-black dark:text-white", "₹{record.amount}" }
                                        p { class: "text-[15px] text-gray-500", "{record.quantity_kg} kg" }
                                    }
                                    button {
                                        class: "text-[#ff3b30] dark:text-[#ff453a] active:opacity-50 p-2 cursor-pointer",
                                        onclick: move |_| {
                                            let id = record.id;
                                            records.write().retain(|r| r.id != id);
                                        },
                                        svg {
                                            xmlns: "http://www.w3.org/2000/svg",
                                            class: "h-5 w-5",
                                            fill: "none",
                                            view_box: "0 0 24 24",
                                            stroke: "currentColor",
                                            stroke_width: "2",
                                            path {
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                d: "M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
                                            }
                                        }
                                    }
                                }
                            }
                            if i < length - 1 {
                                div { class: "ml-4 border-b border-gray-200 dark:border-gray-800" }
                            }
                        }
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
    let mut is_feed_open = use_signal(|| false);

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
            class: "max-w-3xl mx-auto px-4 page-transition",

            h1 { class: "text-[34px] font-bold text-black dark:text-white mt-2 mb-6 tracking-tight cursor-default", "New Record" }

            if *is_submitted.read() {
                div {
                    class: "mb-6 p-3 bg-[#e5f9e7] dark:bg-[#1f3b26] rounded-xl flex items-center justify-between",
                    span { class: "text-[#34c759] font-medium text-[15px]", "Record saved successfully!" }
                    button {
                        class: "text-[#34c759] font-bold px-2 active:opacity-50 cursor-pointer",
                        onclick: move |_| *is_submitted.write() = false,
                        "×"
                    }
                }
            }

            form {
                onsubmit: handle_submit,

                div {
                    class: "ios-panel mb-8 text-[17px] cursor-default",

                    div {
                        class: "flex items-center px-4 py-3 border-b border-gray-200 dark:border-gray-800",
                        label { class: "w-1/3 text-black dark:text-white", "Farmer" }
                        input {
                            type: "text",
                            class: "w-2/3 bg-transparent text-right text-gray-500 dark:text-gray-400 focus:outline-none focus:text-black dark:focus:text-white cursor-text",
                            placeholder: "Name",
                            value: "{farmer_name}",
                            oninput: move |e| *farmer_name.write() = e.value(),
                            required: true,
                        }
                    }

                    div {
                        class: "relative flex items-center px-4 py-3 border-b border-gray-200 dark:border-gray-800 cursor-pointer",
                        onclick: move |e| {
                            e.stop_propagation();
                            let current = *is_feed_open.read();
                            *is_feed_open.write() = !current;
                        },
                        label { class: "w-1/3 text-black dark:text-white pointer-events-none", "Feed" }
                        div {
                            class: "w-2/3 flex justify-end items-center pointer-events-none",
                            span {
                                class: "text-[17px] text-gray-500 dark:text-gray-400 mr-5",
                                "{item_type}"
                            }
                            span {
                                class: "absolute right-4 text-gray-400 text-xl font-medium",
                                "›"
                            }
                        }

                        if *is_feed_open.read() {
                            div {
                                class: "absolute z-[60] w-[400vw] h-[400vh] -top-[200vh] -left-[200vw] cursor-default",
                                onclick: move |e| {
                                    e.stop_propagation();
                                    *is_feed_open.write() = false;
                                }
                            }
                            div {
                                class: "absolute right-4 top-11 w-56 bg-[#f2f2f7] dark:bg-[#2c2c2e] rounded-xl shadow-[0_8px_30px_rgba(0,0,0,0.12)] dark:shadow-[0_8px_30px_rgba(0,0,0,0.6)] border border-gray-200 dark:border-gray-700 z-[70] overflow-hidden flex flex-col page-transition",
                                for option in ["Wheat Bran", "Chaff (Chokar)", "Compound Feed"].into_iter() {
                                    div {
                                        class: "px-4 py-3 text-[17px] bg-white dark:bg-[#1c1c1e] text-black dark:text-white active:bg-gray-100 dark:active:bg-[#3a3a3c] transition-colors flex justify-between items-center cursor-pointer border-b border-gray-100 dark:border-gray-800 last:border-0",
                                        onclick: move |e| {
                                            e.stop_propagation();
                                            *item_type.write() = option.to_string();
                                            *is_feed_open.write() = false;
                                        },
                                        span { "{option}" }
                                        if *item_type.read() == option {
                                            span { class: "text-[#007aff] dark:text-[#0a84ff] font-bold text-lg", "✓" }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div {
                        class: "flex items-center px-4 py-3 border-b border-gray-200 dark:border-gray-800",
                        label { class: "w-1/3 text-black dark:text-white", "Quantity (kg)" }
                        input {
                            type: "number",
                            step: "0.01",
                            class: "w-2/3 bg-transparent text-right text-gray-500 dark:text-gray-400 focus:outline-none focus:text-black dark:focus:text-white cursor-text",
                            placeholder: "0",
                            value: "{quantity}",
                            oninput: move |e| *quantity.write() = e.value(),
                            required: true,
                        }
                    }

                    div {
                        class: "flex items-center px-4 py-3",
                        label { class: "w-1/3 text-black dark:text-white", "Amount (₹)" }
                        input {
                            type: "number",
                            step: "0.01",
                            class: "w-2/3 bg-transparent text-right text-gray-500 dark:text-gray-400 focus:outline-none focus:text-black dark:focus:text-white cursor-text",
                            placeholder: "0",
                            value: "{amount}",
                            oninput: move |e| *amount.write() = e.value(),
                            required: true,
                        }
                    }
                }

                button {
                    type: "submit",
                    class: "btn-ios w-full py-[14px] text-[17px] font-semibold cursor-pointer",
                    "Save Record"
                }
            }
        }
    }
}

#[component]
fn Login() -> Element {
    let mut email = use_signal(|| "".to_string());
    let mut otp = use_signal(|| "".to_string());
    let mut otp_sent = use_signal(|| false);

    let nav = use_navigator();

    use_effect(|| {
        let _ = dioxus::document::eval(
            r#"
            if (window._enterFocusHandler) {
                window.removeEventListener('keydown', window._enterFocusHandler);
            }
            window._enterFocusHandler = function(e) {
                if (e.key === 'Enter') {
                    if (document.activeElement === document.body) {
                        let email = document.getElementById('email-input');
                        let otp = document.getElementById('otp-input');
                        if (otp) {
                            e.preventDefault();
                            otp.focus();
                        } else if (email) {
                            e.preventDefault();
                            email.focus();
                        }
                    }
                }
            };
            window.addEventListener('keydown', window._enterFocusHandler);
        "#,
        );
    });

    let handle_email_submit = move |e: Event<FormData>| {
        e.prevent_default();
        if !email.read().is_empty() {
            *otp_sent.write() = true;
        }
    };

    let handle_verify = move |e: Event<FormData>| {
        e.prevent_default();
        if !otp.read().is_empty() {
            // Mock Supabase OTP verification
            nav.push(Route::Dashboard {});
        }
    };

    let handle_google_login = move |_| {
        // Mock Supabase Google OAuth
        nav.push(Route::Dashboard {});
    };

    rsx! {
        div {
            class: "min-h-screen flex items-center justify-center px-4 page-transition",

            div {
                class: "ios-panel w-full max-w-sm p-8 flex flex-col items-center",

                // Dairy Icon / Logo Placeholder
                div {
                    class: "w-16 h-16 bg-[#007aff] rounded-2xl flex items-center justify-center mb-6 shadow-md",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "w-10 h-10 text-white",
                        view_box: "0 0 24 24",
                        path {
                            fill: "currentColor",
                            d: "m12 3.77l-.75.84S9.97 6.06 8.68 7.94S6 12.07 6 14.23a6 6 0 0 0 6 6a6 6 0 0 0 6-6c0-2.16-1.39-4.41-2.68-6.29s-2.57-3.33-2.57-3.33zm0 3.13c.44.52.84.95 1.68 2.17c1.21 1.76 2.32 4 2.32 5.16c0 2.22-1.78 4-4 4s-4-1.78-4-4c0-1.16 1.11-3.4 2.32-5.16c.84-1.22 1.24-1.65 1.68-2.17"
                        }
                    }
                }

                h1 { class: "text-[26px] font-bold text-black dark:text-white mb-2 tracking-tight", "Satyam Dairy" }

                if !*otp_sent.read() {
                    p { class: "text-[15px] text-gray-500 text-center mb-8", "Sign in with your email or Google. No password required." }

                    form {
                        class: "w-full",
                        onsubmit: handle_email_submit,

                        input {
                            id: "email-input",
                            type: "email",
                            class: "w-full bg-[#f2f2f7] dark:bg-[#2c2c2e] text-black dark:text-white px-4 py-3.5 rounded-xl focus:outline-none mb-4 text-[17px] border border-transparent focus:border-gray-300 dark:focus:border-gray-600 transition-colors placeholder-gray-400",
                            placeholder: "Email address",
                            value: "{email}",
                            oninput: move |e| *email.write() = e.value(),
                            required: true,
                        }

                        button {
                            type: "submit",
                            class: "btn-ios w-full py-3.5 text-[17px] font-semibold",
                            "Send OTP"
                        }
                    }

                    // Divider
                    div {
                        class: "w-full flex items-center my-6",
                        div { class: "flex-grow border-t border-gray-200 dark:border-gray-800" }
                        span { class: "px-3 text-[15px] text-gray-400 font-medium", "or" }
                        div { class: "flex-grow border-t border-gray-200 dark:border-gray-800" }
                    }

                    // Google Button
                    button {
                        class: "w-full flex items-center justify-center gap-3 bg-white dark:bg-[#1c1c1e] text-black dark:text-white border border-gray-200 dark:border-gray-700 py-3.5 rounded-xl text-[17px] font-semibold active:bg-gray-50 dark:active:bg-gray-800 transition-colors cursor-pointer",
                        onclick: handle_google_login,
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            class: "w-5 h-5",
                            view_box: "0 0 24 24",
                            path {
                                fill: "#4285F4",
                                d: "M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"
                            }
                            path {
                                fill: "#34A853",
                                d: "M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"
                            }
                            path {
                                fill: "#FBBC05",
                                d: "M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"
                            }
                            path {
                                fill: "#EA4335",
                                d: "M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"
                            }
                        }
                        "Continue with Google"
                    }
                } else {
                    p { class: "text-[15px] text-gray-500 text-center mb-8", "We sent a secure 6-digit OTP code to {email.read()}" }

                    form {
                        class: "w-full",
                        onsubmit: handle_verify,

                        input {
                            id: "otp-input",
                            type: "text",
                            class: "w-full bg-[#f2f2f7] dark:bg-[#2c2c2e] text-black dark:text-white px-4 py-4 rounded-xl focus:outline-none mb-6 text-[24px] tracking-[0.5em] text-center font-mono border border-transparent focus:border-[#007aff] transition-colors placeholder-gray-300 dark:placeholder-gray-600",
                            placeholder: "------",
                            maxlength: "6",
                            value: "{otp}",
                            oninput: move |e| *otp.write() = e.value(),
                            required: true,
                        }

                        button {
                            type: "submit",
                            class: "btn-ios w-full py-3.5 text-[17px] font-semibold mb-4",
                            "Verify & Sign In"
                        }

                        button {
                            type: "button",
                            class: "w-full text-[15px] text-[#007aff] active:opacity-50 transition-opacity font-medium cursor-pointer",
                            onclick: move |_| *otp_sent.write() = false,
                            "Use a different email"
                        }
                    }
                }
            }
        }
    }
}
