use macroquad::prelude::*;

fn config() -> Conf {
    Conf {
        window_title: "Paint".into(),
        ..Default::default()
    }
}

#[derive(PartialEq, Eq)]
enum Tool {
    Paint,
    Rect,
    Ellipse,
    Line,
}

impl Tool {
    fn toggle(&mut self, tool: Self) {
        *self = if *self == tool { Self::Paint } else { tool }
    }
    const fn name(&self) -> char {
        match self {
            Self::Paint => 'P',
            Self::Rect => 'R',
            Self::Ellipse => 'E',
            Self::Line => 'L',
        }
    }
    fn update(&mut self) {
        if is_key_pressed(KeyCode::P) {
            *self = Self::Paint;
        }
        if is_key_pressed(KeyCode::R) {
            self.toggle(Self::Rect);
        }
        if is_key_pressed(KeyCode::L) {
            self.toggle(Self::Line);
        }
        if is_key_pressed(KeyCode::E) {
            self.toggle(Self::Ellipse);
        }
    }
    fn draw(&self, from: (f32, f32), to: (f32, f32), color: Color) {
        match self {
            Self::Paint => {}
            Self::Rect => {
                draw_rectangle_lines(from.0, from.1, to.0 - from.0, to.1 - from.1, 4.0, color)
            }
            Self::Ellipse => {
                let w = (to.0 - from.0) * 0.5;
                let h = (to.1 - from.1) * 0.5;
                let x = from.0 + w;
                let y = from.1 + h;
                draw_ellipse_lines(x, y, w, h, 0.0, 2.0, color)
            }
            Self::Line => draw_line(from.0, from.1, to.0, to.1, 2.0, color),
        }
    }
}

#[macroquad::main(config)]
async fn main() {
    let win_size = vec2(screen_width(), screen_height());

    let mut last_mouse = mouse_position();
    let mut clicked_mouse = None;
    let mut clear = true;
    let mut tool = Tool::Paint;

    let camera = Camera2D {
        render_target: Some({
            let render = render_target(win_size.x as u32, win_size.y as u32);
            render.texture.set_filter(FilterMode::Nearest);
            render
        }),
        offset: vec2(-1.0, -1.0),
        zoom: 2.0 / win_size,
        ..Default::default()
    };

    loop {
        if is_key_pressed(KeyCode::C) {
            clear = true;
        }

        tool.update();

        set_camera(&camera);

        if clear {
            clear_background(BLACK);
            clear = false;
        }

        let mouse_position = mouse_position();

        match tool {
            Tool::Paint => {
                if is_mouse_button_down(MouseButton::Left) {
                    draw_line(
                        last_mouse.0,
                        last_mouse.1,
                        mouse_position.0,
                        mouse_position.1,
                        2.0,
                        WHITE,
                    );
                }
            }
            ref x => {
                if is_mouse_button_pressed(MouseButton::Left) {
                    clicked_mouse = Some(mouse_position);
                }
                if is_mouse_button_released(MouseButton::Left) {
                    if let Some(clicked_mouse) = clicked_mouse {
                        x.draw(clicked_mouse, mouse_position, WHITE);
                    }
                    clicked_mouse = None;
                }
            }
        }

        last_mouse = mouse_position;

        set_default_camera();

        clear_background(BLACK);

        camera.render_target.as_ref().map(|x| {
            draw_texture_ex(
                &x.texture,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(win_size),
                    ..Default::default()
                },
            );
        });

        if let Some(clicked_mouse) = clicked_mouse {
            tool.draw(clicked_mouse, mouse_position, GRAY);
        }

        draw_text(&format!("{}", tool.name()), 6.6, 33.3, 33.3, WHITE);

        next_frame().await
    }
}
