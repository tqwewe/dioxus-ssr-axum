use app::app::app;

fn main() {
    dioxus::web::launch_cfg(app, |cfg| cfg.hydrate(true));
}
