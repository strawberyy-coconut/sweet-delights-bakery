use dioxus::prelude::*;
use lucide_dioxus::{Heart, Leaf, Wheat};

const ABOUT_HERO: Asset = asset!("/assets/images/cookies.png", AssetOptions::image().with_preload(true));
const INTERIOR: Asset = asset!("/assets/images/interior.png", AssetOptions::image().with_preload(true));

#[component]
pub fn About() -> Element {
    rsx! {
        // Hero banner
        div { class: "relative h-[300px] md:h-[400px] overflow-hidden",
            img {
                src: ABOUT_HERO,
                alt: "Fresh artisan bread on a rustic table",
                class: "w-full h-full object-cover",
                loading: "eager",
            }
            div { class: "absolute inset-0 bg-gradient-to-r from-amber-900/60 via-stone-800/30 to-transparent" }
            div { class: "absolute inset-0 flex items-center",
                div { class: "max-w-7xl mx-auto px-4 w-full",
                    h1 {
                        class: "text-4xl md:text-6xl font-bold text-cream-50",
                        style: "font-family: 'Playfair Display', serif;",
                        "Our Story"
                    }
                    p {
                        class: "text-xl text-cream-50/90 mt-2",
                        style: "font-family: 'Nunito', sans-serif;",
                        "A journey of flour, butter, and love"
                    }
                }
            }
        }

        // Story section
        div { class: "max-w-7xl mx-auto px-4 py-16",
            div { class: "grid grid-cols-1 lg:grid-cols-2 gap-12 items-center mb-20",
                div { class: "space-y-6",
                    h2 {
                        class: "text-3xl md:text-4xl font-bold text-stone-700",
                        style: "font-family: 'Playfair Display', serif;",
                        "From a Small Kitchen to Your Table"
                    }
                    div { class: "space-y-4 text-stone-600 leading-relaxed",
                        p {
                            "Sweet Delights Bakery was born in 2020 from a simple dream: to share the joy of "
                            "homemade baked goods with our community. What started as small batches from our home "
                            "kitchen quickly grew into a beloved neighborhood bakery."
                        }
                        p {
                            "Our founder, Maria Chen, spent years perfecting recipes passed down through generations. "
                            "Combining traditional techniques with modern flavors, she created a menu that honors the "
                            "past while embracing the new."
                        }
                        p {
                            "Today, our team of skilled bakers arrives before dawn each day to craft fresh breads, "
                            "pastries, and cakes from scratch. We believe in the power of simple, quality ingredients "
                            "and the magic they create when handled with care."
                        }
                    }
                }
                img {
                    src: INTERIOR,
                    alt: "Our bakery interior",
                    class: "rounded-2xl shadow-md w-full h-[400px] object-cover",
                    loading: "lazy",
                }
            }

            // Values grid
            h2 {
                class: "text-3xl md:text-4xl font-bold text-stone-700 text-center mb-10",
                style: "font-family: 'Playfair Display', serif;",
                "What We Stand For"
            }
            div { class: "grid grid-cols-1 md:grid-cols-3 gap-8 mb-20",
                // Value 1
                div {
                    class: "card card-warm-hover bg-base-100 shadow-sm border border-stone-200/40 rounded-2xl",
                    "data-theme": "bakery",
                    div { class: "card-body items-center text-center p-8",
                        Wheat { class: "w-12 h-12 text-honey-500 mb-4" }
                        h3 {
                            class: "card-title text-xl font-bold text-stone-700",
                            style: "font-family: 'Playfair Display', serif;",
                            "Quality Ingredients"
                        }
                        p { class: "text-stone-600 text-sm mt-2",
                            "We source organic flour, European butter, farm-fresh eggs, and seasonal produce. No shortcuts, no preservatives — just real food."
                        }
                    }
                }

                // Value 2
                div {
                    class: "card bg-base-100 shadow-sm border border-stone-200/30",
                    "data-theme": "bakery",
                    div { class: "card-body items-center text-center p-8",
                        Heart { class: "w-12 h-12 text-honey-500 mb-4" }
                        h3 {
                            class: "card-title text-xl font-bold text-stone-700",
                            style: "font-family: 'Playfair Display', serif;",
                            "Made with Love"
                        }
                        p { class: "text-stone-600 text-sm mt-2",
                            "Every batch is handcrafted by our talented bakers who pour their hearts into every loaf, pastry, and cake."
                        }
                    }
                }

                // Value 3
                div {
                    class: "card bg-base-100 shadow-sm border border-stone-200/30",
                    "data-theme": "bakery",
                    div { class: "card-body items-center text-center p-8",
                        Leaf { class: "w-12 h-12 text-honey-500 mb-4" }
                        h3 {
                            class: "card-title text-xl font-bold text-stone-700",
                            style: "font-family: 'Playfair Display', serif;",
                            "Sustainable Practices"
                        }
                        p { class: "text-stone-600 text-sm mt-2",
                            "We minimize waste through day-old donations, compostable packaging, and partnerships with local farms."
                        }
                    }
                }
            }
        }
    }
}
