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
            class: "fixed w-full z-50 h-16 flex justify-between items-center px-4 bg-[#fbfdf9] dark:bg-[#191c1a] shadow-sm",
            div {
                class: "flex items-center gap-4",
                Link {
                    to: Route::Dashboard {},
                    class: "text-[22px] font-medium text-[#191c1a] dark:text-[#e1e3de]",
                    "Satyam Dairy"
                }
            }
            div {
                class: "flex items-center gap-2",
                Link {
                    to: Route::AddRecord {},
                    class: "w-12 h-12 rounded-full flex items-center justify-center text-[#404943] dark:text-[#c0c9c1] hover:bg-[#191c1a]/5 dark:hover:bg-[#fbfdf9]/5 transition-colors cursor-pointer",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "w-6 h-6",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke: "currentColor",
                        stroke_width: "2",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M12 4v16m8-8H4"
                        }
                    }
                }
                Link {
                    to: Route::Records {},
                    class: "w-12 h-12 rounded-full flex items-center justify-center text-[#404943] dark:text-[#c0c9c1] hover:bg-[#191c1a]/5 dark:hover:bg-[#fbfdf9]/5 transition-colors cursor-pointer",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "w-6 h-6",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke: "currentColor",
                        stroke_width: "2",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M4 6h16M4 12h16M4 18h16"
                        }
                    }
                }
            }
        }

        div {
            class: "pt-16 min-h-screen w-full",
            Outlet::<Route> {}
        }
    }
}
#[component]
fn Dashboard() -> Element {
    let records = use_context::<Signal<Vec<Record>>>();
    let rs = records.read();

    let total_farmers = {
        let mut names: Vec<&String> = rs.iter().map(|r| &r.farmer_name).collect();
        names.sort();
        names.dedup();
        names.len()
    };

    let total_feed: f32 = rs.iter().map(|r| r.quantity_kg).sum();
    let total_revenue: f32 = rs.iter().map(|r| r.amount).sum();

    rsx! {
        div {
            class: "px-4 pt-6 pb-24 page-transition max-w-2xl mx-auto",

            h1 { class: "text-[28px] font-normal mb-6 text-[#191c1a] dark:text-[#e1e3de]", "Overview" }

            div {
                class: "grid grid-cols-2 gap-4 mb-8",

                div {
                    class: "col-span-2 p-5 flex flex-col bg-[#f0f3ec] dark:bg-[#202522] rounded-[24px]",
                    span { class: "text-[14px] font-medium text-[#404943] dark:text-[#c0c9c1] mb-1", "Total Revenue (₹)" },
                    span {
                        class: "text-[36px] font-normal text-[#191c1a] dark:text-[#e1e3de]",
                        if total_revenue.abs() < 0.001 { "0" } else { "{total_revenue}" }
                    }
                }

                div {
                    class: "p-5 flex flex-col bg-[#f0f3ec] dark:bg-[#202522] rounded-[24px]",
                    span { class: "text-[14px] font-medium text-[#404943] dark:text-[#c0c9c1] mb-1", "Farmers" },
                    span { class: "text-[28px] font-normal text-[#191c1a] dark:text-[#e1e3de]", "{total_farmers}" }
                }

                div {
                    class: "p-5 flex flex-col bg-[#f0f3ec] dark:bg-[#202522] rounded-[24px]",
                    span { class: "text-[14px] font-medium text-[#404943] dark:text-[#c0c9c1] mb-1", "Total Feed (kg)" },
                    span {
                        class: "text-[28px] font-normal text-[#191c1a] dark:text-[#e1e3de]",
                        if total_feed.abs() < 0.001 { "0" } else { "{total_feed}" }
                    }
                }
            }

            div {
                class: "flex flex-row items-center justify-between mb-4",
                h2 { class: "text-[20px] font-medium text-[#191c1a] dark:text-[#e1e3de]", "Recent Activity" },
                Link {
                    to: Route::Records {},
                    class: "text-[14px] font-medium text-[#006c4a] dark:text-[#58d6a5] hover:opacity-80 transition-opacity",
                    "View All"
                }
            }

            div {
                class: "bg-[#f0f3ec] dark:bg-[#202522] rounded-[24px] overflow-hidden",

                if rs.is_empty() {
                    div {
                        class: "p-8 text-center text-[#404943] dark:text-[#c0c9c1]",
                        "No records yet."
                    }
                } else {
                    div {
                        class: "flex flex-col",
                        for (i, record) in rs.iter().rev().take(3).enumerate() {
                            div {
                                class: format!("p-4 mx-2 flex flex-row justify-between items-center {}",
                                    if i != 2 && i != rs.len() - 1 { "border-b border-[#c0c9c1]/30 dark:border-[#404943]/30" } else { "" }),
                                div {
                                    class: "flex flex-col",
                                    span { class: "text-[16px] font-medium text-[#191c1a] dark:text-[#e1e3de]", "{record.farmer_name}" },
                                    span { class: "text-[14px] text-[#404943] dark:text-[#c0c9c1]", "{record.date} • {record.item_type}" }
                                }
                                div {
                                    class: "flex flex-col items-end",
                                    span { class: "text-[16px] font-medium text-[#191c1a] dark:text-[#e1e3de]", "₹{record.amount}" },
                                    span { class: "text-[14px] text-[#404943] dark:text-[#c0c9c1]", "{record.quantity_kg} kg" }
                                }
                            }
                        }
                    }
                }
            }

            Link {
                to: Route::AddRecord {},
                class: "fixed bottom-6 right-6 w-[56px] h-[56px] bg-[#006c4a] dark:bg-[#58d6a5] text-white dark:text-[#003825] rounded-[16px] shadow-md flex items-center justify-center hover:shadow-lg active:scale-95 transition-all z-50",
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    class: "w-6 h-6",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke: "currentColor",
                    stroke_width: "2",
                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M12 4v16m8-8H4" }
                }
            }
        }
    }
}

#[component]
fn Records() -> Element {
    let records = use_context::<Signal<Vec<Record>>>();
    let rs = records.read().clone();

    rsx! {
        div {
            class: "px-4 pt-6 pb-24 page-transition max-w-2xl mx-auto",

            h1 { class: "text-[28px] font-normal mb-6 text-[#191c1a] dark:text-[#e1e3de]", "All Records" }

            div {
                class: "bg-[#f0f3ec] dark:bg-[#202522] rounded-[24px] overflow-hidden",

                if rs.is_empty() {
                    div {
                        class: "p-8 text-center text-[#404943] dark:text-[#c0c9c1]",
                        "No records found."
                    }
                } else {
                    div {
                        class: "flex flex-col",
                        for (i, record) in rs.into_iter().rev().enumerate() {
                            div {
                                class: format!("p-4 mx-2 flex flex-row justify-between items-center hover:bg-[#191c1a]/5 dark:hover:bg-[#fbfdf9]/5 transition-colors {}",
                                    if i != records.read().len() - 1 { "border-b border-[#c0c9c1]/30 dark:border-[#404943]/30" } else { "" }),
                                div {
                                    class: "flex flex-col",
                                    span { class: "text-[16px] font-medium text-[#191c1a] dark:text-[#e1e3de]", "{record.farmer_name}" },
                                    span { class: "text-[14px] text-[#404943] dark:text-[#c0c9c1]", "{record.date} • {record.item_type}" }
                                }
                                div {
                                    class: "flex flex-col items-end",
                                    span { class: "text-[16px] font-medium text-[#191c1a] dark:text-[#e1e3de]", "₹{record.amount}" },
                                    span { class: "text-[14px] text-[#404943] dark:text-[#c0c9c1]", "{record.quantity_kg} kg" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

const FEED_OPTIONS: [&str; 3] = ["Wheat Bran", "Chaff/Chokar", "Corn Meal"];

#[component]
fn AddRecord() -> Element {
    let mut records = use_context::<Signal<Vec<Record>>>();

    let mut farmer_name = use_signal(|| "".to_string());
    let mut item_type = use_signal(|| "Wheat Bran".to_string());
    let mut quantity = use_signal(|| "".to_string());
    let mut amount = use_signal(|| "".to_string());
    let mut is_premium_menu_open = use_signal(|| false);
    let nav = use_navigator();

    let handle_submit = move |e: Event<FormData>| {
        e.prevent_default();

        let q: f32 = quantity.read().parse().unwrap_or(0.0);
        let a: f32 = amount.read().parse().unwrap_or(0.0);

        if !farmer_name.read().is_empty() && q > 0.0 && a > 0.0 {
            let mut rs = records.write();
            let new_id = rs.len() + 1;
            rs.push(Record {
                id: new_id,
                farmer_name: farmer_name.read().clone(),
                item_type: item_type.read().clone(),
                quantity_kg: q,
                amount: a,
                date: "Today".to_string(),
            });
            nav.push(Route::Dashboard {});
        }
    };

    rsx! {
        div {
            class: "px-4 pt-6 pb-24 page-transition max-w-xl mx-auto",

            if *is_premium_menu_open.read() {
                div {
                    class: "absolute z-[60] w-[400vw] h-[400vh] -top-[200vh] -left-[200vw]",
                    onclick: move |_| *is_premium_menu_open.write() = false
                }
            }

            h1 { class: "text-[28px] font-normal mb-6 text-[#191c1a] dark:text-[#e1e3de]", "New Record" }

            form {
                onsubmit: handle_submit,
                class: "flex flex-col gap-6",

                div {
                    class: "bg-[#f0f3ec] dark:bg-[#202522] rounded-[24px] p-6 shadow-sm flex flex-col gap-6",

                    div {
                        class: "flex flex-col",
                        label { class: "text-[12px] font-medium text-[#404943] dark:text-[#c0c9c1] mb-1 px-1", "Farmer Name" }
                        input {
                            type: "text",
                            class: "w-full bg-transparent border border-[#727971] dark:border-[#8b938a] text-[#191c1a] dark:text-[#e1e3de] px-4 py-3.5 rounded-[4px] focus:outline-none focus:border-[#006c4a] dark:focus:border-[#58d6a5] focus:border-2 transition-all",
                            placeholder: "e.g. Ramesh Kumar",
                            value: "{farmer_name}",
                            oninput: move |e| *farmer_name.write() = e.value(),
                            required: true,
                        }
                    }

                    div {
                        class: "flex flex-col relative z-[70]",
                        label { class: "text-[12px] font-medium text-[#404943] dark:text-[#c0c9c1] mb-1 px-1", "Feed Type" }

                        div {
                            class: "w-full bg-transparent border border-[#727971] dark:border-[#8b938a] text-[#191c1a] dark:text-[#e1e3de] px-4 py-3.5 rounded-[4px] cursor-pointer flex justify-between items-center transition-all",
                            onclick: move |_| {
                                let current = *is_premium_menu_open.read();
                                *is_premium_menu_open.write() = !current;
                            },
                            span { class: "text-[16px]", "{item_type}" }
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                class: format!("w-5 h-5 text-[#404943] dark:text-[#c0c9c1] transition-transform duration-200 {}", if *is_premium_menu_open.read() { "rotate-180" } else { "" }),
                                view_box: "0 0 20 20",
                                fill: "currentColor",
                                path { fill_rule: "evenodd", clip_rule: "evenodd", d: "M5.23 7.21a.75.75 0 011.06.02L10 11.168l3.71-3.938a.75.75 0 111.08 1.04l-4.25 4.5a.75.75 0 01-1.08 0l-4.25-4.5a.75.75 0 01.02-1.06z" }
                            }
                        }

                        if *is_premium_menu_open.read() {
                            div {
                                class: "absolute top-[100%] left-0 w-full mt-1 bg-[#fbfdf9] dark:bg-[#191c1a] border border-[#727971]/30 dark:border-[#8b938a]/30 rounded-[8px] shadow-lg overflow-hidden py-1 z-[80]",
                                for opt in FEED_OPTIONS.iter() {
                                    div {
                                        class: "px-4 py-3 text-[16px] text-[#191c1a] dark:text-[#e1e3de] hover:bg-[#191c1a]/5 dark:hover:bg-[#fbfdf9]/5 cursor-pointer flex justify-between items-center transition-colors",
                                        onclick: {
                                            let option_val = opt.to_string();
                                            move |_| {
                                                *item_type.write() = option_val.clone();
                                                *is_premium_menu_open.write() = false;
                                            }
                                        },
                                        span { "{opt}" }
                                        if *item_type.read() == *opt {
                                            svg {
                                                xmlns: "http://www.w3.org/2000/svg",
                                                class: "w-5 h-5 text-[#006c4a] dark:text-[#58d6a5]",
                                                view_box: "0 0 20 20",
                                                fill: "currentColor",
                                                path { fill_rule: "evenodd", clip_rule: "evenodd", d: "M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div {
                        class: "flex flex-col",
                        label { class: "text-[12px] font-medium text-[#404943] dark:text-[#c0c9c1] mb-1 px-1", "Quantity (kg)" }
                        input {
                            type: "number",
                            step: "0.1",
                            class: "w-full bg-transparent border border-[#727971] dark:border-[#8b938a] text-[#191c1a] dark:text-[#e1e3de] px-4 py-3.5 rounded-[4px] focus:outline-none focus:border-[#006c4a] dark:focus:border-[#58d6a5] focus:border-2 transition-all",
                            placeholder: "0.0",
                            value: "{quantity}",
                            oninput: move |e| *quantity.write() = e.value(),
                            required: true,
                        }
                    }

                    div {
                        class: "flex flex-col",
                        label { class: "text-[12px] font-medium text-[#404943] dark:text-[#c0c9c1] mb-1 px-1", "Total Amount (₹)" }
                        input {
                            type: "number",
                            class: "w-full bg-transparent border border-[#727971] dark:border-[#8b938a] text-[#191c1a] dark:text-[#e1e3de] px-4 py-3.5 rounded-[4px] focus:outline-none focus:border-[#006c4a] dark:focus:border-[#58d6a5] focus:border-2 transition-all",
                            placeholder: "0",
                            value: "{amount}",
                            oninput: move |e| *amount.write() = e.value(),
                            required: true,
                        }
                    }
                }

                button {
                    type: "submit",
                    class: "w-full rounded-full bg-[#006c4a] dark:bg-[#58d6a5] text-white dark:text-[#003825] py-4 text-[15px] font-medium tracking-wide shadow-sm hover:shadow-md active:scale-95 transition-all",
                    "Save Record"
                }
            }
        }
    }
}

#[component]
fn Login() -> Element {
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let mut show_password = use_signal(|| false);
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
                class: "w-full max-w-sm p-8 flex flex-col items-center bg-[#f0f3ec] dark:bg-[#202522] rounded-[28px] shadow-sm",

                // Logo
                div {
                    class: "w-16 h-16 bg-[#006c4a] dark:bg-[#58d6a5] rounded-full flex items-center justify-center mb-6 shadow-sm",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "w-10 h-10 text-white dark:text-[#003825]",
                        view_box: "0 0 24 24",
                        path {
                            fill: "currentColor",
                            d: "m12 3.77l-.75.84S9.97 6.06 8.68 7.94S6 12.07 6 14.23a6 6 0 0 0 6 6a6 6 0 0 0 6-6c0-2.16-1.39-4.41-2.68-6.29s-2.57-3.33-2.57-3.33zm0 3.13c.44.52.84.95 1.68 2.17c1.21 1.76 2.32 4 2.32 5.16c0 2.22-1.78 4-4 4s-4-1.78-4-4c0-1.16 1.11-3.4 2.32-5.16c.84-1.22 1.24-1.65 1.68-2.17"
                        }
                    }
                }

                h1 { class: "text-[26px] font-normal text-[#191c1a] dark:text-[#e1e3de] mb-2 tracking-tight", "Satyam Dairy" }

                if !*otp_sent.read() {
                    p { class: "text-[14px] text-[#404943] dark:text-[#c0c9c1] text-center mb-8", "Sign in with your email and password, or use Google." }

                    form {
                        class: "w-full",
                        onsubmit: handle_email_submit,

                        input {
                            id: "email-input",
                            type: "email",
                            class: "w-full bg-transparent border border-[#727971] dark:border-[#8b938a] text-[#191c1a] dark:text-[#e1e3de] px-4 py-3.5 rounded-[4px] focus:outline-none focus:border-[#006c4a] dark:focus:border-[#58d6a5] focus:border-2 transition-all mb-4 text-[16px]",
                            placeholder: "Email address",
                            value: "{email}",
                            oninput: move |e| *email.write() = e.value(),
                            required: true,
                        }

                        div {
                            class: "w-full relative mb-4",
                            input {
                                id: "password-input",
                                type: if *show_password.read() { "text" } else { "password" },
                                class: "w-full bg-transparent border border-[#727971] dark:border-[#8b938a] text-[#191c1a] dark:text-[#e1e3de] px-4 py-3.5 pr-12 rounded-[4px] focus:outline-none focus:border-[#006c4a] dark:focus:border-[#58d6a5] focus:border-2 transition-all text-[16px]",
                                placeholder: "Password",
                                value: "{password}",
                                oninput: move |e| *password.write() = e.value(),
                                required: true,
                            }
                            button {
                                type: "button",
                                class: "absolute right-3 top-1/2 -translate-y-1/2 p-1 text-[#404943] dark:text-[#c0c9c1] hover:text-[#191c1a] dark:hover:text-[#e1e3de] transition-colors cursor-pointer",
                                onclick: move |_| {
                                    let current = *show_password.read();
                                    *show_password.write() = !current;
                                },
                                if *show_password.read() {
                                    svg { xmlns: "http://www.w3.org/2000/svg", class: "w-5 h-5", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                        path { d: "M17.94 17.94A10.07 10.07 0 0112 20c-7 0-11-8-11-8a18.45 18.45 0 015.06-5.94M9.9 4.24A9.12 9.12 0 0112 4c7 0 11 8 11 8a18.5 18.5 0 01-2.16 3.19m-6.72-1.07a3 3 0 11-4.24-4.24" }
                                        path { d: "M1 1l22 22" }
                                    }
                                } else {
                                    svg { xmlns: "http://www.w3.org/2000/svg", class: "w-5 h-5", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                        path { d: "M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z" }
                                        circle { cx: "12", cy: "12", r: "3" }
                                    }
                                }
                            }
                        }

                        button {
                            type: "submit",
                            class: "w-full rounded-full bg-[#006c4a] dark:bg-[#58d6a5] text-white dark:text-[#003825] py-3.5 text-[15px] font-medium tracking-wide shadow-sm hover:shadow-md active:scale-95 transition-all",
                            "Sign In"
                        }
                    }

                    // Divider
                    div {
                        class: "w-full flex items-center my-6",
                        div { class: "flex-grow border-t border-[#c0c9c1]/50 dark:border-[#404943]/50" }
                        span { class: "px-3 text-[14px] text-[#404943] dark:text-[#c0c9c1] font-medium", "or" }
                        div { class: "flex-grow border-t border-[#c0c9c1]/50 dark:border-[#404943]/50" }
                    }

                    // Google Button (M3 Outlined Button style)
                    button {
                        class: "w-full flex items-center justify-center gap-3 bg-transparent border border-[#727971] dark:border-[#8b938a] text-[#191c1a] dark:text-[#e1e3de] py-3.5 rounded-full text-[15px] font-medium hover:bg-[#191c1a]/5 dark:hover:bg-[#fbfdf9]/5 active:scale-95 transition-all cursor-pointer",
                        onclick: handle_google_login,
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            class: "w-5 h-5",
                            view_box: "0 0 24 24",
                            path { fill: "#4285F4", d: "M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z" }
                            path { fill: "#34A853", d: "M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z" }
                            path { fill: "#FBBC05", d: "M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z" }
                            path { fill: "#EA4335", d: "M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z" }
                        }
                        "Continue with Google"
                    }
                } else {
                    p { class: "text-[14px] text-[#404943] dark:text-[#c0c9c1] text-center mb-8", "We sent a secure 6-digit OTP code to {email.read()}" }

                    form {
                        class: "w-full",
                        onsubmit: handle_verify,

                        input {
                            id: "otp-input",
                            type: "text",
                            inputmode: "numeric",
                            pattern: "[0-9]*",
                            class: "w-full bg-transparent border border-[#727971] dark:border-[#8b938a] text-[#191c1a] dark:text-[#e1e3de] px-4 py-4 rounded-[4px] focus:outline-none focus:border-[#006c4a] dark:focus:border-[#58d6a5] focus:border-2 transition-all mb-6 text-[24px] tracking-[0.5em] text-center font-mono",
                            placeholder: "......",
                            maxlength: "6",
                            value: "{otp}",
                            oninput: move |e| {
                                let val: String = e.value().chars().filter(|c| c.is_ascii_digit()).collect();
                                *otp.write() = val;
                            },
                            required: true,
                        }

                        button {
                            type: "submit",
                            class: "w-full rounded-full bg-[#006c4a] dark:bg-[#58d6a5] text-white dark:text-[#003825] py-3.5 text-[15px] font-medium tracking-wide shadow-sm hover:shadow-md active:scale-95 transition-all mb-4",
                            "Verify & Sign In"
                        }

                        button {
                            type: "button",
                            class: "w-full text-[14px] text-[#006c4a] dark:text-[#58d6a5] active:opacity-50 transition-opacity font-medium cursor-pointer",
                            onclick: move |_| *otp_sent.write() = false,
                            "Use a different email"
                        }
                    }
                }
            }
        }
    }
}
