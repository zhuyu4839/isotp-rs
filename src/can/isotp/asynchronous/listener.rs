use std::fmt::Display;
use crate::{IsoTpEvent, IsoTpFrame, IsoTpState, can::CanIsoTpFrame};
use crate::can::{isotp::AsyncCanIsoTp, frame::Frame};
use crate::device::Listener;

impl<C, Id, F> Listener<C, Id, F> for AsyncCanIsoTp<C, F>
where
    C: Clone + Eq + Display + Send + Sync,
    Id: PartialEq<u32>,
    F: Frame<Channel = C> + Clone + Send + Sync {

    fn on_frame_transmitting(&mut self, _: C, _: &F) {
    }

    fn on_frame_transmitted(&mut self, channel: C, id: Id) {
        if channel != self.channel {
            return;
        }

        if let Ok(address) = self.address.lock() {
            if id == address.tx_id ||
                id == address.fid {
                self.state_remove(IsoTpState::Sending);
            }
        }
    }

    fn on_frame_received(&mut self, channel: C, frames: &[F]) {
        if channel != self.channel
            || self.state_contains(IsoTpState::Error) {
            return;
        }

        let address_id = if let Ok(address) = self.address.lock() {
            Some((address.tx_id, address.rx_id))
        }
        else {
            None
        };

        if let Some(address) = address_id {
            for frame in frames {
                if frame.id().into_bits() == address.1 {
                    log::debug!("ISO-TP(CAN async) received: {:?} on {}", frame.data(), channel);

                    match CanIsoTpFrame::decode(frame.data()) {
                        Ok(frame) => match frame {
                            CanIsoTpFrame::SingleFrame { data } => {
                                self.on_single_frame(data);
                            }
                            CanIsoTpFrame::FirstFrame { length, data } => {
                                self.on_first_frame(address.0, length, data);
                            }
                            CanIsoTpFrame::ConsecutiveFrame { sequence, data } => {
                                self.on_consecutive_frame(sequence, data);
                            },
                            CanIsoTpFrame::FlowControlFrame(ctx) => {
                                self.on_flow_ctrl_frame(ctx);
                            },
                        },
                        Err(e) => {
                            log::warn!("ISO-TP(CAN async) - data convert to frame failed: {}", e);
                            self.state_append(IsoTpState::Error);
                            self.iso_tp_event(IsoTpEvent::ErrorOccurred(e));

                            break;
                        }
                    }
                }
            }
        }
    }
}