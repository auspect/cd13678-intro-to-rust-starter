use std::thread;

use crate::custom_struct::{ToMainSender, ToNetReceiver};
use crate::messages::{ToMain, ToNetworking};
use crate::sprite_data::SpriteData;

// URL of the remote server
const URL: &str =
    "https://get-random-sprite-data-dan-chiarlones-projects.vercel.app/api/handler";

// Function to spawn the networking thread
pub fn spawn_network_thread(
    rx: ToNetReceiver,
    tx: ToMainSender,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        loop {
            match rx.inner.recv() {
                Ok(ToNetworking::FetchSprite) => {
                    let result = fetch_sprite();
                    match result {
                        Ok(data) => {
                            let _ = tx.inner.send(ToMain::SpriteFetched(data));
                        }
                        Err(err) => {
                            let _ = tx.inner.send(ToMain::Error(err));
                        }
                    }
                }
                Ok(ToNetworking::Shutdown) => {
                    let _ = tx.inner.send(ToMain::ShutdownAck);
                    break;
                }
                Err(err) => {
                    let _ = tx.inner.send(ToMain::Error(format!(
                        "Channel recv error: {err}"
                    )));
                    break;
                }
            }
        }
    })
}

// Function to fetch sprite data from the remote server
fn fetch_sprite() -> Result<SpriteData, String> {
    let resp = reqwest::blocking::get(URL)
        .map_err(|e| format!("HTTP error: {e}"))?;
    let data: SpriteData = resp
        .json()
        .map_err(|e| format!("JSON error: {e}"))?;
    Ok(data)
}
