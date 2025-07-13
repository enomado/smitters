#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

use egui::{Color32, Stroke};
use smitters::{
    chart_fancy::draw_smith_fancy,
    context::Context,
    math::{xy_to_resistance, Pos2},
};

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        // initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Smith graph XY -> RX",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let ctx = Context::get_context(ui);

            draw_smith_fancy(&ctx);

            let p = ui.input(|x| x.pointer.hover_pos());

            if let Some(point) = p {
                // idk

                let p = Pos2 {
                    x: -((ctx.center().x - point.x as f64) / ctx.half_width),
                    y: -(ctx.center().y - point.y as f64) / ctx.half_width,
                };

                let r = xy_to_resistance(&p);

                let react = r.1;
                let res = r.0;

                let stroke = Stroke {
                    width: 1.,
                    color: Color32::GREEN,
                };

                ctx.react(react.0, 0.0, 1000., &stroke);

                ctx.res(res.0, -1000.0, 1000., &stroke);

                ui.label(format!("px {:?}", &p.x));
                ui.label(format!("py {:?}", &p.y));
                ui.label(format!("r {:?}", &r.0));
                ui.label(format!("x {:?}", &r.1));
            }
        });
    }
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast;
    log::info!("GFS");
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|_cc| Ok(Box::<MyApp>::default())),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}
