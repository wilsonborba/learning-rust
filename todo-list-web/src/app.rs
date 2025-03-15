use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    // create a navbar that will use bootstrap classes
    html! {
        <nav class="navbar navbar-expand-lg navbar-light bg-light">
            <div class="container-fluid">
                <a class="navbar-brand" href="">{"Todo List"}</a>
            </div>
        </nav>
    }
    
}