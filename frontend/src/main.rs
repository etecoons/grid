mod app;
mod footer;
mod header;
mod i18n;
mod storage;
mod types;
mod utils;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
