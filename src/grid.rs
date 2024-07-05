use crate::state::State;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Default, Clone, Eq, PartialEq, Properties)]
pub struct Props {
    pub count: usize,
}

#[function_component(Grid)]
pub fn grid(props: &Props) -> Html {
    let (state, dispatch) = use_store::<State>();

    let onclick = |i: usize| {
        dispatch.reduce_mut_callback(move |state| {
            if state.budget[&state.current] > 0
                && (state.selected[i].is_none()
                    || state.selected[i].as_ref().unwrap() != &state.current)
            {
                if let Some(prev) = &state.selected[i] {
                    state.budget.insert(prev.clone(), state.budget[prev] + 1);
                }
                state.selected[i] = Some(state.current.clone());
                state
                    .budget
                    .insert(state.current.clone(), state.budget[&state.current] - 1);
            }
        })
    };
    html! {
        <div class="container">
        { for (0..props.count).into_iter().map(|i| {
            let cname = state.selected[i].as_ref().map_or("none", |color| color.as_str());
            html! {
                <button class={format!("button-{}", cname)} onclick={onclick(i)}>{"X"}</button>
            }
        })}
        </div>
    }
}
