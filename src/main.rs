use anyhow::Result;
use cursive::{Cursive, reexports::log, views::Canvas, Printer, event::{Event, EventResult}, view::Resizable, theme::{ColorStyle, Color}};

mod world;
use world::WorldApp;

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

    let worldapp = WorldApp::new((80, 40), 0);
    let canvas = Canvas::new(worldapp.clone())
        .with_draw(draw)
        .with_on_event(|worldapp: &mut WorldApp, event| match event {
            Event::Char('a') => {
                EventResult::consumed()
            }
            _ => EventResult::Ignored
        })
        .fixed_size((worldapp.width, worldapp.height));

    siv.add_layer(canvas);

    siv.try_run_with(backend_init)?;

    Ok(())
}

fn draw(worldapp: &WorldApp, p: &Printer) {
    let canvas_width = p.size.x;
    let canvas_height = p.size.y;

    for x in 0..worldapp.width {
        for y in 0..worldapp.height {
            let tile = worldapp.get_tile(x, y);
            let x1 = x as u8;
            let y1 = y as u8;
            let style = ColorStyle::new(
                Color::Rgb(y1, x1, y1), // Why does this not work?
                Color::Rgb(x1, y1, x1)
            );
            p.with_color(style, |printer| {
                let mut buf = [0u8; 4];
                printer.print( (x, y), tile.glyph().encode_utf8(&mut buf));
            })
        }
    }
}