use crate::Route;
use dioxus::prelude::*;
use lucide_dioxus::{ShoppingBag, Star};

#[derive(Clone, PartialEq)]
pub struct Product {
    pub name: &'static str,
    pub description: &'static str,
    pub price: &'static str,
    pub image: Asset,
    pub featured: bool,
}

#[component]
pub fn ProductCard(product: ReadSignal<Product>) -> Element {
    let product = product();

    rsx! {
        div {
            class: "card card-warm-hover bg-base-100 shadow-sm border border-stone-200/40 overflow-hidden rounded-2xl",
            "data-theme": "bakery",

            // Image
            figure { class: "relative h-52 overflow-hidden",
                img {
                    src: "{product.image}",
                    alt: "{product.name}",
                    class: "w-full h-full object-cover hover:scale-110 transition-transform duration-700",
                    loading: "lazy",
                }
                if product.featured {
                    div { class: "absolute top-3 right-3 bg-honey-500 text-stone-800 text-xs font-bold px-3 py-1.5 rounded-full flex items-center gap-1 shadow-lg",
                        Star { class: "w-3 h-3 fill-stone-800" }
                        span { "Baker's Pick" }
                    }
                }
                // Warm gradient overlay at bottom of image
                div { class: "absolute inset-x-0 bottom-0 h-16 bg-gradient-to-t from-base-100 to-transparent" }
            }

            // Body
            div { class: "card-body p-5",
                h3 {
                    class: "card-title text-lg font-bold text-stone-700",
                    style: "font-family: 'Playfair Display', serif;",
                    "{product.name}"
                }
                p { class: "text-sm text-stone-500 leading-relaxed line-clamp-2 mb-1",
                    "{product.description}"
                }
                div { class: "flex items-center justify-between mt-3 pt-3 border-t border-stone-200/50",
                    span {
                        class: "text-2xl font-bold text-honey-600",
                        style: "font-family: 'Nunito', sans-serif;",
                        "{product.price}"
                    }
                    Link {
                        to: Route::Contact {},
                        class: "btn btn-sm bg-honey-600 hover:bg-honey-500 text-cream-50 border-none rounded-full shadow-none gap-1.5",
                        ShoppingBag { class: "w-3.5 h-3.5" }
                        "Order"
                    }
                }
            }
        }
    }
}
