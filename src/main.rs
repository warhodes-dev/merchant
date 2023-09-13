use std::time::SystemTime;

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

    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs() as u32; // Will fail after 19 January 2038

    let worldapp = WorldApp::new((180, 50), seed);
    let canvas = Canvas::new(worldapp.clone())
        .with_draw(draw)
        .with_on_event(|worldapp: &mut WorldApp, event| match event {
            Event::Char('a') => {
                EventResult::consumed()
            }
            Event::Char('r') => {
                let _ = worldapp.generate();
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

    for x in 0..canvas_width {
        for y in 0..canvas_height {
            let tile = worldapp.get_tile(x, y);
            let style = ColorStyle::new(tile.bg_rgb(), tile.fg_rgb());
            p.with_color(style, |printer| {
                printer.print( (x, y), &tile.glyph().to_string());
            })
        }
    }
}

fn noisify(color: Color) -> Color {
    use rand_distr::{Normal, Distribution};
    let norm = Normal::new(0.0, 2.0).unwrap();
    let v = norm.sample(&mut rand::thread_rng()) as u8;
    let Color::Rgb(r, g, b) = color else {
        panic!("noisify only takes Color::Rgb")
    };
    Color::Rgb(r.saturating_sub(v), g.saturating_sub(v), b.saturating_sub(v))

}