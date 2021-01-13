use std::cell::RefCell;
use std::rc::Rc;

use pengui::backend::glium::*;
use pengui::core::*;
use pengui::loc;
use pengui::widget::*;

extern crate glium;
extern crate image;

pub struct Ui {
    ui: Interface,

    font: Rc<RefCell<dyn FontAtlas>>,
    ensps_texture: TextureId,

    pub editable_text: String,
}

impl Ui {
    pub fn new<'a>(backend: &'a mut GliumBackend) -> Self {
        let font = backend.get_font(0).clone();
        let ensps_texture =
            Self::register_image(backend, &include_bytes!("../resources/logo_ensps.png")[..]);

        let ui = Interface::new();

        let editable_text = String::from("Editable text, japanese characters: 色は匂へど散");

        Self {
            ui,
            font,
            ensps_texture,
            editable_text,
        }
    }

    pub fn register_event(&mut self, event: Event, ray: Option<&Ray>) {
        self.ui.register_event(event, ray);
    }

    pub fn ui(&mut self) {
        self.ui.new_frame();

        WindowBuilder::new(|ui| {
            if PaddingBuilder::new(
                (0.2, 0.2),
                Button::new("Clickable button".to_string(), &self.font),
            )
            .build(loc!(), &ui)
            {
                println!("Button inside the padding clicked");
            }
            let frame_number = FrameCounter::new().build(loc!(), &ui);
            if CheckBox::new("A checkbox".to_string(), &self.font).build(loc!(), &ui) {
                Text::new(
                    format!("Frames since beginning : {}", frame_number),
                    &self.font,
                )
                .build(loc!(), &ui);
            }
            Text::new(self.editable_text.clone(), &self.font)
                .size(0.75)
                .build(loc!(), &ui);
            if Button::new("               ".to_string(), &self.font)
                .color((1., 1., 1., 1.))
                .texture(self.ensps_texture)
                .build(loc!(), &ui)
            {
                println!("Button with texture clicked")
            }
            Text::new("↑ Textured button".to_string(), &self.font).build(loc!(), &ui);
        })
        .size((20., 12.))
        .build(loc!(), &self.ui.root);

        self.ui.end_frame();
        self.ui.generate_layout();
    }

    pub fn draw_list(&self) -> DrawList {
        self.ui.draw()
    }

    fn register_image(backend: &mut GliumBackend, bytes: &[u8]) -> TextureId {
        use std::io::Cursor;
        let image = image::load(Cursor::new(bytes), image::ImageFormat::Png)
            .unwrap()
            .to_rgba8();
        let image_dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

        backend.register_texture(image)
    }
}
