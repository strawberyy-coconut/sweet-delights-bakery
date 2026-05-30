//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component  to be used in our app.

mod hero;
pub use hero::Hero;

mod footer;
pub use footer::Footer;

mod product_card;
pub use product_card::{Product, ProductCard};

mod testimonial_card;
pub use testimonial_card::{Testimonial, TestimonialCard};

mod daily_specials;
pub use daily_specials::DailySpecials;

mod newsletter;
pub use newsletter::{NewsletterInline, NewsletterSignup};

mod scroll_reveal;
pub use scroll_reveal::ScrollReveal;

mod decorative;
pub use decorative::{SparkleDivider, WheatDivider};
