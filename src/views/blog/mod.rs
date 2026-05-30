mod post;
mod page;
mod layout;

pub use post::Post;
pub use page::Blog;
pub use layout::BlogLayout;

pub use dioxus::prelude::*;


pub struct BlogPost {
    pub title: &'static str,
    pub slug: &'static str,
    pub excerpt: &'static str,
    pub author: &'static str,
    pub date: &'static str,
    pub image: Asset,
}

pub const BLOG_POSTS: &[BlogPost] = &[
    BlogPost {
        title: "The Secret to Perfect Sourdough",
        slug: "the-secret-to-perfect-sourdough",
        excerpt: "Learn how our 10-year-old starter and patient fermentation create that signature tangy flavor and chewy crumb.",
        author: "Maria Chen",
        date: "May 25, 2026",
        image: asset!("/assets/images/sourdough-boule.png", AssetOptions::image().with_preload(true)),
    },
    BlogPost {
        title: "Spring Pastry Collection is Here!",
        slug: "spring-pastry-collection",
        excerpt: "From lemon lavender scones to strawberry rhubarb danishes — meet our seasonal lineup bursting with spring flavors.",
        author: "Sweet Delights Team",
        date: "May 18, 2026",
        image: asset!("/assets/images/spring-pastry.png", AssetOptions::image().with_preload(true)),
    },
    BlogPost {
        title: "Behind the Scenes: Wedding Cake Design",
        slug: "behind-the-scenes-wedding-cake-design",
        excerpt: "A peek into how we create stunning custom wedding cakes, from the first sketch to the final sugar flower.",
        author: "Liam Torres",
        date: "May 10, 2026",
        image: asset!("/assets/images/wedding-cake.png", AssetOptions::image().with_preload(true)),
    },
    BlogPost {
        title: "Why We Use European Butter",
        slug: "why-we-use-european-butter",
        excerpt: "Higher butterfat, richer flavor, flakier layers — discover what makes European butter a game-changer in baking.",
        author: "Maria Chen",
        date: "May 2, 2026",
        image: asset!("/assets/images/butter.png", AssetOptions::image().with_preload(true)),
    },
    BlogPost {
        title: "A Day in the Life of a Baker",
        slug: "a-day-in-the-life-of-a-baker",
        excerpt: "Wake up at 3 AM, fire up the ovens, and knead the day's first dough. Come behind the counter with us.",
        author: "David Park",
        date: "April 20, 2026",
        image: asset!("/assets/images/baker.png", AssetOptions::image().with_preload(true)),
    },
    BlogPost {
        title: "Our Guide to Bread Flour Types",
        slug: "our-guide-to-bread-flour-types",
        excerpt: "All-purpose, bread flour, whole wheat, rye — which one to use? Let's break it down simply.",
        author: "Maria Chen",
        date: "April 8, 2026",
        image: asset!("/assets/images/flour.png", AssetOptions::image().with_preload(true)),
    },
];
