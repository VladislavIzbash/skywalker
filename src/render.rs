use pancurses::Window;

use crate::level::Level;

const VIEW_OFFSET: f32 = 1.0;
const VIEW_THRESHOLD: f32 = 0.2;

pub struct Renderer {
    fov: f32,
    view_dist: f32,
}

impl Renderer {
    pub fn new(fov: f32, view_dist: f32) -> Self {
        Self { fov, view_dist }
    }

    pub fn render(
        &self, 
        window: 
        &Window, 
        level: &Level, 
        x: f32, 
        y: f32, 
        angle: f32,
        with_hud: bool,
    ) {
        let angle_start = angle - self.fov / 2.0;
        let (screen_height, screen_width) = window.get_max_yx();
        let angle_per_pixel = self.fov / screen_width as f32;

        window.clear();

        for screen_x in 0..screen_width {
            let hit = level.raytrace(
                x, y, 
                angle_start + (screen_x as f32) * angle_per_pixel, 
                self.view_dist
            );

            let dist = match hit {
                Some(dist) => dist,
                None => continue,
            };

            let wall_top = if dist > VIEW_THRESHOLD {
                (screen_height as f32 / 2.0 * (VIEW_OFFSET - VIEW_OFFSET / dist)) as i32
            } else {
                0
            };
            let wall_bottom = if dist > VIEW_THRESHOLD {
                (screen_height as f32 / 2.0 * (VIEW_OFFSET + VIEW_OFFSET / dist)) as i32
            } else {
                screen_height
            };

            let shade = if dist > 3.0 {
                "\u{2591}"
            } else if dist > 1.5 {
                "\u{2592}"
            } else {
                "\u{2593}"
            };

            for screen_y in wall_top..=wall_bottom {
                window.mvaddstr(screen_y, screen_x, shade);
            }
        }

        if with_hud {
            Self::level_hud(window, level, x, y);
            Self::debug_hud(window, x, y, angle);
        }

        window.refresh();
    }

    fn debug_hud(window: &Window, x: f32, y: f32, angle: f32) {  
        const OFFSET_X: i32 = 18;
        const OFFSET_Y: i32 = 1;

        let width = window.get_max_x();

        Self::draw_box(window, width - OFFSET_X, OFFSET_Y, 15, 4);
        
        window.mvprintw(2, width - OFFSET_X + 1, format!("x = {:.3}", x));
        window.mvprintw(3, width - OFFSET_X + 1, format!("y = {:.3}", y));
        window.mvprintw(4, width - OFFSET_X + 1, format!("angle = {:.3}", angle));
    }

    fn level_hud(window: &Window, level: &Level, x: f32, y: f32) {
        const OFFSET_X: i32 = 1;
        const OFFSET_Y: i32 = 1;

        let size = level.size();
        Self::draw_box(window, OFFSET_X, OFFSET_Y, size.w as i32 + 1, size.h as i32 + 1);

        let image = format!("{}", level);
        for (i, line) in image.lines().enumerate() {
            window.mvaddstr(OFFSET_Y + 1 + i as i32, OFFSET_X + 1, line);
        }

        window.mvaddstr(OFFSET_Y + 1 + y as i32, OFFSET_X + 1 + x as i32, "\u{25cf}");
    }

    fn draw_box(window: &Window, x: i32, y: i32, w: i32, h: i32) {
        const CORNER_LT: &str = "\u{250D}";
        const CORNER_RT: &str = "\u{2511}";
        const CORNER_LB: &str = "\u{2515}";
        const CORNER_RB: &str = "\u{2519}";
        const HLINE: &str = "\u{2501}";
        const VLINE: &str = "\u{2502}";

        window.mvaddstr(y, x, CORNER_LT);
        for _ in 0..(w - 1) {
            window.addstr(HLINE);
        }
        window.addstr(CORNER_RT);

        for y in (y + 1)..(y + h) {
            window.mvaddstr(y, x, VLINE);
            for _ in 0..w {
                window.addch(' ');
            }

            window.mvaddstr(y, x + w, VLINE);
        }

        window.mvaddstr(y + h, x, CORNER_LB);
        for _ in 0..(w - 1) {
            window.addstr(HLINE);
        }
        window.addstr(CORNER_RB);
    }
}
