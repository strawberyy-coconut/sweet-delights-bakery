use dioxus::prelude::*;
use lucide_dioxus::Star;

#[derive(Clone, PartialEq)]
pub struct Testimonial {
    pub quote: &'static str,
    pub author: &'static str,
    pub role: &'static str,
    pub rating: u8,
}

#[component]
pub fn TestimonialCard(testimonial: ReadSignal<Testimonial>) -> Element {
    let t = testimonial();

    // Compute star classes before rsx!
    let star_classes: [&str; 5] = std::array::from_fn(|i| {
        if (i as u8) < t.rating {
            "w-4 h-4 text-honey-500 fill-honey-500"
        } else {
            "w-4 h-4 text-stone-200"
        }
    });

    rsx! {
        div {
            class: "card card-warm-hover bg-base-100 shadow-sm border border-stone-200/40 rounded-2xl",
            "data-theme": "bakery",
            div { class: "card-body p-6",
                // Decorative quote mark
                div {
                    class: "text-5xl leading-none text-honey-200 mb-1",
                    style: "font-family: 'Playfair Display', serif;",
                    "“"
                }

                // Stars
                div { class: "flex gap-0.5 mb-3",
                    for class in star_classes {
                        Star { class: "{class}" }
                    }
                }

                // Quote
                p {
                    class: "text-stone-600 leading-relaxed text-sm mb-5",
                    style: "font-family: 'Nunito', sans-serif;",
                    "{t.quote}"
                }

                // Author with warm gradient avatar
                div { class: "flex items-center gap-3 mt-auto pt-4 border-t border-stone-100",
                    div { class: "avatar placeholder",
                        div { class: "rounded-full w-10 h-10 flex items-center justify-center font-bold text-sm bg-gradient-to-br from-honey-500 to-caramel-500 text-cream-50 shadow-sm",
                            {t.author.chars().next().map(|c| c.to_string()).unwrap_or_default()}
                        }
                    }
                    div {
                        p {
                            class: "font-semibold text-stone-700 text-sm",
                            style: "font-family: 'Playfair Display', serif;",
                            "{t.author}"
                        }
                        p { class: "text-stone-400 text-xs", "{t.role}" }
                    }
                }
            }
        }
    }
}
