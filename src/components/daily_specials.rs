use dioxus::prelude::*;
use lucide_dioxus::{Clock, Sparkles};

#[derive(Clone, PartialEq)]
pub struct DailySpecial {
    pub name: &'static str,
    pub description: &'static str,
    pub price: &'static str,
    pub emoji: Asset
}

const DAILY_SPECIALS: &[DailySpecial] = &[
    DailySpecial {
        name: "Lemon Lavender Scone",
        description: "Buttery scone infused with lemon zest and dried lavender, drizzled with honey glaze.",
        price: "$5.25",
        emoji: asset!("/assets/images/lemon-emoji-unicode.png"),
    },
    DailySpecial {
        name: "Strawberry Rhubarb Danish",
        description: "Flaky Danish pastry filled with sweet-tart strawberry rhubarb compote and vanilla cream.",
        price: "$5.75",
        emoji: asset!("/assets/images/strawberry-emoji-unicode.png"),
    },
    DailySpecial {
        name: "Honey Oat Milk Latte",
        description: "Smooth espresso with steamed oat milk, a touch of local honey, and a dusting of cinnamon.",
        price: "$4.95",
        emoji: asset!("/assets/images/hot-beverage-emoji-unicode.png"),
    },
];

#[component]
pub fn DailySpecials() -> Element {
    rsx! {
        div { class: "relative overflow-hidden bg-stone-800 rounded-3xl mx-4 md:mx-auto max-w-7xl my-16",
            // Subtle chalk-texture background effect via inner gradient
            div { class: "absolute inset-0 bg-gradient-to-br from-stone-700/20 via-transparent to-amber-900/10" }

            // Decorative border glow
            div { class: "absolute inset-0 rounded-3xl border border-amber-700/30" }

            div { class: "relative px-6 py-10 md:px-12 md:py-14",
                // Header
                div { class: "text-center mb-10",
                    div { class: "flex items-center justify-center gap-3 mb-3",
                        Sparkles { class: "w-6 h-6 text-amber-400" }
                        h2 {
                            class: "text-3xl md:text-4xl font-bold text-amber-50",
                            style: "font-family: 'Playfair Display', serif;",
                            "Today's Fresh Bakes"
                        }
                        Sparkles { class: "w-6 h-6 text-amber-400" }
                    }
                    p { class: "text-amber-200/80 max-w-xl mx-auto flex items-center justify-center gap-2",
                        Clock { class: "w-4 h-4" }
                        "Baked fresh this morning — while they last!"
                    }
                }

                // Specials grid
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-6",
                    for special in DAILY_SPECIALS.iter() {
                        div { class: "bg-stone-700/40 backdrop-blur-sm rounded-2xl p-6 border border-stone-600/30 hover:border-amber-600/40 transition-all duration-300 hover:bg-stone-700/60",
                            // Emoji & name
                            div { class: "flex items-start gap-3 mb-3",
                                span { class: "text-3xl shrink-0",
                                    img { src: special.emoji, alt: special.name }
                                }
                                div {
                                    h3 {
                                        class: "text-lg font-semibold text-amber-100",
                                        style: "font-family: 'Playfair Display', serif;",
                                        "{special.name}"
                                    }
                                }
                            }
                            // Description
                            p { class: "text-amber-200/60 text-sm leading-relaxed mb-4",
                                "{special.description}"
                            }
                            // Price
                            div { class: "flex items-center justify-between",
                                span { class: "text-xl font-bold text-amber-400", "{special.price}" }
                                span { class: "text-xs text-amber-500/60 bg-amber-500/10 px-2 py-1 rounded-full",
                                    "Fresh today"
                                }
                            }
                        }
                    }
                }

                // Bottom note
                p { class: "text-center text-amber-300/50 text-xs mt-8 italic",
                    "Our daily specials change with the seasons. Follow us for updates!"
                }
            }
        }
    }
}
