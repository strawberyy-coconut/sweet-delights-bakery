use crate::components::{
    DailySpecials, Footer, Hero, NewsletterSignup, Product, ProductCard,
    ScrollReveal, SparkleDivider, Testimonial, TestimonialCard, WheatDivider,
};
use dioxus::prelude::*;

const ABOUT_IMAGE: Asset = asset!("/assets/images/cookies.png", AssetOptions::image().with_preload(true));
const BUTTER_CROISSANT_IMAGE: Asset = asset!("/assets/images/butter-croissant.png");
const CLASSIC_CHOCOLATE_CAKE_IMAGE: Asset = asset!("/assets/images/classic-chocolate-cake.png");
const SOURDOUGH_BOULE_IMAGE: Asset = asset!("/assets/images/sourdough-boule.png");


const FEATURED_PRODUCTS: &[Product] = &[
    Product { name: "Butter Croissant", description: "Flaky, golden-brown croissant with 64 layers of buttery perfection. Baked fresh every morning.", price: "$4.50", image: BUTTER_CROISSANT_IMAGE, featured: true },
    Product { name: "Classic Chocolate Cake", description: "Rich, moist chocolate cake layered with velvety ganache. A celebration favorite.", price: "$42.00", image: CLASSIC_CHOCOLATE_CAKE_IMAGE, featured: true },
    Product { name: "Sourdough Boule", description: "Crusty, tangy sourdough made with our 10-year-old starter. The heart of our bakery.", price: "$7.50", image: SOURDOUGH_BOULE_IMAGE, featured: false },
];

const TESTIMONIALS: &[Testimonial] = &[
    Testimonial { quote: "The best croissants this side of Paris! Flaky, buttery, and absolutely divine. We drive 20 minutes every weekend just to get them.", author: "Sarah M.", role: "Regular Customer", rating: 5 },
    Testimonial { quote: "They made our wedding cake and it was beyond stunning — both in looks and taste. Every guest asked for the bakery's name!", author: "James & Lily", role: "Wedding Clients", rating: 5 },
    Testimonial { quote: "Their sourdough bread is a staple in our home. The crust is perfect and the inside is so soft. Can't recommend enough!", author: "David K.", role: "Local Food Blogger", rating: 5 },
];

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}

        // Daily Specials — warm chalkboard
        ScrollReveal { DailySpecials {} }

        WheatDivider {}

        // Featured Products
        ScrollReveal {
            div { class: "py-16",
                div { class: "max-w-7xl mx-auto px-4",
                    div { class: "text-center mb-12",
                        h2 {
                            class: "text-3xl md:text-4xl font-bold text-stone-700 mb-4",
                            style: "font-family: 'Playfair Display', serif;",
                            "Our Specialties"
                        }
                        SparkleDivider {}
                        p {
                            class: "text-stone-500 max-w-xl mx-auto",
                            style: "font-family: 'Nunito', sans-serif;",
                            "Handcrafted daily with love — these customer favorites keep them coming back for more."
                        }
                    }
                    div { class: "grid grid-cols-1 md:grid-cols-3 gap-8",
                        for p in FEATURED_PRODUCTS.iter() {
                            ProductCard { product: Product { ..p.clone() } }
                        }
                    }
                    div { class: "text-center mt-12",
                        Link {
                            to: crate::Route::Products {},
                            class: "btn bg-honey-600 hover:bg-honey-500 text-white border-none px-10 rounded-full text-base font-bold shadow-sm hover:shadow-md transition-all",
                            "View Our Full Menu →"
                        }
                    }
                }
            }
        }

        WheatDivider {}

        // Our Story snippet
        ScrollReveal {
            div { class: "bg-gradient-to-b from-cream-100/50 to-cream-50 py-20",
                div { class: "max-w-7xl mx-auto px-4",
                    div { class: "grid grid-cols-1 lg:grid-cols-2 gap-14 items-center",
                        img {
                            src: ABOUT_IMAGE,
                            alt: "Fresh artisan bread on a rustic wooden table",
                            class: "rounded-2xl shadow-warm-lg w-full h-[380px] object-cover",
                            loading: "lazy",
                        }
                        div { class: "space-y-5",
                            h2 {
                                class: "text-3xl md:text-4xl font-bold text-stone-700",
                                style: "font-family: 'Playfair Display', serif;",
                                "Our Story"
                            }
                            p {
                                class: "text-stone-600 leading-relaxed",
                                style: "font-family: 'Nunito', sans-serif;",
                                "Sweet Delights Bakery was founded in 2020 with a simple mission: bring the warmth and comfort of homemade bakery goods to our community. What began as weekend baking experiments in a tiny kitchen soon became a beloved neighborhood destination."
                            }
                            p {
                                class: "text-stone-600 leading-relaxed",
                                style: "font-family: 'Nunito', sans-serif;",
                                "Every recipe is crafted from scratch using traditional techniques and the finest ingredients — European butter, organic flour, and farm-fresh eggs. Because you deserve nothing less than perfection."
                            }
                            Link {
                                to: crate::Route::About {},
                                class: "btn btn-outline border-honey-600 text-stone-700 hover:bg-honey-600 hover:text-white mt-3 rounded-full font-semibold",
                                "Read More About Us"
                            }
                        }
                    }
                }
            }
        }

        WheatDivider {}

        // Testimonials
        ScrollReveal {
            div { class: "py-16",
                div { class: "max-w-7xl mx-auto px-4",
                    div { class: "text-center mb-12",
                        h2 {
                            class: "text-3xl md:text-4xl font-bold text-stone-700 mb-4",
                            style: "font-family: 'Playfair Display', serif;",
                            "What Our Customers Say"
                        }
                        SparkleDivider {}
                        p {
                            class: "text-stone-500 max-w-xl mx-auto",
                            style: "font-family: 'Nunito', sans-serif;",
                            "Don't just take our word for it — here's what our community has to say."
                        }
                    }
                    div { class: "grid grid-cols-1 md:grid-cols-3 gap-7",
                        for t in TESTIMONIALS.iter() {
                            TestimonialCard { testimonial: Testimonial { ..t.clone() } }
                        }
                    }
                }
            }
        }

        // Newsletter Signup
        NewsletterSignup {}

        // CTA Banner
        ScrollReveal {
            div { class: "bg-gradient-to-r from-honey-800 via-honey-700 to-honey-600 py-14",
                div { class: "max-w-4xl mx-auto px-4 text-center",
                    h2 {
                        class: "text-3xl md:text-5xl font-bold text-cream-50 mb-4",
                        style: "font-family: 'Playfair Display', serif;",
                        "Ready for Something Delicious?"
                    }
                    p {
                        class: "text-lg text-cream-50/90 mb-8 max-w-xl mx-auto leading-relaxed",
                        style: "font-family: 'Nunito', sans-serif;",
                        "Stop by today and let the aroma of fresh bread and warm pastries welcome you home."
                    }
                    div { class: "flex flex-wrap justify-center gap-4",
                        Link {
                            to: crate::Route::Products {},
                            class: "btn bg-honey-500 hover:bg-honey-400 text-white border-none px-10 font-bold rounded-full shadow-lg hover:shadow-xl transition-all text-base",
                            "Order Now"
                        }
                        Link {
                            to: crate::Route::Contact {},
                            class: "btn btn-outline border-cream-50/50 text-cream-50 hover:bg-cream-50/15 hover:border-cream-50 px-10 rounded-full font-semibold text-base",
                            "Find Us"
                        }
                    }
                }
            }
        }

        Footer {}
    }
}
