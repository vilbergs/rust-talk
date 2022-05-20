use web_sys::PerformanceEntry;
use yew::prelude::*;
use yew::{classes, function_component, html, use_state, Callback};

mod fakedata;

#[function_component(UseState)]
fn state() -> Html {
    let items = use_state(|| fakedata::fake_data());
    let sum = use_state(|| 0);
    let time = use_state(|| 0.0);

    let window = web_sys::window().unwrap();

    let perf = window.performance().unwrap();

    let onnew = {
        let items = items.clone();
        Callback::from(move |_| items.set(fakedata::fake_data()))
    };

    let onsort = {
        let items = items.clone();
        let sum = sum.clone();
        let time = time.clone();

        perf.mark("start").unwrap();
        let total = (*items).iter().sum();
        perf.mark("end").unwrap();

        Callback::from(move |_| {
            perf.measure_with_start_mark_and_end_mark("sort-speed", "start", "end")
                .unwrap();
            sum.set(total);

            let js_val = perf.get_entries_by_name("sort-speed").get(0);
            let entry = PerformanceEntry::from(js_val);

            time.set(entry.duration());

            perf.clear_measures();
        })
    };

    html! {
        <div>
        <button onclick={onnew}>{ "New" }</button>
            <button onclick={onsort}>{ "Sort" }</button>
            <div>{*sum}</div>
            <div>{*time}</div>
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
