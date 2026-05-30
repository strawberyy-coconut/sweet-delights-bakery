use dioxus::prelude::*;
use lucide_dioxus::{Clock, Coffee, Mail, MapPin, Phone, Sparkles};

const BAKERY_INTERIOR: Asset = asset!("/assets/images/interior.png", AssetOptions::image().with_preload(true));

#[component]
pub fn Contact() -> Element {
    rsx! {
        // Page header
        div { class: "bg-gradient-to-r from-honey-700 to-honey-600 py-16",
            div { class: "max-w-7xl mx-auto px-4 text-center",
                h1 {
                    class: "text-4xl md:text-5xl font-bold text-cream-50 mb-4",
                    style: "font-family: 'Playfair Display', serif;",
                    "Get in Touch"
                }
                p {
                    class: "text-lg text-cream-50/90 max-w-xl mx-auto",
                    style: "font-family: 'Nunito', sans-serif;",
                    "We'd love to hear from you! Whether you have a question, a special order, or just want to say hello."
                }
            }
        }

        div { class: "max-w-7xl mx-auto px-4 py-16",
            div { class: "grid grid-cols-1 lg:grid-cols-2 gap-12",
                // Left: Contact info
                div { class: "space-y-8",
                    h2 {
                        class: "text-2xl font-bold text-stone-700",
                        style: "font-family: 'Playfair Display', serif;",
                        "Visit Our Bakery"
                    }
                    p { class: "text-stone-600",
                        "Stop by for a fresh cup of coffee and a pastry. Our doors are always open with a warm welcome."
                    }

                    // Info cards
                    div { class: "space-y-4",
                        InfoCard {
                            icon: rsx! {
                                MapPin { class: "w-6 h-6 text-honey-500" }
                            },
                            title: "Address",
                            detail: "123 Baker Street, Sweet Town, ST 12345",
                        }
                        InfoCard {
                            icon: rsx! {
                                Phone { class: "w-6 h-6 text-honey-500" }
                            },
                            title: "Phone",
                            detail: "(555) 123-4567",
                        }
                        InfoCard {
                            icon: rsx! {
                                Mail { class: "w-6 h-6 text-honey-500" }
                            },
                            title: "Email",
                            detail: "hello@sweetdelights.com",
                        }
                    }

                    // Special orders
                    div { class: "bg-honey-50/50 border border-honey-200 rounded-2xl p-6",
                        div { class: "flex items-start gap-4",
                            Sparkles { class: "w-8 h-8 text-honey-500 shrink-0 mt-1" }
                            div {
                                h3 { class: "text-lg font-bold text-stone-700 mb-2",
                                    "Custom Orders Welcome!"
                                }
                                p { class: "text-stone-600 text-sm",
                                    "Planning a wedding, birthday, or special event? We create custom cakes and dessert tables tailored to your vision. Give us at least 48 hours notice for custom orders."
                                }
                            }
                        }
                    }
                }

                // Right: Hours & photo
                div { class: "space-y-8",
                    img {
                        src: BAKERY_INTERIOR,
                        alt: "Our cozy bakery",
                        class: "rounded-2xl shadow-md w-full h-64 object-cover",
                        loading: "lazy",
                    }

                    div {
                        class: "bg-base-100 shadow-sm border border-stone-200/30 rounded-2xl p-6",
                        "data-theme": "bakery",
                        div { class: "flex items-center gap-2 mb-6",
                            Clock { class: "w-6 h-6 text-honey-500" }
                            h2 { class: "text-xl font-bold text-stone-700", "Opening Hours" }
                        }
                        div { class: "space-y-3",
                            HourRow {
                                day: "Monday – Friday",
                                hours: "7:00 AM – 7:00 PM",
                            }
                            HourRow { day: "Saturday", hours: "8:00 AM – 6:00 PM" }
                            HourRow { day: "Sunday", hours: "9:00 AM – 3:00 PM" }
                        }
                        div { class: "mt-6 pt-4 border-t border-stone-200/50",
                            div { class: "flex items-center gap-2 text-honey-600",
                                Coffee { class: "w-5 h-5" }
                                span { class: "text-sm font-medium", "Coffee & tea served all day!" }
                            }
                        }
                    }

                    p { class: "text-sm text-stone-500 text-center",
                        "We're also available for catering and wholesale. Just give us a call!"
                    }
                }
            }
        }
    }
}

#[component]
fn InfoCard(icon: Element, title: String, detail: String) -> Element {
    rsx! {
        div {
            class: "flex items-start gap-4 bg-base-100 border border-stone-200/30 rounded-xl p-4",
            "data-theme": "bakery",
            div { class: "bg-cream-100/50 p-2 rounded-lg shrink-0", {icon} }
            div {
                h3 { class: "font-semibold text-stone-700", "{title}" }
                p { class: "text-stone-600 text-sm", "{detail}" }
            }
        }
    }
}

#[component]
fn HourRow(day: String, hours: String) -> Element {
    rsx! {
        div { class: "flex justify-between items-center",
            span { class: "text-stone-700 font-medium", "{day}" }
            span { class: "text-honey-700 font-semibold", "{hours}" }
        }
    }
}
