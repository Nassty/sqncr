use crate::{app::Color, state::State};
use yew::prelude::*;
use yewdux::prelude::*;
#[function_component]
pub fn ColorPicker() -> Html {
    let (state, _) = use_store::<State>();

    html! {
        <div class="container">
            <ColorButton color={Color::Red} count={state.budget.get(&Color::Red).unwrap_or(&0)} />
            <ColorButton color={Color::Green} count={state.budget.get(&Color::Green).unwrap_or(&0)} />
            <ColorButton color={Color::Blue} count={state.budget.get(&Color::Blue).unwrap_or(&0)} />
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct ColorButtonProps {
    color: Color,
    count: usize,
}

#[function_component]
fn ColorButton(props: &ColorButtonProps) -> Html {
    let (state, dispatch) = use_store::<State>();
    let onclick = |color: Color| {
        dispatch.reduce_mut_callback(move |state| {
            state.current = color.clone();
        })
    };
    let disabled = state.current == props.color;
    let fill_class = if props.count == 0 {
        "empty"
    } else {
        "contains"
    };
    html! {
        <button {disabled}
            class={classes!(format!("button-{}", props.color.as_str()), format!("button-{fill_class}"))}
            onclick={onclick(props.color.clone())}>
            {format!("{}", props.count)}
        </button>
    }
}
