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

        div { class: "product-grid",
            ProductComponent {}
            ProductComponent {}
            ProductComponent {}
            ProductComponent {}
            ProductComponent {}
            ProductComponent {}
        }
    )
}

#[component]
fn ProductComponent() -> Element {
    rsx! {
        div{ style: "display: flex; flex-direction: column; gap: 12px; justify-content: center; align-items: center; border: 1px solid ",
            img { src: IMAGE, style: "width: 300px; height: auto;" }
        }
    }
}


#[component]
fn CounterComponent() -> Element {
    let mut counter_context = use_context::<CounterData>();
    rsx! (
        div { style: "display: flex; flex-direction: column; gap: 8px; margin: 8px;",
            h1 { class: "header", "Count: {counter_context.count}" },
            div{ style: "display: flex; flex-direction: row; gap: 8px;",
                button { class: "button",
                    onclick: move |_| { counter_context.increment() },
                    "Increment",
                },
                button { class: "button",
                    onclick: move |_| { counter_context.reset() },
                    "Reset",
                },
                button { class: "button",
                    onclick: move |_| { counter_context.decrement() },
                    "Decrement",
                },
            }
        }


    )
}


const CSS: &str = include_str!("./style.css");
