use fastbuf::{ReadBuf, WriteBuf};

pub trait VarInt: Sized {
    fn encode_var(&self, buf: &mut impl WriteBuf) -> Result<(), ()>;

    fn decode_var_from_buf(buf: &impl ReadBuf) -> Result<(Self, usize), ()>;

    fn decode_var(buf: &[u8]) -> Result<(Self, usize), ()>;
}

impl VarInt for i32 {
    fn encode_var(&self, buf: &mut impl WriteBuf) -> Result<(), ()> {
        (*self as u32).encode_var(buf)
    }

    fn decode_var_from_buf(buf: &impl ReadBuf) -> Result<(Self, usize), ()> {
        let (data, read_length) = u32::decode_var_from_buf(buf)?;
        Ok((data as i32, read_length))
    }

    fn decode_var(buf: &[u8]) -> Result<(Self, usize), ()> {
        let (data, read_length) = u32::decode_var(buf)?;
        Ok((data as i32, read_length))
    }
}

impl VarInt for u32 {
    fn encode_var(&self, buf: &mut impl WriteBuf) -> Result<(), ()> {
        let x = *self as u64;
        let stage1 = (x & 0x000000000000007f)
            | ((x & 0x0000000000003f80) << 1)
            | ((x & 0x00000000001fc000) << 2)
            | ((x & 0x000000000fe00000) << 3)
            | ((x & 0x00000000f0000000) << 4);

        let leading = stage1.leading_zeros();

        let unused_bytes = (leading - 1) >> 3;
        let bytes_needed = 8 - unused_bytes;

        // set all but the last MSBs
        let msbs = 0x8080808080808080;
        let msbmask = 0xffffffffffffffff >> (((8 - bytes_needed + 1) << 3) - 1);

        let merged = stage1 | (msbs & msbmask);
        let bytes = merged.to_le_bytes();

        buf.try_write(unsafe { bytes.get_unchecked(..bytes_needed as usize) })?;
        Ok(())
    }

    fn decode_var_from_buf(buf: &impl ReadBuf) -> Result<(Self, usize), ()> {
        let bytes = buf.get_continuous(u32::BITS as usize / 8 + 1);
        Self::decode_var(bytes)
    }

    fn decode_var(buf: &[u8]) -> Result<(Self, usize), ()> {
        let mut val = 0;
        for i in 0..5 {
            if buf.len() < i + 1 {
                Err(())?
            }
            let byte = *unsafe { buf.get_unchecked(i) };
            val |= (byte as i32 & 0b01111111) << (i * 7);
            if byte & 0b10000000 == 0 {
                return Ok((val as u32, i + 1));
            }
        }
        Err(())
    }
}
