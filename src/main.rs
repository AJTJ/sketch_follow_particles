use nannou::prelude::*;

const WIDTH: u32 = 1250;
const HEIGHT: u32 = 650;
const N_ELEMENTS: usize = 200;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(1, 1)
        .run();
}

struct Element {
    position: Vec2,
}

impl Element {
    pub fn new(p: Vec2) -> Self {
        Element { position: p }
    }
}

struct Model {
    elements: Vec<Element>,
}

fn model(_app: &App) -> Model {
    let mut elements = Vec::new();
    for _ in 0..N_ELEMENTS {
        let elem = Element::new(vec2(
            (random::<f32>() - 0.5) * WIDTH as f32,
            (random::<f32>() - 0.5) * HEIGHT as f32,
        ));
        elements.push(elem);
    }

    Model { elements }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // move it to the corner of the screen
    app.main_window().set_outer_position_pixels(0, 0);
    // resize it so that it is now visible
    app.main_window().set_inner_size_pixels(WIDTH, HEIGHT);

    let m = app.mouse.position();

    if m.x != 0.0 && m.y != 0.0 {
        for el in model.elements.iter_mut() {
            // FOLLOWING MOUSE
            let e = el.position;

            let main_distance =
                ((Pow::<f32>::pow(m.x - e.x, 2.0)) + (Pow::<f32>::pow(m.y - e.y, 2.0))).sqrt();
            let inc = 4.0;

            let x_new = e.x + ((inc / main_distance) * (m.x - e.x));
            let y_new = e.y + ((inc / main_distance) * (m.y - e.y));

            // this creates points that follow the mouse pointer
            el.position = vec2(x_new, y_new);

            // HOVERING POINT IN MOTION EFFECT
            el.position += vec2(random::<f32>() - 0.5, random::<f32>() - 0.5);
        }
    }
}

fn get_color(time: f32) -> Rgba {
    rgba(
        (time / 1000.0).sin() + 0.5,
        ((time + 200.0) / 800.0).sin() + 0.5,
        ((time + 150.0) / 700.0).sin() + 0.5,
        1.0,
    )
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    draw.rect()
        .w_h(WIDTH as f32, HEIGHT as f32)
        .color(srgba(0.0, 0.0, 0.0, 0.005));

    // TIME AS FRAMES
    // let time = app.elapsed_frames() as f32 / 60.0;

    // TIME AS MILLISECONDS
    let time = app.duration.since_start.as_millis();

    for el in model.elements.iter() {
        // FOR SINGLE POINT DRAWING
        draw.ellipse()
            .xy(el.position)
            .radius(10.0)
            .color(get_color(time as f32));
    }

    draw.to_frame(app, &frame).unwrap();
}
