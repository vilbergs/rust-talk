use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct EntryProps {
  pub id: usize,
  pub name: String,
  pub is_complete: bool,
  pub ontoggle: Callback<usize>,
}

#[function_component(TodoItem)]
pub fn todo_item(props: &EntryProps) -> Html {
  let id = props.id;
  let name = props.name.clone();
  let is_complete = props.is_complete;

  let ontoggle = {
    let ontoggle = props.ontoggle.clone();
    move |_| ontoggle.emit(id)
  };

  let onclicktoggle = {
    let ontoggle = props.ontoggle.clone();
    move |_| ontoggle.emit(id)
  };

  html! {
      <div class={if is_complete { classes!("item", "complete")} else { classes!("item") }} onclick={onclicktoggle}>
          <input type="checkbox" checked={is_complete} onchange={ontoggle}/>
          {name}
      </div>
  }
}
