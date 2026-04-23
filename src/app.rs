use dioxus_native::prelude::*;

const IMAGE: Asset = asset!("./image.png");

#[derive(Clone, Copy)]
pub struct CounterData {
    count: Signal<i32>,
}

impl CounterData {

    fn increment(&mut self) {
        self.count.set(self.count + 1);
    }
    fn decrement(&mut self) {
        self.count.set(self.count - 1);
    }
    fn reset(&mut self) {
        self.count.set(0);
    }
}



pub fn app() -> Element {
    let counter_context = use_context_provider(|| CounterData { count: Signal::new(0) });

    rsx! (
        style { {CSS} }

        div{ class:"grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4 justify-center items-center px-8 py-8",
            ProductCard{}
            ProductCard{}
            ProductCard{}
            ProductCard{}
            ProductCard{}
            ProductCard{}
            ProductCard{}
            ProductCard{}
            ProductCard{}
            ProductCard{}
        }
    )
}

#[component]
fn ProductCard() -> Element {
    rsx! (
        div{ class: "bg-gray-50 hover:shadow-gray-100 shadow-md border-1 border-gray-200 rounded-lg p-8",
            div{ class: "flex flex-col gap-8 items-center",
                img {  src: {IMAGE} }
                p { class: "font-bold text-md text-center", "Coca Cola Zero"}
            }
        }
    )
}


#[component]
fn CounterComponent() -> Element {
    let mut counter_context = use_context::<CounterData>();

    rsx! (
        div { class: "flex flex-col gap-8",
            h1 { class: "text-2xl font-bold select-none", "Count: {counter_context.count}" },
            div{ style: "flex flex-row gap-4",
                button { class: "inline-flex shrink-0 items-center justify-center text-sm font-medium whitespace-nowrap transition-all outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:pointer-events-none disabled:opacity-50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 bg-primary text-primary-foreground hover:bg-primary/90 gap-1.5 px-3 has-[>svg]:px-2.5 h-[31px] rounded-lg",
                    onclick: move |_| { counter_context.increment() },
                    "Increment",
                },
                button { class: "inline-flex shrink-0 items-center justify-center text-sm font-medium whitespace-nowrap transition-all outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:pointer-events-none disabled:opacity-50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 bg-primary text-primary-foreground hover:bg-primary/90 gap-1.5 px-3 has-[>svg]:px-2.5 h-[31px] rounded-lg",
                    onclick: move |_| { counter_context.reset() },
                    "Reset",
                },
                button { class: "inline-flex shrink-0 items-center justify-center text-sm font-medium whitespace-nowrap transition-all outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:pointer-events-none disabled:opacity-50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 bg-primary text-primary-foreground hover:bg-primary/90 gap-1.5 px-3 has-[>svg]:px-2.5 h-[31px] rounded-lg",
                    onclick: move |_| { counter_context.decrement() },
                    "Decrement",
                },
            }
        }


    )
}


const CSS: &str = include_str!("../assets/tailwind.css");
