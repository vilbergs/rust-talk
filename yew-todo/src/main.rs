use std::cmp::Ordering;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use yew::{classes, function_component, html, use_state, Callback};

mod todo;

use todo::TodoItem;

#[derive(Clone)]
struct Todo {
    id: usize,
    name: String,
    is_complete: bool,
}

#[function_component(Todos)]
fn state() -> Html {
    let todos = use_state(|| {
        vec![Todo {
            id: 0,
            name: "Buy Cat food".to_owned(),
            is_complete: false,
        }]
    });

    let add_todo = {
        let todos = todos.clone();

        Callback::from(move |event: KeyboardEvent| {
            let mut new_todos = (*todos).clone();

            if event.key_code() == 13 {
                let target: EventTarget = event
                    .target()
                    .expect("Event should have a target when dispatched");
                let value = target.unchecked_into::<HtmlInputElement>().value();
                // Events can bubble so this listener might catch events from child
                // elements which are not of type HtmlInputElement

                new_todos.push(Todo {
                    id: new_todos.len(),
                    name: value,
                    is_complete: false,
                });

                todos.set(new_todos);
            }
        })
    };

    let ontoggle = {
        let todos = todos.clone();

        Callback::from(move |id: usize| {
            let mut new_todos = (*todos).clone();

            let old_todo = new_todos[id].clone();

            new_todos[id] = Todo {
                id: old_todo.id,
                name: old_todo.name,
                is_complete: !old_todo.is_complete,
            };

            todos.set(new_todos);
        })
    };

    html! {
        <div class={classes!("wrapper")}>
        <input class={classes!("input")} type="text" onkeyup={add_todo}/>
            {todos.iter().map(|todo| html!{
                    <TodoItem
                        id={todo.id}
                        name={todo.name.clone()}
                        is_complete={todo.is_complete}
                        ontoggle={ontoggle.clone()}
                    />
            }).collect::<Html>()}

        </div>
    }
}

fn main() {
    yew::start_app::<Todos>();
}
