#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 400.0])
            .with_min_inner_size([500.0, 400.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .unwrap(),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "rety",
        native_options,
        Box::new(|cc| Box::new(rety::App::new(cc))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "rety_canvas",
                web_options,
                Box::new(|cc| Box::new(rety::App::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
