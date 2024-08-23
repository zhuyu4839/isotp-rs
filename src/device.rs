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
