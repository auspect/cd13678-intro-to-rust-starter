
// Update window + sleep
#[macro_export]
macro_rules! tick {
    ($duration: expr) => {
        {
            $crate::ffi::update_window();
            std::thread::sleep($duration);
        }
    };
}

// Init window
#[macro_export]
macro_rules! init_game {
    ($title: expr, $width: expr, $height: expr) => {
        {
            $crate::ffi::create_window($title, $width, $height);
            println!("Window initialized: {}", $title);
        }
    };
}

// Start window
#[macro_export]
macro_rules! start_window_and_game_loop {
    ($title: expr, $width: expr, $height: expr, $duration: expr, $code: block) => {
        {
            // Initialize game window
            init_game!($title, $width, $height);
            // Game loop
            loop {
                // Check if window should close
                if $crate::ffi::should_close() {
                    println!("Closing window...");
                    break;
                }
                // Run code and tick
                $code
                $crate::tick!(std::time::Duration::from_millis($duration));
            }
        }
    };
}

// Take action if key pressed
#[macro_export]
macro_rules! on_key_press {
    ($key: expr, $action: block) => {
        if $crate::ffi::key_pressed($key) {
            $action
        }
    }
}

// Create and render sprite
#[macro_export]
macro_rules! spawn_sprite {
    ($x: expr, $y: expr, $w: expr, $h: expr, $r: expr, $g: expr, $b: expr) => {
        {
            let sprite_ob: *mut $crate::ffi::Sprite;
            sprite_ob = $crate::ffi::sprite($x, $y, $w, $h, $r, $g, $b);
            $crate::ffi::render(sprite_ob); // Render sprite
            sprite_ob // Return sprite object
        }
    }
}

// Move sprite
#[macro_export]
macro_rules! move_sprite {
    ($to_clear: expr, $sprite_ob: expr, $x: expr, $y: expr) => {
        {
            // Clear screen if needed
            if $to_clear {
                $crate::ffi::clear();
            }
            // Update sprite position and render
            $crate::ffi::update_sprite($sprite_ob, $x, $y);
            $crate::ffi::render($sprite_ob);
        }
    }
}

// Change sprite color
#[macro_export]
macro_rules! change_sprite_color {
    ($sprite_ob: expr, $new_r: expr, $new_g: expr, $new_b: expr) => {
        $crate::spawn_sprite!(
            ($sprite_ob).x,
            ($sprite_ob).y,
            ($sprite_ob).width,
            ($sprite_ob).height,
            $new_r,
            $new_g,
            $new_b
        )
    }
}