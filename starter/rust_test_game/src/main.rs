mod messages;
mod sprite_data;
mod networking;
mod custom_struct;

use custom_struct::create_channels;
use messages::{ToNetworking, ToMain};

use my_game_engine::start_window_and_game_loop;
use my_game_engine::on_key_press;
use my_game_engine::spawn_sprite;
use my_game_engine::init_game;
use my_game_engine::ffi::{
    clear,
    render,
    update_window,
    should_close,
    GLFW_KEY_SPACE,
};


fn main() {
    // Create channels
    let (to_net_tx, to_net_rx, to_main_tx, to_main_rx) = create_channels();


    // Spawn networking thread
    let net_handle = networking::spawn_network_thread(
        to_net_rx,
        to_main_tx,
    );

    // Store sprites created from network data
    let mut sprites: Vec<*mut my_game_engine::ffi::Sprite> = Vec::new();

    // Start game window and loop
    start_window_and_game_loop!("Rust Test Game", 800, 600, 16, {
        // Check for messages from networking thread
        while let Ok(message) = to_main_rx.inner.try_recv() {
            match message {
                ToMain::ShutdownAck => {
                    println!("Networking thread acknowledged shutdown.");
                    break;
                }
                ToMain::SpriteFetched(data) => {
                    println!("Sprite fetched: {:?}", data);

                    // Create sprite and store it
                    let sprite = spawn_sprite!(
                        data.x,
                        data.y,
                        data.width,
                        data.height,
                        data.r,
                        data.g,
                        data.b
                    );
                    sprites.push(sprite);
                }
                ToMain::Error(err) => {
                    eprintln!("Error: {}", err);
                }
            }
        }

        // On space key press, request sprite fetch
        // (we could use other keys/events if needed)
        on_key_press!(GLFW_KEY_SPACE, {
            let _ = to_net_tx.inner.send(ToNetworking::FetchSprite);
        });

        // Render all sprites and update window
        clear();
        for &s in &sprites {
            render(s);
        }
        update_window();

        // If window should close, signal networking thread to shutdown
        if should_close() {
            let _ = to_net_tx.inner.send(ToNetworking::Shutdown);
        }
    });

    // Wait for networking thread to finish
    let _ = net_handle.join();
}
