mod post;
mod page;
mod layout;

pub use post::Post;
pub use page::Blog;
pub use layout::BlogLayout;

pub use dioxus::prelude::*;


pub const BLOG_POSTS: &[(&str, &str, &str, &str, Asset)] = &[
    (
        "The Secret to Perfect Sourdough",
        "Learn how our 10-year-old starter and patient fermentation create that signature tangy flavor and chewy crumb.",
        "Maria Chen",
        "May 25, 2026",
        asset!("/assets/images/sourdough-boule.png", AssetOptions::image().with_preload(true)),
    ),
    (
        "Spring Pastry Collection is Here!",
        "From lemon lavender scones to strawberry rhubarb danishes — meet our seasonal lineup bursting with spring flavors.",
        "Sweet Delights Team",
        "May 18, 2026",
        asset!("/assets/images/spring-pastry.png", AssetOptions::image().with_preload(true)),
    ),
    (
        "Behind the Scenes: Wedding Cake Design",
        "A peek into how we create stunning custom wedding cakes, from the first sketch to the final sugar flower.",
        "Liam Torres",
        "May 10, 2026",
        asset!("/assets/images/wedding-cake.png", AssetOptions::image().with_preload(true)),
    ),
    (
        "Why We Use European Butter",
        "Higher butterfat, richer flavor, flakier layers — discover what makes European butter a game-changer in baking.",
        "Maria Chen",
        "May 2, 2026",
        asset!("/assets/images/butter.png", AssetOptions::image().with_preload(true)),
    ),
    (
        "A Day in the Life of a Baker",
        "Wake up at 3 AM, fire up the ovens, and knead the day's first dough. Come behind the counter with us.",
        "David Park",
        "April 20, 2026",
        asset!("/assets/images/baker.png", AssetOptions::image().with_preload(true)),
    ),
    (
        "Our Guide to Bread Flour Types",
        "All-purpose, bread flour, whole wheat, rye — which one to use? Let's break it down simply.",
        "Maria Chen",
        "April 8, 2026",
        asset!("/assets/images/flour.png", AssetOptions::image().with_preload(true)),
    ),
];
