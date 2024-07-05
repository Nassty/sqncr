#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
mod app;
mod colorpicker;
mod grid;
mod state;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
