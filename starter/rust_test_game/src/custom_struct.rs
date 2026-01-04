use crossbeam_channel::{Receiver, Sender, unbounded};
use crate::messages::{ToMain, ToNetworking};

// Structs for channel senders and receivers
pub struct ToNetSender {
    pub inner: Sender<ToNetworking>,
}
pub struct ToNetReceiver {
    pub inner: Receiver<ToNetworking>,
}

pub struct ToMainSender {
    pub inner: Sender<ToMain>,
}
pub struct ToMainReceiver {
    pub inner: Receiver<ToMain>,
}

// Function to create channels for communication between threads
pub fn create_channels() -> (ToNetSender, ToNetReceiver, ToMainSender, ToMainReceiver) {
    let (tx_net, rx_net) = unbounded();
    let (tx_main, rx_main) = unbounded();

    (
        ToNetSender { inner: tx_net },
        ToNetReceiver { inner: rx_net },
        ToMainSender { inner: tx_main },
        ToMainReceiver { inner: rx_main },
    )
}
