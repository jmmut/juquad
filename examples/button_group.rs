use std::mem::ManuallyDrop;
use juquad::widgets::anchor::Anchor;
use juquad::widgets::button::{Button, Style};
use juquad::widgets::button_group::ButtonGroup;
use macroquad::prelude::{BLUE, clear_background, next_frame, screen_height, screen_width};

const FONT_SIZE: f32 = 16.0;
const STYLE: Style = Style::new();

struct Buttons {
    button1: Button,
    button2: Button,
    button3: Button,
    button4: Button,
}
union ButtonUnion {
    b: ManuallyDrop<Buttons>,
    v: ManuallyDrop<[Button; size_of::<Buttons>() / size_of::<Button>()]>,
}
union Vector {
    xyz: XYZ,
    v: [f64; size_of::<XYZ>() / size_of::<f64>()],
}

#[derive(Copy, Clone)]
struct XYZ {
    x: f64,
    y: f64,
    z: f64,
}

fn main2() {
    let mut vec = Vector {
        xyz: XYZ { x: 1.0, y: 2.0, z: 3.0 },
    };

    unsafe {
        // Access xyz
        println!("x: {}, y: {}, z: {}", vec.xyz.x, vec.xyz.y, vec.xyz.z);

        // Access v
        println!("v[0]: {}, v[1]: {}, v[2]: {}", vec.v[0], vec.v[1], vec.v[2]);
    }
}

#[macroquad::main("juquad button group")]
async fn main() {
    let button_group = ButtonGroup::new(
        FONT_SIZE,
        Anchor::top_center(screen_width() * 0.5, screen_height() * 0.25),
    );

    let mut buttons :Buttons = create_T(button_group);

    let mut show_button3 = false;
    loop {
        clear_background(BLUE);
        if buttons.button1.interact().is_clicked() {
            show_button3 = !show_button3;
        }
        buttons.button2.interact();
        buttons.button3.interact();
        buttons.button4.interact();

        buttons.button1.render(&STYLE);
        buttons.button2.render(&STYLE);
        if show_button3 {
            buttons.button3.render(&STYLE);
            buttons.button4.render(&STYLE);
        }
        next_frame().await
    }
}

fn create_T(button_group: ButtonGroup) -> Buttons {
    let vec = button_group.create(["some button", "some long long long button", "another button", "UPPER CASE BUTTON"]);
    let mut buttons_u = ButtonUnion { v: ManuallyDrop::new(vec) };
    unsafe { ManuallyDrop::take(&mut buttons_u.b) }
}
