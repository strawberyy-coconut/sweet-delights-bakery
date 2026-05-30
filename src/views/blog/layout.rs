use crate::Route;
use dioxus::prelude::*;

/// Layout component shared by all blog pages (list + individual posts).
#[component]
pub fn BlogLayout() -> Element {
    rsx! {
        // Hero banner
        div { class: "relative h-[250px] md:h-[320px] overflow-hidden",
            img {
                src: asset!("/assets/images/cinnamon-roll.png", AssetOptions::image().with_preload(true)),
                alt: "Fresh pastries on display",
                class: "w-full h-full object-cover",
                loading: "eager",
            }
            div { class: "absolute inset-0 bg-gradient-to-r from-amber-900/60 via-stone-800/30 to-transparent" }
            div { class: "absolute inset-0 flex items-center",
                div { class: "max-w-7xl mx-auto px-4 w-full",
                    h1 {
                        class: "text-4xl md:text-5xl font-bold text-cream-50 mb-3",
                        style: "font-family: 'Playfair Display', serif;",
                        "From Our Kitchen"
                    }
                    p {
                        class: "text-lg text-white/90 max-w-lg",
                        style: "font-family: 'Nunito', sans-serif;",
                        "Stories, recipes, and behind-the-scenes moments from the heart of our bakery."
                    }
                }
            }
        }

        Outlet::<Route> {}
    }
}
