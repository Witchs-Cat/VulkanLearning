mod application;
mod rendering;

use std::env;
use std::fs::File;
use log::{debug, info, LevelFilter};
use application::ApplicationWindow;
use simple_logger::SimpleLogger;

use self::rendering::{
    RenderingQueue,
    RenderingPipelineConfig,
    RenderingResolution
};


fn main(){
    SimpleLogger::new()
        .with_colors(true)
        .without_timestamps()
        .init()
        .expect("logger initialization exception");

    log::set_max_level(LevelFilter::Error);

    let window =
        ApplicationWindow::new()
            .expect("window creation exception");

    let config = RenderingPipelineConfig{
        window: &window,
        use_validation_layer: true,
        rendering_resolution: RenderingResolution::from(window.inner_size())
    };

    let rendering_queue = RenderingQueue::create(&config)
        .expect("rendering queue create exception");

    window.run()
        .expect("main loop exception");

} // drop(str2);
