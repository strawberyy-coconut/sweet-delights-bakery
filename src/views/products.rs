use crate::components::{Product, ProductCard};
use dioxus::prelude::*;

const BUTTER_CROISSANT_IMAGE: Asset = asset!("/assets/images/butter-croissant.png");
const CLASSIC_CHOCOLATE_CAKE_IMAGE: Asset = asset!("/assets/images/classic-chocolate-cake.png");
const SOURDOUGH_BOULE_IMAGE: Asset = asset!("/assets/images/sourdough-boule.png");
const VANILLA_BEAN_DREAM_IMAGE: Asset = asset!("/assets/images/vanilla-bean-dream.png");
const CARROT_WALNUT_CAKE_IMAGE: Asset = asset!("/assets/images/carrot-walnut-cake.png");
const ALMOND_DANISH_IMAGE: Asset = asset!("/assets/images/almond-danish.png");
const CINNAMON_ROLL_IMAGE: Asset = asset!("/assets/images/cinnamon-roll.png");
const CIABATTA_IMAGE: Asset = asset!("/assets/images/ciabatta.png");
const BRIOCHE_LOAF_IMAGE: Asset = asset!("/assets/images/brioche-loaf.png");
const CHOCOLATE_CHIP_COOKIES_IMAGE: Asset = asset!("/assets/images/cookies.png");
const MACARONS_IMAGE: Asset = asset!("/assets/images/macarons.png");
const SHORTBREAD_BISCUITS_IMAGE: Asset = asset!("/assets/images/shortbread-biscuits.png");

const PRODUCTS: &[Product] = &[
    // Cakes
    Product { name: "Classic Chocolate Cake", description: "Rich, moist chocolate cake layered with velvety ganache and topped with chocolate shavings.", price: "$42.00", image: CLASSIC_CHOCOLATE_CAKE_IMAGE, featured: true },
    Product { name: "Vanilla Bean Dream", description: "Light and fluffy vanilla sponge with fresh berries and whipped cream frosting.", price: "$38.00", image: VANILLA_BEAN_DREAM_IMAGE, featured: false },
    Product { name: "Carrot Walnut Cake", description: "Spiced carrot cake with crunchy walnuts and cream cheese frosting.", price: "$40.00", image: CARROT_WALNUT_CAKE_IMAGE, featured: false },
    // Pastries
    Product { name: "Butter Croissant", description: "Flaky, golden-brown croissant with 64 layers of buttery perfection.", price: "$4.50", image: BUTTER_CROISSANT_IMAGE, featured: true },
    Product { name: "Almond Danish", description: "Danish pastry filled with almond cream and topped with sliced almonds.", price: "$5.25", image: ALMOND_DANISH_IMAGE, featured: false },
    Product { name: "Cinnamon Roll", description: "Soft, swirled cinnamon roll drizzled with cream cheese glaze.", price: "$4.75", image: CINNAMON_ROLL_IMAGE, featured: true },
    // Bread
    Product { name: "Sourdough Boule", description: "Crusty, tangy sourdough made with our 10-year-old starter.", price: "$7.50", image: SOURDOUGH_BOULE_IMAGE, featured: false },
    Product { name: "Ciabatta", description: "Italian white bread with an airy, open crumb and crisp crust.", price: "$6.00", image: CIABATTA_IMAGE, featured: false },
    Product { name: "Brioche Loaf", description: "Enriched, buttery brioche — perfect for French toast or on its own.", price: "$8.00", image: BRIOCHE_LOAF_IMAGE, featured: false },
    // Cookies
    Product { name: "Chocolate Chip Cookies", description: "Classic chewy cookies packed with dark chocolate chunks.", price: "$2.50", image: CHOCOLATE_CHIP_COOKIES_IMAGE, featured: true },
    Product { name: "Macarons", description: "Delicate almond meringue cookies in assorted flavors (box of 6).", price: "$18.00", image: MACARONS_IMAGE, featured: false },
    Product { name: "Shortbread Biscuits", description: "Buttery, crumbly Scottish shortbread — a timeless treat.", price: "$3.00", image: SHORTBREAD_BISCUITS_IMAGE, featured: false },
];

#[component]
pub fn Products() -> Element {
    rsx! {
        // Warm intro hero
        div { class: "bg-gradient-to-r from-honey-700 to-honey-600 py-14",
            div { class: "max-w-7xl mx-auto px-4 text-center",
                h1 {
                    class: "text-4xl md:text-5xl font-bold text-cream-50 mb-4",
                    style: "font-family: 'Playfair Display', serif;",
                    "Our Products"
                }
                p {
                    class: "text-lg text-cream-50/90 max-w-2xl mx-auto",
                    style: "font-family: 'Nunito', sans-serif;",
                    "Every item is handcrafted with passion and the finest ingredients. From classic favorites to seasonal specials, there's something for every craving."
                }
            }
        }

        div { class: "max-w-7xl mx-auto px-4 py-16",
            // Section header
            div { class: "text-center mb-4",
                p {
                    class: "text-honey-600 text-sm font-semibold uppercase tracking-widest mb-1",
                    style: "font-family: 'Nunito', sans-serif;",
                    "Fresh from the oven"
                }
                h2 {
                    class: "text-3xl font-bold text-stone-700",
                    style: "font-family: 'Playfair Display', serif;",
                    "Our Menu"
                }
            }

            // Cakes section
            Section { title: "🎂 Cakes" }
            div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 mb-16",
                for p in PRODUCTS[..3].iter() {
                    ProductCard { product: Product { ..p.clone() } }
                }
            }

            // Pastries section
            Section { title: "🥐 Pastries" }
            div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 mb-16",
                for p in PRODUCTS[3..6].iter() {
                    ProductCard { product: Product { ..p.clone() } }
                }
            }

            // Bread section
            Section { title: "🍞 Bread" }
            div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 mb-16",
                for p in PRODUCTS[6..9].iter() {
                    ProductCard { product: Product { ..p.clone() } }
                }
            }

            // Cookies section
            Section { title: "🍪 Cookies & Treats" }
            div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6",
                for p in PRODUCTS[9..].iter() {
                    ProductCard { product: Product { ..p.clone() } }
                }
            }
        }
    }
}

#[component]
fn Section(title: String) -> Element {
    rsx! {
        div { class: "flex items-center gap-4 mb-6",
            h2 {
                class: "text-2xl font-bold text-stone-700",
                style: "font-family: 'Playfair Display', serif;",
                "{title}"
            }
            div { class: "flex-1 h-px bg-stone-200" }
        }
    }
}
