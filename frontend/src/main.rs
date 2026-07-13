mod app;
mod board_normalizer;
mod components;
mod i18n;
mod types;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
