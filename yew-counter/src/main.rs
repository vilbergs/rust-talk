use yew::{function_component, html, use_state, Callback};

#[function_component(Counter)]
fn state() -> Html {
    let count = use_state(|| 0);

    let increment = {
        let count = count.clone();
        Callback::from(move |_| count.set(*count + 1))
    };

    html! {
        <>
            <div>{ *count }</div>
            <button onclick={increment}>{ "Increment" }</button>
        </>
    }
}

fn main() {
    yew::start_app::<Counter>();
}
