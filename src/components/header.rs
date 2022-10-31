use yew::prelude::*;

pub struct Header;
impl Component for Header {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &yew::html::Context<Self>) -> Self { Self }

    fn view(&self, ctx: &yew::html::Context<Self>) -> Html {
        html! {
            <div class="header">
            <h1>
                {"This is the header"}
            </h1>
            </div>
        }
    }
}