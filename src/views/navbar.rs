use crate::Route;
use dioxus::prelude::*;
use lucide_dioxus::{CakeSlice, Sparkles};

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div { class: "sticky top-0 z-50",
            // DaisyUI collapse with hidden checkbox — responsive nav with no JS
            div { class: "max-lg:collapse rounded-none bg-stone-800 shadow-sm w-full",
                input {
                    id: "navbar-toggle",
                    class: "peer hidden",
                    "type": "checkbox",
                }
                // Backdrop: tap outside to close
                label {
                    "for": "navbar-toggle",
                    class: "fixed inset-0 hidden max-lg:peer-checked:block",
                }
                div { class: "collapse-title rounded-none navbar bg-gradient-to-r from-stone-800 to-stone-700 text-cream-50 shadow-lg  border-stone-600/40 min-h-16",
                    div { class: "navbar-start",
                        // Mobile hamburger
                        label {
                            "for": "navbar-toggle",
                            class: "btn btn-ghost lg:hidden text-cream-50 hover:bg-stone-700",
                            "aria-label": "Open navigation menu",
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                class: "h-5 w-5",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke: "currentColor",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M4 6h16M4 12h8m-8 6h16",
                                }
                            }
                        }

                        // Logo
                        Link {
                            to: Route::Home {},
                            class: "btn btn-ghost text-xl font-bold text-cream-50 hover:bg-stone-700 gap-2 px-4",
                            CakeSlice { class: "w-7 h-7 text-honey-400" }
                            span {
                                class: "hidden sm:inline",
                                style: "font-family: 'Playfair Display', serif;",
                                "Sweet Delights"
                            }
                            span { class: "hidden md:inline-flex items-center gap-0.5 bg-honey-600/30 text-honey-300 text-[10px] font-semibold px-2 py-0.5 rounded-full ml-1",
                                Sparkles { class: "w-2.5 h-2.5" }
                                "Fresh Daily"
                            }
                        }
                    }

                    // Desktop nav links
                    div { class: "navbar-center hidden lg:flex",
                        ul { class: "menu menu-horizontal px-1 gap-1",
                            li {
                                Link {
                                    to: Route::Home {},
                                    class: "text-cream-50/80 hover:bg-stone-700 hover:text-cream-50 active:bg-stone-600 rounded-lg font-medium",
                                    "Home"
                                }
                            }
                            li {
                                Link {
                                    to: Route::Products {},
                                    class: "text-cream-50/80 hover:bg-stone-700 hover:text-cream-50 active:bg-stone-600 rounded-lg font-medium",
                                    "Products"
                                }
                            }
                            li {
                                Link {
                                    to: Route::About {},
                                    class: "text-cream-50/80 hover:bg-stone-700 hover:text-cream-50 active:bg-stone-600 rounded-lg font-medium",
                                    "About"
                                }
                            }
                            li {
                                Link {
                                    to: Route::Contact {},
                                    class: "text-cream-50/80 hover:bg-stone-700 hover:text-cream-50 active:bg-stone-600 rounded-lg font-medium",
                                    "Contact"
                                }
                            }
                            li {
                                Link {
                                    to: Route::Blog {},
                                    class: "text-cream-50/80 hover:bg-stone-700 hover:text-cream-50 active:bg-stone-600 rounded-lg font-medium",
                                    "Blog"
                                }
                            }
                        }
                    }

                    // Right side: CTA
                    div { class: "navbar-end hidden lg:flex",
                        Link {
                            to: Route::Contact {},
                            class: "btn btn-sm bg-honey-600 hover:bg-honey-500 text-white border-none mr-2 rounded-full shadow-sm",
                            "Get in Touch"
                        }
                    }
                }

                // Mobile menu — visible when collapse is open
                div { class: "collapse-content lg:hidden bg-stone-800 border-t border-stone-700/50 shadow-lg",
                    ul { class: "menu p-4 gap-1",
                        li {
                            Link {
                                to: Route::Home {},
                                class: "text-cream-50/80 hover:bg-stone-700 hover:text-cream-50 rounded-lg font-medium",
                                "Home"
                            }
                        }
                        li {
                            Link {
                                to: Route::Products {},
                                class: "text-cream-50/80 hover:bg-stone-700 hover:text-cream-50 rounded-lg font-medium",
                                "Products"
                            }
                        }
                        li {
                            Link {
                                to: Route::About {},
                                class: "text-cream-50/80 hover:bg-stone-700 hover:text-cream-50 rounded-lg font-medium",
                                "About"
                            }
                        }
                        li {
                            Link {
                                to: Route::Contact {},
                                class: "text-cream-50/80 hover:bg-stone-700 hover:text-cream-50 rounded-lg font-medium",
                                "Contact"
                            }
                        }
                        li {
                            Link {
                                to: Route::Blog {},
                                class: "text-cream-50/80 hover:bg-stone-700 hover:text-cream-50 rounded-lg font-medium",
                                "Blog"
                            }
                        }
                    }
                }
            }
        }

        Outlet::<Route> {}
    }
}
