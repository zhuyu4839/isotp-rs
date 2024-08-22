use crate::can::CanIsoTpFrame;
use crate::can::utils::parse;
use crate::can::constant::{CAN_FRAME_MAX_SIZE, CANFD_FRAME_MAX_SIZE, CONSECUTIVE_FRAME_SIZE, DEFAULT_PADDING, ISO_TP_MAX_LENGTH_2004, SINGLE_FRAME_SIZE_2004, FIRST_FRAME_SIZE_2004};
use crate::error::Error;
use crate::FrameType;

#[cfg(feature = "can-fd")]
use crate::can::utils::can_fd_resize;

pub(crate) fn decode_single(data: &[u8],
                            byte0: u8,
                            length: usize
) -> Result<CanIsoTpFrame, Error> {
    #[cfg(feature = "can-fd")]
    let max_len = CANFD_FRAME_MAX_SIZE;
    #[cfg(not(feature = "can-fd"))]
    let max_len = CAN_FRAME_MAX_SIZE;

    if length > max_len {
        return Err(Error::LengthOutOfRange(length));
    }

    let pdu_len = byte0 & 0x0F;
    if length < pdu_len as usize + 1 {
        return Err(Error::InvalidPdu(data.to_vec()));
    }

    Ok(CanIsoTpFrame::SingleFrame { data: Vec::from(&data[1..=pdu_len as usize]) })
}

pub(crate) fn decode_first(data: &[u8],
                           byte0: u8,
                           length: usize,
) -> Result<CanIsoTpFrame, Error> {
    #[cfg(not(feature = "can-fd"))]
    if length != CAN_FRAME_MAX_SIZE {
        return Err(Error::InvalidDataLength { actual: length, expect: CAN_FRAME_MAX_SIZE })
    }
    #[cfg(feature = "can-fd")]
    if length != CANFD_FRAME_MAX_SIZE {
        return Err(Error::InvalidDataLength { actual: length, expect: CANFD_FRAME_MAX_SIZE })
    }

    let pdu_len = (byte0 as u16 & 0x0F) << 8 | data[1] as u16;
    Ok(CanIsoTpFrame::FirstFrame { length: pdu_len as u32, data: Vec::from(&data[2..]) })
}

pub(crate) fn encode_single(mut data: Vec<u8>, padding: Option<u8>) -> Vec<u8> {
    let length = data.len();
    let mut result = vec![FrameType::Single as u8 | length as u8];
    result.append(&mut data);
    #[cfg(not(feature = "can-fd"))]
    result.resize(CAN_FRAME_MAX_SIZE, padding.unwrap_or(DEFAULT_PADDING));
    #[cfg(feature = "can-fd")]
    if let Some(resize) = can_fd_resize(length) {
        result.resize(resize, padding.unwrap_or(DEFAULT_PADDING));
    }

    result
}

pub(crate) fn encode_first(length: u32, mut data: Vec<u8>) -> Vec<u8> {
    let len_h = ((length & 0x0F00) >> 8) as u8;
    let len_l = (length & 0x00FF) as u8;
    let mut result = vec![FrameType::First as u8 | len_h, len_l];
    result.append(&mut data);
    result
}

pub(crate) fn new_single<T: AsRef<[u8]>>(data: T) -> Result<CanIsoTpFrame, Error> {
    let data = data.as_ref();
    let length = data.len();
    match length {
        0 => Err(Error::EmptyPdu),
        1..=SINGLE_FRAME_SIZE_2004 => {
            let mut result = vec![FrameType::Single as u8 | length as u8];
            result.append(&mut data.to_vec());
            result.resize(SINGLE_FRAME_SIZE_2004, DEFAULT_PADDING);
            Ok(CanIsoTpFrame::SingleFrame { data: result })
        },
        v => Err(Error::LengthOutOfRange(v)),
    }
}

pub(crate) fn from_data(data: &[u8]) -> Result<Vec<CanIsoTpFrame>, Error> {
    let length = data.len();
    match length {
        0 => Err(Error::EmptyPdu),
        1..=CONSECUTIVE_FRAME_SIZE => Ok(vec![CanIsoTpFrame::SingleFrame { data: data.to_vec() }]),
        ..=ISO_TP_MAX_LENGTH_2004 => {
            let mut offset = 0;
            let mut sequence = 1;
            let mut results = Vec::new();

            parse::<FIRST_FRAME_SIZE_2004>(data, &mut offset, &mut sequence, &mut results, length);

            Ok(results)
        },
        v => Err(Error::LengthOutOfRange(v)),
    }
}

