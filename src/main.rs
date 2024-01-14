#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([0.0, 300.0])
            .with_min_inner_size([0.0, 220.0]),
        ..Default::default()
    };
    eframe::run_native(
        "regex wasm",
        native_options,
        Box::new(|cc| Box::new(regex_wasm::App::new(cc))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(regex_wasm::App::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
