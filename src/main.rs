use std::fs::File;
use std::vec::Vec;
use std::io::Read;
use std::result::Result;
use std::str::from_utf8;

#[macro_use]
extern crate conrod;
use conrod::widget;
use conrod::Widget;
use conrod::Colorable;
use conrod::Sizeable;
use conrod::Positionable;

extern crate glium;
use glium::DisplayBuild;
use glium::Surface;

extern crate glutin;

widget_ids!{
    struct Ids {
        master,
        scrollbar,
        text,
    }
}

fn read_src() -> std::io::Result<Vec<u8>> {
    let mut file = File::open("src/main.rs")?;
    let mut contents = Vec::<u8>::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

fn main() {
    let display = glium::glutin::WindowBuilder::new()
        .with_vsync()
        .with_dimensions(800, 600)
        .with_title("TEA")
        .build_glium()
        .unwrap();

    let mut ui = conrod::UiBuilder::new([800.0, 600.0]).build();
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();
    let ids = Ids::new(ui.widget_id_generator());

    let contents = read_src().unwrap();
    let content_str = from_utf8(contents.as_slice()).unwrap();

    let font = ui.fonts.insert_from_file("assets/iosevka-regular.ttf").unwrap();
    ui.theme.font_id = Some(font);

    'main: loop {
        for event in display.poll_events() {
            if let Some(event) = conrod::backend::winit::convert(event.clone(), &display) {
                ui.handle_event(event);
            }
            
            match event {
                glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Escape)) |
                glium::glutin::Event::Closed =>
                    break 'main,
                _ => (),
            }
        }

        {
            let mut widgets = ui.set_widgets();

            widget::Canvas::new()
                .color(conrod::color::WHITE)
                .scroll_kids_vertically()
                .set(ids.master, &mut widgets);

            widget::Scrollbar::y_axis(ids.text).set(ids.scrollbar, &mut widgets);

            widget::Text::new(content_str)
                .font_id(font)
                .font_size(15)
                .color(conrod::color::BLACK)
                .w_of(ids.master)
                .top_left_of(ids.master)
                .line_spacing(10.0)
                .set(ids.text, &mut widgets);
        }

        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(1.0, 1.0, 1.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}
