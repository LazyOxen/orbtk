use orbclient::Color;
use orbimage::Image;

use super::{Point, Rect};

pub trait Renderer {
    fn clear(&mut self, color: Color);
    fn char(&mut self, pos: Point, c: char, color: Color);
    fn image(&mut self, pos: Point, image: &Image);
    fn pixel(&mut self, point: Point, color: Color);
    fn arc(&mut self, center: Point, radius: i32, parts: u8, color: Color);
    fn circle(&mut self, center: Point, radius: i32, color: Color);
    fn line(&mut self, start: Point, end: Point, color: Color);
    fn rect(&mut self, rect: Rect, color: Color);
    fn linear_gradient(&mut self, bounded_area: Rect, start: Point, end: Point, start_color: Color, end_color: Color);

    fn rounded_rect(&mut self, rect: Rect, radius: u32, filled: bool, color: Color) {
        let x = rect.x;
        let y = rect.y;
        let w = rect.width as i32;
        let h = rect.height as i32;
        let r = radius as i32;


        if filled {
            //Draw inside corners
            self.arc(Point::new(x + r, y + r), -r, 1 << 4 | 1 << 6, color);
            self.arc(Point::new(x + w - 1 - r, y + r), -r, 1 << 5 | 1 << 7, color);
            self.arc(Point::new(x + r, y + h - 1 - r),- r, 1 << 0 | 1 << 2, color);
            self.arc(Point::new(x + w - 1 - r, y + h - 1 - r), -r, 1 << 1 | 1 << 3, color);

            // Draw inside rectangles
            self.rect(Rect::new(x + r, y, (w - 1 - r * 2) as u32, r as u32 + 1), color);
            self.rect(Rect::new(x + r, y + h - 1 - r, (w - 1 - r * 2) as u32, r as u32 + 1), color);
            self.rect(Rect::new(x, y + r + 1, w as u32, (h - 2 - r * 2) as u32), color);
        } else {
            //Draw outside corners
            self.arc(Point::new(x + r, y + r), r, 1 << 4 | 1 << 6, color);
            self.arc(Point::new(x + w - 1 - r, y + r), r, 1 << 5 | 1 << 7, color);
            self.arc(Point::new(x + r, y + h - 1 - r), r, 1 << 0 | 1 << 2, color);
            self.arc(Point::new(x + w - 1 - r, y + h - 1 - r), r, 1 << 1 | 1 << 3, color);

            // Draw outside rectangles
            self.rect(Rect::new(x + r + 1, y, (w - 2 - r * 2) as u32, 1), color);
            self.rect(Rect::new(x + r + 1, y + h - 1, (w - 2 - r * 2) as u32, 1), color);
            self.rect(Rect::new(x, y + r + 1, 1, (h - 2 - r * 2) as u32), color);
            self.rect(Rect::new(x + w - 1, y + r + 1, 1, (h - 2 - r * 2) as u32), color);
        }
    }
}
