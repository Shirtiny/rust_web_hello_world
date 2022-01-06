/*
 * @Author: Shirtiny
 * @Date: 2022-01-05 10:06:31
 * @LastEditTime: 2022-01-05 16:18:47
 * @Description:
 */
use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
    ha: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
            ha: "haha".into(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
                  <div>
                      <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                      <p>{ self.value }</p>
                      <button>{self.ha.clone()}</button>
                  </div>
        }
    }
}

#[function_component(App)]
fn app() -> Html {
    let count = use_state(|| 0);
    let handle_click = {
        let counter = count.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };
    html! {
        <div>
            { "hello world, " }
            {*count}
            <button onclick={handle_click} style={"margin-left: 8px"}>{"+1"}</button>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
