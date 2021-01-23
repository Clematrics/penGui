use std::cell::RefCell;
use std::f32::consts::FRAC_PI_2;
use std::rc::Rc;

use pengui::backend::glium::*;
use pengui::core::*;
use pengui::loc;
use pengui::widget::*;

use nalgebra::*;

extern crate glium;
extern crate image;
extern crate rand;

pub struct Ui {
    ui: Interface,

    font: Rc<RefCell<dyn FontAtlas>>,
    ensps_texture: TextureId,

    window_width: f32,
    window_height: f32,
    random_color: (f32, f32, f32, f32),
    extrude: f32,
    button_active: bool,
    texture_button_active: bool,
    pub radius: f32,
    pub editable_text: String,
}

impl Ui {
    pub fn new(backend: &mut GliumBackend) -> Self {
        let font = backend.get_font(0);
        let ensps_texture =
            Self::register_image(backend, &include_bytes!("../resources/pengui.png")[..]);

        let ui = Interface::new();

        let editable_text = String::from("Editable text. Type here!");

        Self {
            ui,
            font,
            ensps_texture,
            window_width: 25.,
            window_height: 25.,
            random_color: (0.231, 0.294, 0.451, 1.),
            extrude: 0.2,
            button_active: false,
            texture_button_active: false,
            radius: 5.,
            editable_text,
        }
    }

    pub fn register_event(&mut self, event: Event, ray: Option<&Ray>) {
        self.ui.register_event(event, ray);
    }

    fn input_mul<'a>(
        string: &'a str,
        var: &'a mut f32,
        font: &'a Rc<RefCell<dyn FontAtlas>>,
    ) -> InlineBuilder<'a> {
        InlineBuilder::new(move |ui| {
            LabelBuilder::new(string, font).build(loc!(), &ui);
            if Button::new("◀".to_string(), font).build(loc!(), &ui) {
                *var /= 1.1;
            }
            LabelBuilder::new(format!("{:.2}", var).as_str(), font).build(loc!(), &ui);
            if Button::new("▶".to_string(), font).build(loc!(), &ui) {
                *var *= 1.1;
            }
        })
    }

    fn input_add<'a>(
        string: &'a str,
        var: &'a mut f32,
        font: &'a Rc<RefCell<dyn FontAtlas>>,
    ) -> InlineBuilder<'a> {
        InlineBuilder::new(move |ui| {
            LabelBuilder::new(string, font).build(loc!(), &ui);
            if Button::new("◀".to_string(), font).build(loc!(), &ui) {
                *var -= 1.0;
            }
            LabelBuilder::new(format!("{:.2}", var).as_str(), font).build(loc!(), &ui);
            if Button::new("▶".to_string(), font).build(loc!(), &ui) {
                *var += 1.0;
            }
        })
    }

    pub fn ui(&mut self) {
        self.ui.new_frame();
        let root = self.ui.root.clone();
        let window_width = self.window_width;
        let window_height = self.window_height;

        WindowBuilder::new(|ui| {
            let frame_number = FrameCounter::new().build(loc!(), &ui);
            if CenterLayout::new(Button::new("Clickable button".to_string(), &self.font))
                .build(loc!(), &ui)
            {
                self.button_active = !self.button_active;
            }
            if self.button_active {
                LabelBuilder::new(
                    format!("Frames since beginning : {}", frame_number).as_str(),
                    &self.font,
                )
                .build(loc!(), &ui);
            }
            if Button3D::new("3D BUTTON !!!".to_string(), &self.font)
                .extrude(self.extrude)
                .color(self.random_color)
                .build(loc!(), &ui)
            {
                let r = rand::random::<f32>();
                let g = rand::random::<f32>();
                let b = rand::random::<f32>();
                self.random_color = (r, g, b, 1.);
            }

            if CheckBox::new("A checkbox".to_string(), &self.font).build(loc!(), &ui) {
                if Button::new("        ".to_string(), &self.font)
                    .font_size(5.)
                    .color((1., 1., 1., 1.))
                    .texture(self.ensps_texture)
                    .build(loc!(), &ui)
                {
                    self.texture_button_active = !self.texture_button_active;
                }
                if self.texture_button_active {
                    LabelBuilder::new("Hi!", &self.font).build(loc!(), &ui);
                }
            }
            MaximizeLayout::new(TextBuilder::new(&mut self.editable_text, &self.font).size(0.75))
                .build(loc!(), &ui);

            Self::input_mul("Extrusion", &mut self.extrude, &self.font).build(loc!(), &ui);

            Self::input_mul("Radius", &mut self.radius, &self.font).build(loc!(), &ui);

            Self::input_add("Window width", &mut self.window_width, &self.font).build(loc!(), &ui);

            Self::input_add("Window height", &mut self.window_height, &self.font)
                .build(loc!(), &ui);
        })
        .size((window_width, window_height))
        .transform(Similarity3::new(
            Vector3::new(0., 0., -20.),
            Vector3::zeros(),
            1.,
        ))
        .build(loc!(), &root);

        WindowBuilder::new(|ui| {
            PaddingBuilder::new(
                (1., 1.),
                LabelBuilder::new("Many characters supported.\rJapanese: 色は匂へど散\rRussian: Не́которые иностра́нцы ду́мают", &self.font)
            ).build(loc!(), &ui);
            LabelBuilder::new(self.editable_text.as_str(), &self.font).build(loc!(), &ui);
        })
        .size((20., 15.))
        .transform(Similarity3::new(Vector3::new(-20., 0., 0.), Vector3::y() * FRAC_PI_2, 1.))
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
