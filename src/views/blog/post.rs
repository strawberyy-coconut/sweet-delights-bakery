use crate::Route;
use dioxus::prelude::*;
use lucide_dioxus::{ArrowLeft, ArrowRight, Calendar, Heart, User};

#[component]
pub fn Post(slug: String) -> Element {
    let posts = super::BLOG_POSTS;
    let idx = posts.iter().position(|p| p.slug == slug).unwrap_or(0);
    let post = &posts[idx];

    rsx! {
        // Blog post
        div { class: "max-w-4xl mx-auto px-4 py-12",
            // Meta info
            div { class: "flex flex-wrap items-center gap-4 text-sm text-stone-500 mb-6",
                span { class: "flex items-center gap-1",
                    Calendar { class: "w-4 h-4" }
                    "{post.date}"
                }
                span { class: "flex items-center gap-1",
                    User { class: "w-4 h-4" }
                    "{post.author}"
                }
                span { class: "flex items-center gap-1",
                    Heart { class: "w-4 h-4 text-honey-500" }
                    "Post #{(idx + 1)}"
                }
            }

            // Title
            h2 {
                class: "text-3xl md:text-4xl font-bold text-stone-700 mb-6",
                style: "font-family: 'Playfair Display', serif;",
                "{post.title}"
            }

            // Content
            div { class: "prose prose-stone max-w-none text-stone-600 leading-relaxed space-y-4 mb-12",
                p { "{post.excerpt}" }
                p {
                    "At Sweet Delights Bakery, we believe every recipe tells a story. Whether it's the crackle of a freshly baked "
                    "sourdough crust or the delicate layers of a butter croissant, our blog is where we share the passion, "
                    "techniques, and love that go into everything we make."
                }
                p {
                    "Stay tuned for more updates, seasonal specials, and baking tips from our team. Have a topic you'd like "
                    "us to cover? Stop by the bakery and let us know — we'd love to hear from you!"
                }
            }

            // Navigation
            div { class: "flex items-center justify-between border-t border-stone-200 pt-8",
                Link {
                    to: Route::Post {
                        slug: get_adjacent_slug(posts, idx, -1).to_string(),
                    },
                    class: "btn btn-outline border-stone-200 text-honey-700 hover:bg-honey-600 hover:text-white hover:border-honey-600 gap-2 rounded-full",
                    ArrowLeft { class: "w-4 h-4" }
                    "Previous Post"
                }
                Link {
                    to: Route::Home {},
                    class: "text-honey-600 hover:text-honey-700 text-sm underline underline-offset-2",
                    "Back to Home"
                }
                Link {
                    to: Route::Post {
                        slug: get_adjacent_slug(posts, idx, 1).to_string(),
                    },
                    class: "btn btn-outline border-stone-200 text-honey-700 hover:bg-honey-600 hover:text-white hover:border-honey-600 gap-2 rounded-full",
                    "Next Post"
                    ArrowRight { class: "w-4 h-4" }
                }
            }
        }
    }
}

fn get_adjacent_slug(posts: &[super::BlogPost], current: usize, delta: isize) -> &'static str {
    let len = posts.len();
    let next = (current as isize + delta).rem_euclid(len as isize) as usize;
    posts[next].slug
}
