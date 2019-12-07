rltk::add_wasm_support!();

use rltk::{Console, GameState, Rltk, RGB};
use rltk::gui::*;
use rltk::element;

struct State {
    gui : Option<UI>
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        if let Some(gui) = &mut self.gui {
            ctx.cls();

            if let Some(fps) = element!(gui, "fps", PlainText) {
                fps.text = format!("FPS: {}", ctx.fps);
            }

            gui.render(ctx);
        } else {
            self.gui = Some(self.build_gui(ctx));
        }
    }    
}

impl State {
    fn build_gui(&self, ctx : &mut Rltk) -> UI {
        let mut ui = UI::new(Background::default(ctx, "bg"));
            ui.add("bg", MenuBar::default(ctx, "menubar"));
            ui.add("bg", StatusBar::default(ctx, "statusbar"));
            ui.add("statusbar", PlainText::default(ctx, "altx", "Alt-X", 1, 0, RGB::named(rltk::RED), RGB::named(rltk::LIGHT_GRAY)));
            ui.add("statusbar", PlainText::default(ctx, "exit", "Exit | ", 7, 0, RGB::named(rltk::NAVY), RGB::named(rltk::LIGHT_GRAY)));
            ui.add("statusbar", PlainText::default(ctx, "fps", &format!("FPS: {}", ctx.fps), 16, 0, RGB::named(rltk::NAVY), RGB::named(rltk::LIGHT_GRAY)));
            ui.add("menubar", PlainText::default(ctx, "F", "F", 1, 0, RGB::named(rltk::RED), RGB::named(rltk::LIGHT_GRAY)));
            ui.add("menubar", PlainText::default(ctx, "file", "ile", 2, 0, RGB::named(rltk::NAVY), RGB::named(rltk::LIGHT_GRAY)));
            ui.add("bg", Window::new(ctx, "win", 10, 10, 50, 30, "This is a window"));
            ui.add("win", PlainText::default(ctx, "body", "Body text", 0, 0, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK)));
            ui
    }
}

fn main() {
    let context = Rltk::init_simple8x8(80, 50, "Hello GUI", "resources");

    let gs: State = State { gui : None };
    rltk::main_loop(context, gs);
}