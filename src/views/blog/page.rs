use crate::Route;
use dioxus::prelude::*;
use lucide_dioxus::{ArrowRight, Calendar, User};

#[component]
pub fn Blog() -> Element {
    rsx! {
        // Blog post grid
        div { class: "max-w-7xl mx-auto px-4 py-16",
            div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8",
                for post in super::BLOG_POSTS.iter() {
                    Link {
                        to: Route::Post {
                            slug: post.slug.to_string(),
                        },
                        class: "group card bg-base-100 shadow-sm hover:shadow-md transition-all duration-300 hover:-translate-y-1 overflow-hidden border border-stone-200/40 rounded-2xl",
                        "data-theme": "bakery",

                        figure { class: "relative h-52 overflow-hidden",
                            img {
                                src: "{post.image}",
                                alt: "{post.title}",
                                class: "w-full h-full object-cover group-hover:scale-105 transition-transform duration-500",
                                loading: "lazy",
                            }
                            div { class: "absolute inset-0 bg-gradient-to-t from-stone-900/30 to-transparent" }
                        }

                        div { class: "card-body p-5",
                            div { class: "flex items-center gap-4 text-xs text-stone-500 mb-3",
                                span { class: "flex items-center gap-1",
                                    Calendar { class: "w-3.5 h-3.5" }
                                    "{post.date}"
                                }
                                span { class: "flex items-center gap-1",
                                    User { class: "w-3.5 h-3.5" }
                                    "{post.author}"
                                }
                            }

                            h3 {
                                class: "text-xl font-bold text-stone-700 group-hover:text-honey-700 transition-colors leading-snug mb-2",
                                style: "font-family: 'Playfair Display', serif;",
                                "{post.title}"
                            }

                            p { class: "text-stone-600 text-sm leading-relaxed line-clamp-2 mb-4",
                                "{post.excerpt}"
                            }

                            span {
                                class: "flex items-center gap-1 text-honey-600 text-sm font-semibold group-hover:gap-2 transition-all",
                                style: "font-family: 'Nunito', sans-serif;",
                                "Read more"
                                ArrowRight { class: "w-4 h-4" }
                            }
                        }
                    }
                }
            }
        }
    }
}
