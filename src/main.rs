use anyhow::Result;
use cursive::{Cursive, reexports::log, views::Canvas, Printer, event::Event};

fn main() -> Result<()> {
    let backend_init = || -> std::io::Result<Box<dyn cursive::backend::Backend>> {
        let backend = cursive::backends::crossterm::Backend::init()?;
        let buffered_backend = cursive_buffered_backend::BufferedBackend::new(backend);
        Ok(Box::new(buffered_backend))
    };

    let mut siv = Cursive::new();

    cursive::logger::init();
    log::set_max_level(log::LevelFilter::Info);
    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback('`', Cursive::toggle_debug_console);

    let state = ();
    let canvas = Canvas::new(state)
        .with_draw(draw)
        .with_on_event(|state: &(), event| match event {
            Event::Char('a) => {
                let st = state;
            }
        });

    Ok(())
}

fn draw(state: &(), printer: &Printer) {
    log::info!("Drawing... Size: {}x{}", printer.size.x, printer.size.y);
}