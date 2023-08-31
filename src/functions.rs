use std::rc::Rc;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;

use enigo::*;
use mlua::prelude::*;

#[derive(Clone)]
pub struct Controller<'a> {
    enigo: Rc<Mutex<Enigo>>,
    lua: &'a Lua,
    function_table: &'a LuaTable<'a>,
}

impl<'a> Controller<'a> {
    pub fn new(lua: &'a Lua, function_table: &'a LuaTable) -> Self {
        Self {
            lua,
            function_table,
            enigo: Rc::new(Mutex::new(Enigo::new())),
        }
    }

    pub fn register_functions(&self) {
        self.sleep();
        self.get_arch();
        self.get_os_name();
        self.type_();
        self.get_display_height();
        self.get_display_width();
        self.mouse_left_click();
        self.mouse_right_click();
        self.mouse_move();
    }

    fn sleep(&self) {
        let _ = self.function_table.set(
            "sleep",
            self.lua
                .create_function(move |_, time: u64| {
                    sleep(Duration::from_millis(time));
                    Ok(())
                })
                .unwrap(),
        );
    }

    fn get_os_name(&self) {
        let _ = self.function_table.set(
            "get_os_name",
            self.lua
                .create_function(move |_, ()| -> LuaResult<String> {
                    Ok(std::env::consts::OS.to_string())
                })
                .unwrap(),
        );
    }

    fn get_arch(&self) {
        let _ = self.function_table.set(
            "get_arch",
            self.lua
                .create_function(move |_, ()| -> LuaResult<String> {
                    Ok(std::env::consts::ARCH.to_string())
                })
                .unwrap(),
        );
    }

    fn type_(&self) {
        let controller = self.enigo.clone();
        let _ = self.function_table.set(
            "type_text",
            self.lua
                .create_function(move |_, text: String| {
                    controller.lock().unwrap().key_sequence(&text);
                    Ok(())
                })
                .unwrap(),
        );
    }

    fn get_display_height(&self) {
        let controller = self.enigo.clone();
        let _ = self.function_table.set(
            "get_display_height",
            self.lua
                .create_function(move |_, ()| -> LuaResult<i32> {
                    Ok(controller.lock().unwrap().main_display_size().1)
                })
                .unwrap(),
        );
    }

    fn get_display_width(&self) {
        let controller = self.enigo.clone();
        let _ = self.function_table.set(
            "get_display_width",
            self.lua
                .create_function(move |_, ()| -> LuaResult<i32> {
                    Ok(controller.lock().unwrap().main_display_size().0)
                })
                .unwrap(),
        );
    }

    fn mouse_move(&self) {
        let controller = self.enigo.clone();
        let _ = self.function_table.set(
            "move_mouse",
            self.lua
                .create_function(move |_, (x, y): (i32, i32)| {
                    let (width, height) = controller.lock().unwrap().main_display_size();

                    if x > width || y > height {
                        println!(
                            "[Error]: Mouse move coords should be less than the display dimension."
                        );
                    }

                    controller.lock().unwrap().mouse_move_to(x, y);
                    Ok(())
                })
                .unwrap(),
        );
    }

    fn mouse_left_click(&self) {
        let controller = self.enigo.clone();
        let _ = self.function_table.set(
            "mouse_left_click",
            self.lua
                .create_function(move |_, ()| {
                    controller.lock().unwrap().mouse_click(MouseButton::Left);
                    Ok(())
                })
                .unwrap(),
        );
    }

    fn mouse_right_click(&self) {
        let controller = self.enigo.clone();
        let _ = self.function_table.set(
            "mouse_right_click",
            self.lua
                .create_function(move |_, ()| {
                    controller.lock().unwrap().mouse_click(MouseButton::Right);
                    Ok(())
                })
                .unwrap(),
        );
    }
}
