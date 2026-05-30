use crate::components::WheatDivider;
use crate::Route;
use dioxus::prelude::*;

const HERO_IMAGE: Asset = asset!("/assets/images/cinnamon-roll.png", AssetOptions::image().with_preload(true));

#[component]
pub fn Hero() -> Element {
    rsx! {
        div { class: "relative h-screen overflow-hidden",
            // Parallax background image
            img {
                src: HERO_IMAGE,
                alt: "Warm bakery counter with fresh pastries",
                class: "w-full h-full object-cover",
                loading: "eager",
                style: "will-change: transform;",
            }
            // Warm caramel overlay
            div { class: "absolute inset-0 bg-gradient-to-r from-amber-950/70 via-amber-900/40 to-transparent" }
            div { class: "absolute inset-0 bg-gradient-to-t from-stone-900/30 via-transparent to-transparent" }

            // Steam particles
            div { class: "absolute bottom-0 left-0 right-0 h-24 overflow-hidden",
                div { class: "steam-particle" }
                div { class: "steam-particle" }
                div { class: "steam-particle" }
                div { class: "steam-particle" }
                div { class: "steam-particle" }
            }

            // Fade-out to page background
            div { class: "absolute inset-x-0 bottom-0 h-48 bg-gradient-to-t from-cream-50 to-transparent" }

            // Content
            div { class: "absolute inset-0 flex items-center",
                div { class: "max-w-7xl mx-auto px-4 w-full",
                    div { class: "max-w-xl",
                        // Est. badge
                        div { class: "flex items-center gap-2 text-honey-400 mb-5",
                            span { class: "w-8 h-0.5 bg-honey-400 inline-block" }
                            span {
                                class: "text-sm font-semibold uppercase tracking-[0.2em]",
                                style: "font-family: 'Nunito', sans-serif;",
                                "Est. 2020"
                            }
                        }
                        h1 {
                            class: "text-5xl md:text-7xl font-bold text-cream-50 mb-5 leading-tight",
                            style: "font-family: 'Playfair Display', serif;",
                            "Fresh from the Oven,"
                            br {}
                            "Baked with Love"
                        }
                        p {
                            class: "text-lg md:text-xl text-cream-50/90 mb-8 max-w-lg leading-relaxed",
                            style: "font-family: 'Nunito', sans-serif;",
                            "The aroma of freshly baked bread. The warmth of butter croissants. The joy of handmade pastries — crafted daily with the finest ingredients."
                        }
                        div { class: "flex flex-wrap gap-4",
                            Link {
                                to: Route::Products {},
                                class: "btn bg-honey-600 hover:bg-honey-500 text-white border-none px-9 py-3 text-base font-bold rounded-full transition-all shadow-lg shadow-amber-900/30 hover:shadow-amber-800/40",
                                "Browse Our Menu"
                            }
                            Link {
                                to: Route::Contact {},
                                class: "btn btn-outline border-cream-50/40 text-cream-50 hover:bg-cream-50/15 hover:border-cream-50/60 px-9 py-3 text-base font-semibold rounded-full",
                                "Visit Us Today"
                            }
                        }
                    }
                }
            }
        }
        WheatDivider {}
    }
}
