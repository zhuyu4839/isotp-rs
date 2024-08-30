mod listener;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use tokio::time::sleep;
use std::time::Duration;
use crate::{FlowControlContext, FlowControlState, IsoTpEvent, IsoTpEventListener, IsoTpFrame, IsoTpState, can::{Address, CanIsoTpFrame, context::IsoTpContext, frame::Frame}};
use crate::error::Error;

#[derive(Clone)]
pub struct AsyncCanIsoTp<C, F> {
    pub(crate) channel: C,
    pub(crate) address: Arc<Mutex<Address>>,
    pub(crate) sender: Sender<F>,
    pub(crate) context: Arc<Mutex<IsoTpContext>>,
    pub(crate) state: Arc<Mutex<IsoTpState>>,
    pub(crate) listener: Arc<Mutex<Box<dyn IsoTpEventListener>>>,
}

unsafe impl<C, F> Send for AsyncCanIsoTp<C, F> {}

impl<C: Clone, F: Frame<Channel = C>> AsyncCanIsoTp<C, F> {

    pub fn new(channel: C,
               address: Address,
               sender: Sender<F>,
               listener: Box<dyn IsoTpEventListener>
    ) -> Self {
        Self {
            channel,
            address: Arc::new(Mutex::new(address)),
            sender,
            context: Default::default(),
            state: Default::default(),
            listener: Arc::new(Mutex::new(listener)),
        }
    }

    #[inline]
    pub fn update_address(&self, address: Address) {
        if let Ok(mut addr) = self.address.lock() {
            *addr = address;
        }
    }

    pub async fn write(&self, functional: bool, data: Vec<u8>) -> Result<(), Error> {
        log::debug!("ISO-TP(CAN async) - Sending: {:?}", data);

        let frames = CanIsoTpFrame::from_data(data)?;
        let frame_len = frames.len();

        let can_id = match self.address.lock() {
            Ok(address) => if functional { Ok(address.fid) } else { Ok(address.tx_id) },
            Err(_) => Err(Error::ContextError("can't get address context".into())),
        }?;
        for (index, frame) in frames.into_iter().enumerate() {
            self.write_waiting(index).await?;
            let mut frame = F::from_iso_tp(can_id, frame, None)
                .ok_or(Error::ConvertError {
                    src: "iso-tp frame",
                    target: "can-frame",
                })?;
            frame.set_channel(self.channel.clone());

            self.state_append(IsoTpState::Sending);
            if 0 == index && 1 < frame_len  {
                self.state_append(IsoTpState::WaitFlowCtrl);
            }
            self.sender.send(frame)
                .map_err(|e| {
                    log::warn!("ISO-TP(CAN async) - transmit failed: {:?}", e);
                    Error::DeviceError
                })?;
        }

        Ok(())
    }

    #[inline]
    pub(crate) fn on_single_frame(&self, data: Vec<u8>) {
        self.iso_tp_event(IsoTpEvent::DataReceived(data));
    }

    #[inline]
    pub(crate) fn on_first_frame(&self, tx_id: u32, length: u32, data: Vec<u8>) {
        self.update_consecutive(length, data);

        let iso_tp_frame = CanIsoTpFrame::default_flow_ctrl_frame();
        match F::from_iso_tp(tx_id, iso_tp_frame, None) {
            Some(mut frame) => {
                frame.set_channel(self.channel.clone());

                self.state_append(IsoTpState::Sending);
                match self.sender.send(frame) {
                    Ok(_) => {
                        self.iso_tp_event(IsoTpEvent::FirstFrameReceived);
                    },
                    Err(e) => {
                        log::warn!("ISO-TP - transmit failed: {:?}", e);
                        self.state_append(IsoTpState::Error);

                        self.iso_tp_event(IsoTpEvent::ErrorOccurred(Error::DeviceError));
                    },
                }
            },
            None => log::error!("ISO-TP: convert `iso-tp frame` to `can-frame` error"),
        }
    }

    #[inline]
    pub(crate) fn on_consecutive_frame(&self, sequence: u8, data: Vec<u8>) {
        match self.append_consecutive(sequence, data) {
            Ok(event) => {
                match event {
                    IsoTpEvent::DataReceived(_) => {
                        self.context_reset();
                    },
                    _ => {},
                }
                self.iso_tp_event(event);
            },
            Err(e) => {
                self.state_append(IsoTpState::Error);
                self.iso_tp_event(IsoTpEvent::ErrorOccurred(e));
            }
        }
    }

    #[inline]
    pub(crate) fn on_flow_ctrl_frame(&self, ctx: FlowControlContext) {
        match ctx.state() {
            FlowControlState::Continues => {
                self.state_remove(IsoTpState::WaitBusy | IsoTpState::WaitFlowCtrl);
            },
            FlowControlState::Wait => {
                self.state_append(IsoTpState::WaitBusy);
                self.iso_tp_event(IsoTpEvent::Wait);
                return;
            }
            FlowControlState::Overload => {
                self.state_append(IsoTpState::Error);
                self.iso_tp_event(IsoTpEvent::ErrorOccurred(Error::OverloadFlow));
                return;
            }
        }

        self.update_flow_ctrl(ctx);
    }

    fn iso_tp_event(&self, event: IsoTpEvent) {
        match self.listener.lock() {
            Ok(mut listener) => {
                // println!("ISO-TP(CAN asyn): Sending iso-tp event: {:?}", event);
                log::trace!("ISO-TP(CAN asyn): Sending iso-tp event: {:?}", event);
                listener.on_iso_tp_event(event);
            },
            Err(_) => log::warn!("ISO-TP(CAN async): Sending event failed"),
        }
    }

    async fn write_waiting(&self, index: usize) -> Result<(), Error> {
        match self.context.lock() {
            Ok(ctx) => {
                if let Some(ctx) = &ctx.flow_ctrl {
                    if ctx.block_size != 0 &&
                        0 == ctx.block_size as usize % (index + 1) {
                        self.state_append(IsoTpState::WaitFlowCtrl);
                    }
                    sleep(Duration::from_micros(ctx.st_min as u64)).await;
                }

                Ok(())
            },
            Err(_) => Err(Error::ContextError("can't get `context`".into()))
        }?;

        loop {
            if self.state_contains(IsoTpState::Error) {
                return Err(Error::DeviceError);
            }

            if self.state_contains(IsoTpState::Sending | IsoTpState::WaitBusy | IsoTpState::WaitFlowCtrl) {
                sleep(Duration::from_micros(10)).await;
            }
            else {
                break;
            }
        }

        Ok(())
    }

    fn update_flow_ctrl(&self, ctx: FlowControlContext) {
        if let Ok(mut context) = self.context.lock() {
            context.update_flow_ctrl(ctx);
        };
    }

    fn append_consecutive(&self, sequence: u8, data: Vec<u8>) -> Result<IsoTpEvent, Error> {
        match self.context.lock() {
            Ok(mut context) => {
                context.append_consecutive(sequence, data)
            },
            Err(_) => Err(Error::ContextError("can't get `context`".into()))
        }
    }

    fn update_consecutive(&self, length: u32, data: Vec<u8>) {
        if let Ok(mut context) = self.context.lock() {
            context.update_consecutive(length, data);
        }
    }

    fn context_reset(&self) {
        if let Ok(mut context) = self.context.lock() {
            context.reset();
        };
    }

    #[inline]
    fn state_contains(&self, flags: IsoTpState) -> bool {
        match self.state.lock() {
            Ok(v) => *v & flags != IsoTpState::Idle,
            Err(_) => {
                log::warn!("ISO-TP: state mutex is poisoned");
                false
            },
        }
    }

    #[inline]
    fn state_append(&self, flags: IsoTpState) {
        match self.state.lock() {
            Ok(mut v) => {
                if flags.contains(IsoTpState::Error) {
                    *v = IsoTpState::Error;
                }
                else {
                    *v |= flags;
                }
            }
            Err(_) => log::warn!("ISO-TP: state mutex is poisoned"),
        }
    }

    #[inline]
    fn state_remove(&self, flags: IsoTpState) {
        match self.state.lock() {
            Ok(mut v) => v.remove(flags),
            Err(_) => log::warn!("ISO-TP: state mutex is poisoned"),
        }
    }
}
