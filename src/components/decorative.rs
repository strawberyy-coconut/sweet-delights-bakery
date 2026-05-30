use dioxus::prelude::*;

/// A warm wheat-stalk decorative divider SVG. Use between sections for bakery flair.
#[component]
pub fn WheatDivider() -> Element {
    rsx! {
        div { class: "flex items-center justify-center gap-4 py-6",
            span { class: "h-px flex-1 bg-stone-300/50" }
            svg {
                class: "w-8 h-8 text-amber-600 shrink-0",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.5",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                // Wheat stalk glyph (simplified)
                path { d: "M12 4v16" }
                path { d: "M12 4c-2 0-4 4-4 6s2 4 4 4" }
                path { d: "M12 4c2 0 4 4 4 6s-2 4-4 4" }
                path { d: "M8 8c-1 0-3 2-3 3s2 3 3 3" }
                path { d: "M16 8c1 0 3 2 3 3s-2 3-3 3" }
            }
            span { class: "h-px flex-1 bg-stone-300/50" }
        }
    }
}

/// A small decorative star/sparkle to sprinkle warmth throughout the site.
#[component]
pub fn SparkleDivider() -> Element {
    rsx! {
        div { class: "flex items-center justify-center gap-3 py-4",
            span { class: "w-2 h-2 rounded-full bg-amber-400" }
            span { class: "w-1.5 h-1.5 rounded-full bg-amber-300" }
            span { class: "w-2 h-2 rounded-full bg-amber-400" }
        }
    }
}
