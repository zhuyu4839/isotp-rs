//! ISO-TP device trait define.

use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex, MutexGuard};

pub trait Listener<Channel, Id, Frame>: Send {
    /// Callback when frame transmitting.
    fn on_frame_transmitting(&mut self, channel: Channel, frame: &Frame);
    /// Callback when frame transmit success.
    fn on_frame_transmitted(&mut self, channel: Channel, id: Id);
    /// Callback when frames received.
    fn on_frame_received(&mut self, channel: Channel, frames: &[Frame]);
}

/// The synchronous device trait define
/// Sync driver of the device implements this trait.
///
/// # Examples
///
/// ```ignore
/// use std::sync::{Arc, Mutex, MutexGuard};
/// use std::sync::mpsc::{Receiver, Sender};
/// use isotp_rs::can::{Address, SyncCanIsoTp};
/// use isotp_rs::device::{Listener, SyncDevice};
///
/// use isotp_rs::{IsoTpEvent, IsoTpEventListener};
///
/// pub struct MyIsoTpEventListener {
///     // field of struct
/// }
///
/// impl IsoTpEventListener for MyIsoTpEventListener {
///     // impl functions
/// }
///
/// pub struct MyDevice {
///     // fields of struct
/// }
///
/// impl SyncDevice for MyDevice {
///     // impl functions
/// }
///
/// fn main() {
///     let mut device = MyDevice {};
///     let listener = MyIsoTpEventListener {};
///     let address = Address {
///         tx_id: 0x70,
///         rx_id: 0x78,
///         fid: 0x7DF,
///     };
///
///     device.sync_start(100);
///
///     let mut isotp = SyncCanIsoTp::new(0, address, device.sender(), Box::new(listener));
///     device.register_listener("ISO-TP".into(), Box::new(isotp.clone()));
///
///     isotp.write(false, vec![0x10, 0x01]).unwrap();
///
///     device.close();
/// }
/// ```
pub trait SyncDevice {
    type Device;
    type Channel;
    type Id;
    type Frame;

    fn new(device: Self::Device) -> Self;
    /// Get the sender for transmit frame.
    fn sender(&self) -> Sender<Self::Frame>;
    /// Register transmit and receive frame listener.
    fn register_listener(
        &mut self,
        name: String,
        listener: Box<dyn Listener<Self::Channel, Self::Id, Self::Frame>>,
    ) -> bool;
    /// Unregister transmit and receive frame listener.
    fn unregister_listener(&mut self, name: String) -> bool;
    /// Unregister all transmit and receive frame listeners.
    fn unregister_all(&mut self) -> bool;
    /// Get all transmit and receive frame listener's names.
    fn listener_names(&self) -> Vec<String>;
    /// transmit loop.
    fn sync_transmit(device: MutexGuard<Self>,
                     interval_us: u64,
                     stopper: Arc<Mutex<Receiver<()>>>,
    );
    /// receive loop.
    fn sync_receive(device: MutexGuard<Self>,
                    interval_us: u64,
                    stopper: Arc<Mutex<Receiver<()>>>,
    );
    /// start [`Self::sync_transmit`] and [`Self::sync_receive`]
    fn sync_start(&mut self, interval_us: u64);
    /// Close the device and stop transmit and receive loop.
    fn close(&mut self);
}

/// The synchronous device trait define
/// Async driver of the device implements this trait.
///
/// # Examples
///
/// ```ignore
/// use std::sync::{Arc, Mutex, MutexGuard};
/// use std::sync::mpsc::{Receiver, Sender};
/// use isotp_rs::can::{Address, AsyncCanIsoTp};
/// use isotp_rs::device::{AsyncDevice, Listener};
///
/// use isotp_rs::{IsoTpEvent, IsoTpEventListener};
///
/// pub struct MyIsoTpEventListener {
///     // field of struct
/// }
///
/// impl IsoTpEventListener for MyIsoTpEventListener {
///     // impl functions
/// }
///
/// pub struct MyDevice {
///     // fields of struct
/// }
///
/// impl AsyncDevice for MyDevice {
///     // impl functions
/// }
///
/// async fn main() {
///     let mut device = MyDevice {};
///     let listener = MyIsoTpEventListener {};
///     let address = Address {
///         tx_id: 0x70,
///         rx_id: 0x78,
///         fid: 0x7DF,
///     };
///
///     let mut isotp = AsyncCanIsoTp::new(0, address, device.sender(), Box::new(listener));
///     device.register_listener("ISO-TP".into(), Box::new(isotp.clone()));
///
///     isotp.write(false, vec![0x10, 0x01]).await.unwrap();
///
///     device.close();
/// }
/// ```
pub trait AsyncDevice {
    type Device;
    type Channel;
    type Id;
    type Frame;

    fn new(device: Self::Device) -> Self;
    /// Get the sender for transmit frame.
    fn sender(&self) -> Sender<Self::Frame>;
    /// Register transmit and receive frame listener.
    fn register_listener(
        &mut self,
        name: String,
        listener: Box<dyn Listener<Self::Channel, Self::Id, Self::Frame>>,
    ) -> bool;
    /// Unregister transmit and receive frame listener.
    fn unregister_listener(&mut self, name: String) -> bool;
    /// Unregister all transmit and receive frame listeners.
    fn unregister_all(&mut self) -> bool;
    /// Get all transmit and receive frame listener's names.
    fn listener_names(&self) -> Vec<String>;
    /// transmit loop.
    fn async_transmit(device: Arc<Mutex<Self>>,
                      interval_us: u64,
                      stopper: Arc<Mutex<Receiver<()>>>,
    ) -> impl std::future::Future<Output = ()> + Send;
    /// receive loop.
    fn async_receive(device: Arc<Mutex<Self>>,
                     interval_us: u64,
                     stopper: Arc<Mutex<Receiver<()>>>,
    ) -> impl std::future::Future<Output = ()> + Send;
    /// start [`Self::async_transmit`] and [`Self::async_receive`]
    fn async_start(&mut self, interval_us: u64);
    /// Close the device and stop transmit and receive loop.
    fn close(&mut self) -> impl std::future::Future<Output = ()> + Send;
}
