use crate::state::State;

use crate::colorpicker::ColorPicker;
use crate::grid::Grid;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Red => "red",
            Self::Green => "green",
            Self::Blue => "blue",
        }
    }
}

#[function_component]
pub fn App() -> Html {
    let (state, _) = use_store::<State>();
    let solved = state.selected.iter().all(std::option::Option::is_some);

    if solved {
        if state.validate(3) {
            return html! {
                <div class="container">
                    <h1>{"🎉"}</h1>
                </div>
            };
        }

        return html! {
            <div class="container">
                <h1>{"👎"}</h1>
            </div>
        };
    }

    html! {
        <>
        <ColorPicker />
        <Grid count={state.count} />
        </>
    }
}
