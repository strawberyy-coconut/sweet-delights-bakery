use dioxus::prelude::*;
use lucide_dioxus::{Heart, Mail, Sparkles};

/// Full-width newsletter signup section for the home page.
#[component]
pub fn NewsletterSignup() -> Element {
    let mut email = use_signal(String::new);
    let mut submitted = use_signal(|| false);

    let handle_submit = move |_| {
        if !email.read().is_empty() {
            submitted.set(true);
        }
    };

    rsx! {
        div { class: "bg-gradient-to-br from-amber-50 via-cream-50 to-orange-50 py-16",
            div { class: "max-w-2xl mx-auto px-4 text-center",
                if submitted() {
                    // Success state
                    div { class: "space-y-4",
                        Heart { class: "w-12 h-12 text-amber-500 mx-auto" }
                        h2 {
                            class: "text-3xl font-bold text-stone-700",
                            style: "font-family: 'Playfair Display', serif;",
                            "You're in the family!"
                        }
                        p { class: "text-stone-600",
                            "Thank you for joining! We'll send you weekly recipes, secret menu items, and first dibs on seasonal treats."
                        }
                    }
                } else {
                    // Form state
                    div { class: "mb-6",
                        div { class: "flex items-center justify-center gap-2 mb-3",
                            Sparkles { class: "w-5 h-5 text-amber-500" }
                            h2 {
                                class: "text-3xl md:text-4xl font-bold text-stone-700",
                                style: "font-family: 'Playfair Display', serif;",
                                "Join Our Baking Family"
                            }
                            Sparkles { class: "w-5 h-5 text-amber-500" }
                        }
                        p { class: "text-stone-600 max-w-md mx-auto",
                            "Weekly recipes, secret menu items, and first dibs on seasonal treats — delivered with a sprinkle of joy."
                        }
                    }

                    div { class: "flex flex-col sm:flex-row gap-3 max-w-md mx-auto",
                        div { class: "relative flex-1",
                            Mail { class: "absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-stone-400" }
                            input {
                                class: "input input-bordered w-full pl-10 bg-white/80 border-stone-400 focus:border-amber-400 focus:ring-2 focus:ring-amber-200 rounded-full transition-all duration-300",
                                style: "font-family: 'Nunito', sans-serif;",
                                r#type: "email",
                                placeholder: "Your email address",
                                value: "{email}",
                                oninput: move |e| email.set(e.value()),
                            }
                        }
                        button {
                            class: "btn bg-amber-600 hover:bg-amber-500 text-white border-none rounded-full px-8",
                            style: "font-family: 'Nunito', sans-serif;",
                            onclick: handle_submit,
                            "Subscribe"
                        }
                    }

                    p { class: "text-xs text-stone-400 mt-4",
                        "No spam, ever. Just warm baking tips and sweet surprises. Unsubscribe anytime."
                    }
                }
            }
        }
    }
}

/// Compact inline newsletter for the footer.
#[component]
pub fn NewsletterInline() -> Element {
    let mut email = use_signal(String::new);
    let mut submitted = use_signal(|| false);

    rsx! {
        div { class: "space-y-3",
            h4 {
                class: "text-sm font-semibold text-stone-200",
                style: "font-family: 'Nunito', sans-serif;",
                "Get baking tips & specials"
            }
            if submitted() {
                p { class: "text-xs text-green-400 flex items-center gap-1",
                    Heart { class: "w-3 h-3" }
                    "You're subscribed!"
                }
            } else {
                div { class: "flex gap-2",
                    input {
                        class: "input input-sm bg-stone-700/50 border-stone-600/50 text-stone-200 placeholder:text-stone-400 rounded-full text-xs flex-1",
                        style: "font-family: 'Nunito', sans-serif;",
                        r#type: "email",
                        placeholder: "your@email.com",
                        value: "{email}",
                        oninput: move |e| email.set(e.value()),
                    }
                    button {
                        class: "btn btn-xs bg-amber-600 hover:bg-amber-500 text-white border-none rounded-full",
                        onclick: move |_| {
                            if !email.read().is_empty() {
                                submitted.set(true);
                            }
                        },
                        "Join"
                    }
                }
            }
        }
    }
}
