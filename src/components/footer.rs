use crate::components::NewsletterInline;
use crate::Route;
use dioxus::prelude::*;
use lucide_dioxus::{CakeSlice, Clock, Heart, Mail, MapPin, Phone};

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "bg-stone-800",
            // Main footer
            div { class: "max-w-7xl mx-auto px-4 py-14",
                div { class: "grid grid-cols-1 md:grid-cols-4 gap-10",
                    // Brand column
                    div { class: "space-y-5",
                        div {
                            class: "flex items-center gap-2 text-2xl font-bold text-cream-50",
                            style: "font-family: 'Playfair Display', serif;",
                            CakeSlice { class: "w-8 h-8 text-honey-400" }
                            span { "Sweet Delights" }
                        }
                        p { class: "text-stone-300 text-sm leading-relaxed",
                            "Freshly baked with love since 2020. Every loaf, pastry, and cake is handcrafted with the finest ingredients — because you deserve nothing less."
                        }
                        div { class: "flex gap-3 pt-1",
                            a {
                                href: "https://instagram.com/sweetdelightsbakery",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                class: "bg-stone-700/40 hover:bg-honey-600 text-stone-300 hover:text-cream-50 p-2 rounded-full transition-all duration-300",
                                "aria-label": "Follow us on Instagram",
                                svg {
                                    class: "w-5 h-5",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    rect {
                                        width: "20",
                                        height: "20",
                                        x: "2",
                                        y: "2",
                                        rx: "5",
                                        ry: "5",
                                    }
                                    path { d: "M16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z" }
                                    line {
                                        x1: "17.5",
                                        y1: "6.5",
                                        x2: "17.51",
                                        y2: "6.5",
                                    }
                                }
                            }
                            a {
                                href: "https://facebook.com/sweetdelightsbakery",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                class: "bg-stone-700/40 hover:bg-honey-600 text-stone-300 hover:text-cream-50 p-2 rounded-full transition-all duration-300",
                                "aria-label": "Follow us on Facebook",
                                svg {
                                    class: "w-5 h-5",
                                    view_box: "0 0 24 24",
                                    fill: "currentColor",
                                    stroke: "none",
                                    path { d: "M18 2h-3a5 5 0 0 0-5 5v3H7v4h3v8h4v-8h3l1-4h-4V7a1 1 0 0 1 1-1h3z" }
                                }
                            }
                        }
                    }

                    // Quick links
                    div { class: "space-y-4",
                        h3 {
                            class: "text-lg font-bold text-cream-50 mb-2",
                            style: "font-family: 'Playfair Display', serif;",
                            "Quick Links"
                        }
                        ul { class: "space-y-2.5",
                            li {
                                Link {
                                    to: Route::Home {},
                                    class: "text-stone-300 hover:text-honey-400 transition-colors text-sm",
                                    "Home"
                                }
                            }
                            li {
                                Link {
                                    to: Route::Products {},
                                    class: "text-stone-300 hover:text-honey-400 transition-colors text-sm",
                                    "Our Products"
                                }
                            }
                            li {
                                Link {
                                    to: Route::About {},
                                    class: "text-stone-300 hover:text-honey-400 transition-colors text-sm",
                                    "About Us"
                                }
                            }
                            li {
                                Link {
                                    to: Route::Contact {},
                                    class: "text-stone-300 hover:text-honey-400 transition-colors text-sm",
                                    "Contact"
                                }
                            }
                            li {
                                Link {
                                    to: Route::Blog {},
                                    class: "text-stone-300 hover:text-honey-400 transition-colors text-sm",
                                    "From Our Kitchen"
                                }
                            }
                        }
                    }

                    // Hours
                    div { class: "space-y-4",
                        h3 {
                            class: "text-lg font-bold text-cream-50 mb-2",
                            style: "font-family: 'Playfair Display', serif;",
                            "Visit Us"
                        }
                        div { class: "space-y-3 text-sm",
                            div { class: "flex items-start gap-2",
                                MapPin { class: "w-4 h-4 mt-0.5 text-honey-400 shrink-0" }
                                span { class: "text-stone-300", "123 Baker Street, Sweet Town, ST 12345" }
                            }
                            div { class: "flex items-center gap-2",
                                Phone { class: "w-4 h-4 text-honey-400 shrink-0" }
                                span { class: "text-stone-300", "(555) 123-4567" }
                            }
                            div { class: "flex items-center gap-2",
                                Mail { class: "w-4 h-4 text-honey-400 shrink-0" }
                                span { class: "text-stone-300", "hello@sweetdelights.com" }
                            }
                            div { class: "flex items-start gap-2 pt-2",
                                Clock { class: "w-4 h-4 mt-0.5 text-honey-400 shrink-0" }
                                div { class: "text-stone-300 space-y-1",
                                    p { "Mon–Fri: 7:00 AM – 7:00 PM" }
                                    p { "Saturday: 8:00 AM – 6:00 PM" }
                                    p { "Sunday: 9:00 AM – 3:00 PM" }
                                }
                            }
                        }
                    }

                    // Newsletter
                    div { class: "space-y-4",
                        h3 {
                            class: "text-lg font-bold text-cream-50 mb-2",
                            style: "font-family: 'Playfair Display', serif;",
                            "Stay in Touch"
                        }
                        NewsletterInline {}
                    }
                }
            }

            // Bottom bar
            div { class: "border-t border-stone-700/40",
                div { class: "max-w-7xl mx-auto px-4 py-5 text-center text-xs text-stone-400",
                    p { class: "flex items-center justify-center gap-1.5",
                        "© 2026 Sweet Delights Bakery. All rights reserved."
                        Heart { class: "w-3.5 h-3.5 text-honey-500 fill-honey-500" }
                        "Made with love and butter."
                    }
                }
            }
        }
    }
}
