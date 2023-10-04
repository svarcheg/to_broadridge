use std::f32::consts::PI;
use macroquad::prelude::*;

#[macroquad::main("To Broadridge/BTCS/Itiviti/Tbricks/Ullink people")]
async fn main() {
    let mut hearts = Vec::new();
    for _i in 0..2000 {
        hearts.push(Heart::new())
    }
    loop {
        for heart in &mut hearts {
            heart.update();
            heart.draw()
        }
        draw_message("Thank you guys for all, love you");
        next_frame().await
    }
}

struct Heart {
    x: f32,
    y: f32,
    z: f32,
    beat: f32,
}

impl Heart {
    fn new() -> Self {
        Heart {
            x: rand::gen_range(-screen_width(), screen_width()),
            y: rand::gen_range(-screen_height(), screen_height()),
            z: rand::gen_range(0., screen_width()),
            beat: rand::gen_range(0., 1.),
        }
    }

    fn update(&mut self) {
        self.beat += 0.02;
        if self.beat >= 1.
        {
            self.beat = 0.;
        }
        self.z -= 3.;
        if self.z < 0. {
            self.z = screen_width();
            self.x = rand::gen_range(-screen_width(), screen_width());
            self.y = rand::gen_range(-screen_height(), screen_height());
        }
    }

    fn draw(&self) {
        let coef = (self.beat * PI).sin();
        let hx = map(self.x / self.z, 0., 1., 0., screen_width());
        let hy = map(self.y / self.z, 0., 1., 0., screen_height());
        let hr = map(self.z, 0., screen_width(), 2., 0.);
        draw_heart(hx + screen_width() / 2., hy + screen_height() / 2., hr * (1. - coef), RED);
    }
}

fn map(value: f32, istart: f32, istop: f32, ostart: f32, ostop: f32) -> f32 {
    ostart + (ostop - ostart) * ((value - istart) / (istop - istart))
}

fn draw_heart(x: f32, y: f32, r: f32, color: Color) {
    let sides = 20;
    let context: InternalGlContext = unsafe { get_internal_gl() };

    let mut vertices = Vec::<Vertex>::with_capacity(sides as usize + 2);
    let mut indices = Vec::<u16>::with_capacity(sides as usize * 3);

    vertices.push(Vertex::new(x, y, 0., 0., 0., color));
    for i in 0..=sides {
        let angle = i as f32 / sides as f32 * 2. * PI;
        let hx = 16. * angle.sin().powi(3);
        let hy = -(13. * angle.cos() - 5. * (2. * angle).cos() - 2. * (3. * angle).cos() - (4. * angle).cos());

        let vertex = Vertex::new(x + r * hx, y + r * hy, 0., 0., 0., color);
        vertices.push(vertex);

        if i != sides {
            indices.extend_from_slice(&[0, i as u16 + 1, i as u16 + 2]);
        }
    }

    context.quad_gl.texture(None);
    context.quad_gl.draw_mode(DrawMode::Triangles);
    context.quad_gl.geometry(&vertices, &indices);
}

fn draw_message(text : &str) {
    let td = measure_text(text, Option::None, 40, 1. );
    draw_text(text, (screen_width()  - td.width) / 2., (screen_height()  - td.height)/ 2., 40., GOLD);
}