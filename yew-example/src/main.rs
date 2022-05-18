use yew::prelude::*;
use yew::{classes, function_component, html, use_state, Callback};

mod fakedata;

#[function_component(UseState)]
fn state() -> Html {
    let items = use_state(|| fakedata::fake_data());

    let onclick = {
        let new = items.clone();

        let mut sortable = (*new).clone();
        sortable.sort();

        Callback::from(move |_| new.set(sortable.clone()))
    };

    html! {
        <div>
            <button {onclick}>{ "Sort" }</button>

            {items.iter().map(|n| html!{
                <div class={classes!("item")} style={format!("width: {}px;", n)}>
                    {n}
                </div>
            }).collect::<Html>()}
        </div>
    }
}

fn main() {
    yew::start_app::<UseState>();
}
