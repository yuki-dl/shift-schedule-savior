// In docker 
// $ trunk serve --address 0.0.0.0
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(src::app::app);
    // src::run();
}
