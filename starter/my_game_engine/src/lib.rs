// Definitions for the game engine
pub mod macros;
pub mod ffi;

// Tests for the game engine
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ffi::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    #[ignore]
    fn test_simple_game_loop() {
        start_window_and_game_loop!("Test simple game loop", 800, 600, 2, {
        });
    }

    #[test]
    #[ignore]
    fn test_sprite_rendering() {
        start_window_and_game_loop!("Test Sprite rendering", 800, 600, 2, {
            let sprite_ob = spawn_sprite!(100.0, 100.0, 50, 50, 255, 0, 0);
            assert_ne!(sprite_ob, std::ptr::null_mut());
            thread::sleep(Duration::from_secs(1));
        });
    }

    #[test]
    #[ignore]
    fn test_screen_clearing() {
        start_window_and_game_loop!("Test screen clearing", 800, 600, 2, {
            
            let red_sprite = spawn_sprite!(100.0, 100.0, 50, 50, 255, 0, 0);
            let green_sprite = spawn_sprite!(100.0, 100.0, 50, 50, 0, 255, 0);

            let start = std::time::Instant::now();

            while start.elapsed().as_secs() < 3 {
                clear();
                render(red_sprite);
                update_window();
                thread::sleep(Duration::from_millis(10));
            }

            while !should_close() {
                clear();
                render(green_sprite);
                update_window();
                thread::sleep(Duration::from_millis(10));
            }
        });
    }

    #[test]
    #[ignore]
    fn test_key_presses() {
        start_window_and_game_loop!("Test key presses", 800, 600, 2, {
            let green_sprite = spawn_sprite!(100.0, 100.0, 50, 50, 0, 255, 0);
            let mut right = false;

            on_key_press!(GLFW_KEY_RIGHT, {
                    right = true;
                });

                if right {
                    move_sprite!(true, green_sprite, 200.0, 100.0);
                    break;
                }
        });
    }

    #[test]
    #[ignore]
    fn test_sprite_position_update() {
        start_window_and_game_loop!("Test sprite position update", 800, 600, 2, {
            let mut x = 100.0;
            let mut y = 100.0;
            let sprite_ob = spawn_sprite!(x, y, 50, 50, 255, 0, 0);

            let start = std::time::Instant::now();

            while start.elapsed().as_secs() < 3 {
                x += 1.0;
                y += 1.0;
                move_sprite!(true, sprite_ob, x, y);
                update_window();
                thread::sleep(Duration::from_millis(10));
            }
        });
    }

}
