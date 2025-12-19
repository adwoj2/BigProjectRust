use macroquad::prelude::*;

pub struct Button {
    pub rect: Rect,
    pub label: String,
    pub color: Color,
}

impl Button {
    pub fn draw(&self) {
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            self.color,
        );
        draw_rectangle_lines(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            2.0,
            BLACK,
        );

        let font_size = 24.0;
        let dims = measure_text(&self.label, None, font_size as u16, 1.0);

        draw_text(
            &self.label,
            self.rect.x + (self.rect.w - dims.width) / 2.0,
            self.rect.y + (self.rect.h + dims.height) / 2.0,
            font_size,
            WHITE,
        );
    }

    pub fn clicked(&self) -> bool {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            self.rect.contains(vec2(mx, my))
        } else {
            false
        }
    }
}
