use macroquad::prelude::*;

pub struct Menu {
    pub is_active: bool,
    state: i32,
}


impl Menu {
    pub fn new() -> Self {
        Self {
            is_active: false,
            state: 0
        }
    }
}

pub fn update_menu(menu: &mut Menu, camera: &Camera2D) {
    if is_key_pressed(KeyCode::Escape) {
        if !menu.is_active {
            menu.is_active = true;
        }
		else 
        {
			menu.is_active = false;
        }
    }
    if !menu.is_active {
        return;
    }
    let buttons = [
        (1, Rect::new(110.0, 80.0,  180.0, 25.0)),
        (2, Rect::new(110.0, 112.0, 180.0, 25.0)),
        (3, Rect::new(110.0, 144.0, 180.0, 25.0)),
    ];
    let (mx, my) = mouse_position();
    let world_mouse = camera.screen_to_world(vec2(mx, my));

    for (id, rect) in &buttons {
        let hovered = rect.contains(world_mouse);
        if hovered && is_mouse_button_pressed(MouseButton::Left) {
            menu.state = *id;
            menu.is_active = false;
        }
    }
    
    match menu.state {
        1 => {
            menu.state = 0;
            menu.is_active = false;
            return
        }
        2 => {
            menu.state = 0;
            menu.is_active = false;
            return
        }
        3 => {
            std::process::exit(0);
        }
        _ => {}
}
}

pub fn draw_menu(menu: &Menu, camera: &Camera2D) {

    if menu.is_active {
        draw_rectangle(100.0, 70.0 , 200.0, 100.0, Color::new(255.0, 193.0, 0.0, 1.0));
        let labels = ["Jouer", "Options", "Quitter"];
        let (mx, my) = mouse_position();
        let world_mouse = camera.screen_to_world(vec2(mx, my)); // ← idem ici

        for (i, label) in labels.iter().enumerate() {
            let rect = Rect::new(110.0, 80.0 + i as f32 * 32.0, 180.0, 25.0);
            let hovered = rect.contains(world_mouse); 

            let bg_color = if hovered {
                Color::new(1.0, 1.0, 1.0, 0.2)
            } else {
                Color::new(1.0, 1.0, 1.0, 0.05)
            };

            draw_rectangle(rect.x, rect.y, rect.w, rect.h, bg_color);
            draw_text(label, rect.x + 8.0, rect.y + 17.0, 18.0, WHITE);
        }
    }
    
}
            
