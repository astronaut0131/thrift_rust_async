use std::convert::{From, TryFrom};
use std::io::Cursor;

use async_trait::async_trait;
use byteorder::{BigEndian, ByteOrder, ReadBytesExt, WriteBytesExt};

use crate::errors::{Error, ProtocolError, ProtocolErrorKind};
use crate::transport::{TAsyncReadTransport, TAsyncWriteTransport};

use super::{TAsyncInputProtocol, TAsyncInputProtocolFactory, TAsyncOutputProtocol, TAsyncOutputProtocolFactory};
use super::{
    TFieldIdentifier, TListIdentifier,
    TMapIdentifier, TMessageIdentifier,
    TMessageType, TSetIdentifier,
    TStructIdentifier, TType,
};

const BINARY_PROTOCOL_VERSION_1: u32 = 0x80010000;

#[derive(Debug)]
pub struct TAsyncBinaryInputProtocol<T>
    where
        T: TAsyncReadTransport,
{
    strict: bool,
    pub transport: T,
    // FIXME: shouldn't be public
    buf1: [u8; 1],
    buf2: [u8; 2],
    buf4: [u8; 4],
    buf8: [u8; 8],
}

impl<'a, T> TAsyncBinaryInputProtocol<T>
    where
        T: TAsyncReadTransport,
{
    /// Create a `TBinaryInputProtocol` that reads bytes from `transport`.
    ///
    /// Set `strict` to `true` if all incoming messages contain the protocol
    /// version number in the protocol header.
    pub fn new(transport: T, strict: bool) -> TAsyncBinaryInputProtocol<T> {
        TAsyncBinaryInputProtocol {
            strict,
            transport,
            buf1: [0; 1],
            buf2: [0; 2],
            buf4: [0; 4],
            buf8: [0; 8],
        }
    }
}

#[derive(Debug)]
pub struct TAsyncBinaryOutputProtocol<T>
    where
        T: TAsyncWriteTransport,
{
    strict: bool,
    pub transport: T,
    // FIXME: do not make public; only public for testing!
    buf1: [u8; 1],
    buf2: [u8; 2],
    buf4: [u8; 4],
    buf8: [u8; 8],
}


impl<T> TAsyncBinaryOutputProtocol<T>
    where
        T: TAsyncWriteTransport,
{
    /// Create a `TBinaryOutputProtocol` that writes bytes to `transport`.
    ///
    /// Set `strict` to `true` if all outgoing messages should contain the
    /// protocol version number in the protocol header.
    pub fn new(transport: T, strict: bool) -> TAsyncBinaryOutputProtocol<T> {
        TAsyncBinaryOutputProtocol {
            strict,
            transport,
            buf1: [0; 1],
            buf2: [0; 2],
            buf4: [0; 4],
            buf8: [0; 8],
        }
    }
}

/// Factory for creating instances of `TBinaryInputProtocol`.
#[derive(Default)]
pub struct TAsyncBinaryInputProtocolFactory;

impl TAsyncBinaryInputProtocolFactory {
    /// Create a `TBinaryInputProtocolFactory`.
    pub fn new() -> TAsyncBinaryInputProtocolFactory {
        TAsyncBinaryInputProtocolFactory {}
    }
}

impl TAsyncInputProtocolFactory for TAsyncBinaryInputProtocolFactory {
    fn create(&self, transport: Box<dyn TAsyncReadTransport + Send>) -> Box<dyn TAsyncInputProtocol + Send> {
        Box::new(TAsyncBinaryInputProtocol::new(transport, true))
    }
}

/// Factory for creating instances of `TBinaryOutputProtocol`.
#[derive(Default)]
pub struct TAsyncBinaryOutputProtocolFactory;

impl TAsyncBinaryOutputProtocolFactory {
    /// Create a `TBinaryOutputProtocolFactory`.
    pub fn new() -> TAsyncBinaryOutputProtocolFactory {
        TAsyncBinaryOutputProtocolFactory {}
    }
}

impl TAsyncOutputProtocolFactory for TAsyncBinaryOutputProtocolFactory {
    fn create(&self, transport: Box<dyn TAsyncWriteTransport + Send>) -> Box<dyn TAsyncOutputProtocol + Send> {
        Box::new(TAsyncBinaryOutputProtocol::new(transport, true))
    }
}

#[async_trait]
impl<T> TAsyncInputProtocol for TAsyncBinaryInputProtocol<T>
    where
        T: TAsyncReadTransport + std::marker::Send,
{
    #[cfg_attr(feature = "cargo-clippy", allow(collapsible_if))]
    async fn read_message_begin(&mut self) -> crate::Result<TMessageIdentifier> {
        self.transport.read(&mut self.buf4).await?;

        // the thrift version header is intentionally negative
        // so the first check we'll do is see if the sign bit is set
        // and if so - assume it's the protocol-version header
        if self.buf4[0] >= 8 {
            // apparently we got a protocol-version header - check
            // it, and if it matches, read the rest of the fields
            if self.buf4[0..2] != [0x80, 0x01] {
                Err(crate::errors::Error::Protocol(ProtocolError {
                    kind: ProtocolErrorKind::BadVersion,
                    message: format!("received bad version: {:?}", &self.buf4[0..2]),
                }))
            } else {
                let message_type: TMessageType = TryFrom::try_from(self.buf4[3])?;
                let name = self.read_string().await?;
                let sequence_number = self.read_i32().await?;
                Ok(TMessageIdentifier::new(name, message_type, sequence_number))
            }
        } else {
            // apparently we didn't get a protocol-version header,
            // which happens if the sender is not using the strict protocol
            if self.strict {
                // we're in strict mode however, and that always
                // requires the protocol-version header to be written first
                Err(crate::Error::Protocol(ProtocolError {
                    kind: ProtocolErrorKind::BadVersion,
                    message: format!("received bad version: {:?}", &self.buf4[0..2]),
                }))
            } else {
                // in the non-strict version the first message field
                // is the message name. strings (byte arrays) are length-prefixed,
                // so we've just read the length in the first 4 bytes
                let name_size = BigEndian::read_i32(&self.buf4) as usize;
                let mut name_buf: Vec<u8> = vec![0; name_size];
                self.transport.read(&mut name_buf).await?;
                let name = String::from_utf8(name_buf);

                // read the rest of the fields
                let message_type: TMessageType = self.read_byte().await.and_then(TryFrom::try_from)?;
                let sequence_number = self.read_i32().await?;
                Ok(TMessageIdentifier::new(name.unwrap(), message_type, sequence_number))
            }
        }
    }

    async fn read_message_end(&mut self) -> crate::Result<()> {
        Ok(())
    }

    async fn read_struct_begin(&mut self) -> crate::Result<Option<TStructIdentifier>> {
        Ok(None)
    }

    async fn read_struct_end(&mut self) -> crate::Result<()> {
        Ok(())
    }

    async fn read_field_begin(&mut self) -> crate::Result<TFieldIdentifier> {
        let field_type_byte = self.read_byte().await?;
        let field_type = field_type_from_u8(field_type_byte)?;
        let id = match field_type {
            TType::Stop => Ok(0),
            _ => self.read_i16().await,
        }?;
        Ok(TFieldIdentifier::new::<Option<String>, String, i16>(
            None, field_type, id,
        ))
    }

    async fn read_field_end(&mut self) -> crate::Result<()> {
        Ok(())
    }

    async fn read_bool(&mut self) -> crate::Result<bool> {
        let b = self.read_i8().await?;
        match b {
            0 => Ok(false),
            _ => Ok(true),
        }
    }

    async fn read_bytes(&mut self) -> crate::Result<Vec<u8>> {
        let num_bytes = self.read_i32().await? as usize;
        let mut buf = vec![0u8; num_bytes];
        self.transport
            .read(&mut buf).await
            .map(|_| buf)
            .map_err(From::from)
    }

    async fn read_i8(&mut self) -> crate::Result<i8> {
        self.transport.read(&mut self.buf1).await;

        Ok(self.buf1[0] as i8)
    }

    async fn read_i16(&mut self) -> crate::Result<i16> {
        self.transport.read(&mut self.buf2).await;

        Ok(BigEndian::read_i16(&mut self.buf2))
    }

    async fn read_i32(&mut self) -> crate::Result<i32> {
        self.transport.read(&mut self.buf4).await;

        Ok(BigEndian::read_i32(&mut self.buf4))
    }

    async fn read_i64(&mut self) -> crate::Result<i64> {
        self.transport.read(&mut self.buf8).await;

        Ok(BigEndian::read_i64(&mut self.buf8))
    }

    async fn read_double(&mut self) -> crate::Result<f64> {
        self.transport.read(&mut self.buf8).await;

        Ok(BigEndian::read_f64(&mut self.buf8))
    }

    async fn read_string(&mut self) -> crate::Result<String> {
        let bytes = self.read_bytes().await?;
        let res = String::from_utf8(bytes);

        Ok(res.unwrap())
    }

    async fn read_list_begin(&mut self) -> crate::Result<TListIdentifier> {
        let element_type: TType = self.read_byte().await.and_then(field_type_from_u8)?;
        let size = self.read_i32().await?;
        Ok(TListIdentifier::new(element_type, size))
    }

    async fn read_list_end(&mut self) -> crate::Result<()> {
        Ok(())
    }

    async fn read_set_begin(&mut self) -> crate::Result<TSetIdentifier> {
        let element_type: TType = self.read_byte().await.and_then(field_type_from_u8)?;
        let size = self.read_i32().await?;
        Ok(TSetIdentifier::new(element_type, size))
    }

    async fn read_set_end(&mut self) -> crate::Result<()> {
        Ok(())
    }

    async fn read_map_begin(&mut self) -> crate::Result<TMapIdentifier> {
        let key_type: TType = self.read_byte().await.and_then(field_type_from_u8)?;
        let value_type: TType = self.read_byte().await.and_then(field_type_from_u8)?;
        let size = self.read_i32().await?;
        Ok(TMapIdentifier::new(key_type, value_type, size))
    }

    async fn read_map_end(&mut self) -> crate::Result<()> {
        Ok(())
    }

    // utility
    //

    async fn read_byte(&mut self) -> crate::Result<u8> {
        self.transport.read(&mut self.buf1).await;

        Ok(self.buf1[0])
    }
}

#[async_trait]
impl<T> TAsyncOutputProtocol for TAsyncBinaryOutputProtocol<T>
    where
        T: TAsyncWriteTransport + std::marker::Send,
{
    async fn write_message_begin(&mut self, identifier: &TMessageIdentifier) -> crate::Result<()> {
        if self.strict {
            let message_type: u8 = identifier.message_type.into();
            let header = BINARY_PROTOCOL_VERSION_1 | (message_type as u32);
            // write u32
            let mut wtr = Vec::new();
            wtr.write_u32::<BigEndian>(header).unwrap();
            self.transport.write(&wtr).await;
            //
            self.write_string(&identifier.name).await?;
            self.write_i32(identifier.sequence_number).await
        } else {
            self.write_string(&identifier.name).await?;
            self.write_byte(identifier.message_type.into()).await?;
            self.write_i32(identifier.sequence_number).await
        }
    }

    async fn write_message_end(&mut self) -> crate::Result<()> {
        Ok(())
    }

    async fn write_struct_begin(&mut self, _: &TStructIdentifier) -> crate::Result<()> {
        Ok(())
    }

    async fn write_struct_end(&mut self) -> crate::Result<()> {
        Ok(())
    }

    async fn write_field_begin(&mut self, identifier: &TFieldIdentifier) -> crate::Result<()> {
        if identifier.id.is_none() && identifier.field_type != TType::Stop {
            return Err(crate::Error::Protocol(ProtocolError {
                kind: ProtocolErrorKind::Unknown,
                message: format!(
                    "cannot write identifier {:?} without sequence number",
                    &identifier
                ),
            }));
        }

        self.write_byte(field_type_to_u8(identifier.field_type)).await?;
        if let Some(id) = identifier.id {
            self.write_i16(id).await
        } else {
            Ok(())
        }
    }

    async fn write_field_end(&mut self) -> crate::Result<()> {
        Ok(())
    }

    async fn write_field_stop(&mut self) -> crate::Result<()> {
        self.write_byte(field_type_to_u8(TType::Stop)).await
    }

    async fn write_bool(&mut self, b: bool) -> crate::Result<()> {
        if b {
            self.write_i8(1).await
        } else {
            self.write_i8(0).await
        }
    }

    async fn write_bytes(&mut self, b: &[u8]) -> crate::Result<()> {
        self.write_i32(b.len() as i32).await?;
        self.transport.write(b).await;

        Ok(())
    }

    async fn write_i8(&mut self, i: i8) -> crate::Result<()> {
        self.transport.write(&[i as u8; 1]).await;

        Ok(())
    }

    async fn write_i16(&mut self, i: i16) -> crate::Result<()> {
        BigEndian::write_i16(&mut self.buf2, i);
        self.transport.write(&self.buf2).await;
        ///
        Ok(())
    }

    async fn write_i32(&mut self, i: i32) -> crate::Result<()> {
        BigEndian::write_i32(&mut self.buf4, i);
        self.transport.write(&self.buf4).await;

        Ok(())
    }

    async fn write_i64(&mut self, i: i64) -> crate::Result<()> {
        BigEndian::write_i64(&mut self.buf8, i);
        self.transport.write(&self.buf8).await;

        Ok(())
    }

    async fn write_double(&mut self, d: f64) -> crate::Result<()> {
        BigEndian::write_f64(&mut self.buf8, d);
        self.transport.write(&self.buf8).await;

        Ok(())
    }

    async fn write_string(&mut self, s: &str) -> crate::Result<()> {
        self.write_bytes(s.as_bytes()).await
    }

    async fn write_list_begin(&mut self, identifier: &TListIdentifier) -> crate::Result<()> {
        self.write_byte(field_type_to_u8(identifier.element_type)).await?;
        self.write_i32(identifier.size).await
    }

    async fn write_list_end(&mut self) -> crate::Result<()> {
        Ok(())
    }

    async fn write_set_begin(&mut self, identifier: &TSetIdentifier) -> crate::Result<()> {
        self.write_byte(field_type_to_u8(identifier.element_type)).await?;
        self.write_i32(identifier.size).await
    }

    async fn write_set_end(&mut self) -> crate::Result<()> {
        Ok(())
    }

    async fn write_map_begin(&mut self, identifier: &TMapIdentifier) -> crate::Result<()> {
        let key_type = identifier
            .key_type
            .expect("map identifier to write should contain key type");
        self.write_byte(field_type_to_u8(key_type)).await?;
        let val_type = identifier
            .value_type
            .expect("map identifier to write should contain value type");
        self.write_byte(field_type_to_u8(val_type)).await?;
        self.write_i32(identifier.size).await
    }

    async fn write_map_end(&mut self) -> crate::Result<()> {
        Ok(())
    }

    async fn flush(&mut self) -> crate::Result<()> {
        self.transport.flush().await.map_err(From::from)
    }

    // utility
    //

    async fn write_byte(&mut self, b: u8) -> crate::Result<()> {
        self.transport.write(&[b; 1]).await;

        Ok(())
    }
}

fn field_type_to_u8(field_type: TType) -> u8 {
    match field_type {
        TType::Stop => 0x00,
        TType::Void => 0x01,
        TType::Bool => 0x02,
        TType::I08 => 0x03, // equivalent to TType::Byte
        TType::Double => 0x04,
        TType::I16 => 0x06,
        TType::I32 => 0x08,
        TType::I64 => 0x0A,
        TType::String | TType::Utf7 => 0x0B,
        TType::Struct => 0x0C,
        TType::Map => 0x0D,
        TType::Set => 0x0E,
        TType::List => 0x0F,
        TType::Utf8 => 0x10,
        TType::Utf16 => 0x11,
    }
}

fn field_type_from_u8(b: u8) -> crate::Result<TType> {
    match b {
        0x00 => Ok(TType::Stop),
        0x01 => Ok(TType::Void),
        0x02 => Ok(TType::Bool),
        0x03 => Ok(TType::I08), // Equivalent to TType::Byte
        0x04 => Ok(TType::Double),
        0x06 => Ok(TType::I16),
        0x08 => Ok(TType::I32),
        0x0A => Ok(TType::I64),
        0x0B => Ok(TType::String), // technically, also a UTF7, but we'll treat it as string
        0x0C => Ok(TType::Struct),
        0x0D => Ok(TType::Map),
        0x0E => Ok(TType::Set),
        0x0F => Ok(TType::List),
        0x10 => Ok(TType::Utf8),
        0x11 => Ok(TType::Utf16),
        unkn => Err(Error::Protocol(ProtocolError {
            kind: ProtocolErrorKind::InvalidData,
            message: format!("cannot convert {} to TType", unkn),
        }))
    }
}