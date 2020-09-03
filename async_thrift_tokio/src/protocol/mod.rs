use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Display, Formatter};

use async_trait::async_trait;

use crate::errors::{ProtocolError, ProtocolErrorKind};
use crate::transport::{TAsyncReadTransport, TAsyncWriteTransport};

pub mod async_binary;

// Default maximum depth to which `TInputProtocol::skip` will skip a Thrift
// field. A default is necessary because Thrift structs or collections may
// contain nested structs and collections, which could result in indefinite
// recursion.
const MAXIMUM_SKIP_DEPTH: i8 = 64;

#[async_trait]
pub trait TAsyncInputProtocol: Send {
    /// Read the beginning of a Thrift message.
    async fn read_message_begin(&mut self) -> crate::Result<TMessageIdentifier>;
    /// Read the end of a Thrift message.
    async fn read_message_end(&mut self) -> crate::Result<()>;
    /// Read the beginning of a Thrift struct.
    async fn read_struct_begin(&mut self) -> crate::Result<Option<TStructIdentifier>>;
    /// Read the end of a Thrift struct.
    async fn read_struct_end(&mut self) -> crate::Result<()>;
    /// Read the beginning of a Thrift struct field.
    async fn read_field_begin(&mut self) -> crate::Result<TFieldIdentifier>;
    /// Read the end of a Thrift struct field.
    async fn read_field_end(&mut self) -> crate::Result<()>;
    /// Read a bool.
    async fn read_bool(&mut self) -> crate::Result<bool>;
    /// Read a fixed-length byte array.
    async fn read_bytes(&mut self) -> crate::Result<Vec<u8>>;
    /// Read a word.
    async fn read_i8(&mut self) -> crate::Result<i8>;
    /// Read a 16-bit signed integer.
    async fn read_i16(&mut self) -> crate::Result<i16>;
    /// Read a 32-bit signed integer.
    async fn read_i32(&mut self) -> crate::Result<i32>;
    /// Read a 64-bit signed integer.
    async fn read_i64(&mut self) -> crate::Result<i64>;
    /// Read a 64-bit float.
    async fn read_double(&mut self) -> crate::Result<f64>;
    /// Read a fixed-length string (not null terminated).
    async fn read_string(&mut self) -> crate::Result<String>;
    /// Read the beginning of a list.
    async fn read_list_begin(&mut self) -> crate::Result<TListIdentifier>;
    /// Read the end of a list.
    async fn read_list_end(&mut self) -> crate::Result<()>;
    /// Read the beginning of a set.
    async fn read_set_begin(&mut self) -> crate::Result<TSetIdentifier>;
    /// Read the end of a set.
    async fn read_set_end(&mut self) -> crate::Result<()>;
    /// Read the beginning of a map.
    async fn read_map_begin(&mut self) -> crate::Result<TMapIdentifier>;
    /// Read the end of a map.
    async fn read_map_end(&mut self) -> crate::Result<()>;
    /// Skip a field with type `field_type` recursively until the default
    /// maximum skip depth is reached.
    async fn skip(&mut self, field_type: TType) -> crate::Result<()> {
        self.skip_till_depth(field_type, MAXIMUM_SKIP_DEPTH).await
    }
    /// Skip a field with type `field_type` recursively up to `depth` levels.
    async fn skip_till_depth(&mut self, field_type: TType, depth: i8) -> crate::Result<()> {
        if depth == 0 {
            return Err(crate::Error::Protocol(ProtocolError {
                kind: ProtocolErrorKind::DepthLimit,
                message: format!("cannot parse past {:?}", field_type),
            }));
        }

        match field_type {
            TType::Bool => self.read_bool().await.map(|_| ()),
            TType::I08 => self.read_i8().await.map(|_| ()),
            TType::I16 => self.read_i16().await.map(|_| ()),
            TType::I32 => self.read_i32().await.map(|_| ()),
            TType::I64 => self.read_i64().await.map(|_| ()),
            TType::Double => self.read_double().await.map(|_| ()),
            TType::String => self.read_string().await.map(|_| ()),
            TType::Struct => {
                self.read_struct_begin().await?;
                loop {
                    let field_ident = self.read_field_begin().await?;
                    if field_ident.field_type == TType::Stop {
                        break;
                    }
                    self.skip_till_depth(field_ident.field_type, depth - 1).await?;
                }
                self.read_struct_end().await
            }
            TType::List => {
                let list_ident = self.read_list_begin().await?;
                for _ in 0..list_ident.size {
                    self.skip_till_depth(list_ident.element_type, depth - 1).await?;
                }
                self.read_list_end().await
            }
            TType::Set => {
                let set_ident = self.read_set_begin().await?;
                for _ in 0..set_ident.size {
                    self.skip_till_depth(set_ident.element_type, depth - 1).await?;
                }
                self.read_set_end().await
            }
            TType::Map => {
                let map_ident = self.read_map_begin().await?;
                for _ in 0..map_ident.size {
                    let key_type = map_ident
                        .key_type
                        .expect("non-zero sized map should contain key type");
                    let val_type = map_ident
                        .value_type
                        .expect("non-zero sized map should contain value type");
                    self.skip_till_depth(key_type, depth - 1).await?;
                    self.skip_till_depth(val_type, depth - 1).await?;
                }
                self.read_map_end().await
            }
            u => Err(crate::Error::Protocol(ProtocolError {
                kind: ProtocolErrorKind::Unknown,
                message: format!("cannot skip field type {:?}", &u),
            })),
        }
    }

    // utility (DO NOT USE IN GENERATED CODE!!!!)
    //

    /// Read an unsigned byte.
    ///
    /// This method should **never** be used in generated code.
    async fn read_byte(&mut self) -> crate::Result<u8>;
}

#[async_trait]
pub trait TAsyncOutputProtocol: Send {
    /// Write the beginning of a Thrift message.
    async fn write_message_begin(&mut self, identifier: &TMessageIdentifier) -> crate::Result<()>;
    /// Write the end of a Thrift message.
    async fn write_message_end(&mut self) -> crate::Result<()>;
    /// Write the beginning of a Thrift struct.
    async fn write_struct_begin(&mut self, identifier: &TStructIdentifier) -> crate::Result<()>;
    /// Write the end of a Thrift struct.
    async fn write_struct_end(&mut self) -> crate::Result<()>;
    /// Write the beginning of a Thrift field.
    async fn write_field_begin(&mut self, identifier: &TFieldIdentifier) -> crate::Result<()>;
    /// Write the end of a Thrift field.
    async fn write_field_end(&mut self) -> crate::Result<()>;
    /// Write a STOP field indicating that all the fields in a struct have been
    /// written.
    async fn write_field_stop(&mut self) -> crate::Result<()>;
    /// Write a bool.
    async fn write_bool(&mut self, b: bool) -> crate::Result<()>;
    /// Write a fixed-length byte array.
    async fn write_bytes(&mut self, b: &[u8]) -> crate::Result<()>;
    /// Write an 8-bit signed integer.
    async fn write_i8(&mut self, i: i8) -> crate::Result<()>;
    /// Write a 16-bit signed integer.
    async fn write_i16(&mut self, i: i16) -> crate::Result<()>;
    /// Write a 32-bit signed integer.
    async fn write_i32(&mut self, i: i32) -> crate::Result<()>;
    /// Write a 64-bit signed integer.
    async fn write_i64(&mut self, i: i64) -> crate::Result<()>;
    /// Write a 64-bit float.
    async fn write_double(&mut self, d: f64) -> crate::Result<()>;
    /// Write a fixed-length string.
    async fn write_string(&mut self, s: &str) -> crate::Result<()>;
    /// Write the beginning of a list.
    async fn write_list_begin(&mut self, identifier: &TListIdentifier) -> crate::Result<()>;
    /// Write the end of a list.
    async fn write_list_end(&mut self) -> crate::Result<()>;
    /// Write the beginning of a set.
    async fn write_set_begin(&mut self, identifier: &TSetIdentifier) -> crate::Result<()>;
    /// Write the end of a set.
    async fn write_set_end(&mut self) -> crate::Result<()>;
    /// Write the beginning of a map.
    async fn write_map_begin(&mut self, identifier: &TMapIdentifier) -> crate::Result<()>;
    /// Write the end of a map.
    async fn write_map_end(&mut self) -> crate::Result<()>;
    /// Flush buffered bytes to the underlying transport.
    async fn flush(&mut self) -> crate::Result<()>;

    // utility (DO NOT USE IN GENERATED CODE!!!!)
    //

    /// Write an unsigned byte.
    ///
    /// This method should **never** be used in generated code.
    async fn write_byte(&mut self, b: u8) -> crate::Result<()>; // FIXME: REMOVE
}

#[async_trait]
impl<P> TAsyncInputProtocol for Box<P>
    where
        P: TAsyncInputProtocol + ?Sized + Send,
{
    async fn read_message_begin(&mut self) -> crate::Result<TMessageIdentifier> {
        (**self).read_message_begin().await
    }

    async fn read_message_end(&mut self) -> crate::Result<()> {
        (**self).read_message_end().await
    }

    async fn read_struct_begin(&mut self) -> crate::Result<Option<TStructIdentifier>> {
        (**self).read_struct_begin().await
    }

    async fn read_struct_end(&mut self) -> crate::Result<()> {
        (**self).read_struct_end().await
    }

    async fn read_field_begin(&mut self) -> crate::Result<TFieldIdentifier> {
        (**self).read_field_begin().await
    }

    async fn read_field_end(&mut self) -> crate::Result<()> {
        (**self).read_field_end().await
    }

    async fn read_bool(&mut self) -> crate::Result<bool> {
        (**self).read_bool().await
    }

    async fn read_bytes(&mut self) -> crate::Result<Vec<u8>> {
        (**self).read_bytes().await
    }

    async fn read_i8(&mut self) -> crate::Result<i8> {
        (**self).read_i8().await
    }

    async fn read_i16(&mut self) -> crate::Result<i16> {
        (**self).read_i16().await
    }

    async fn read_i32(&mut self) -> crate::Result<i32> {
        (**self).read_i32().await
    }

    async fn read_i64(&mut self) -> crate::Result<i64> {
        (**self).read_i64().await
    }

    async fn read_double(&mut self) -> crate::Result<f64> {
        (**self).read_double().await
    }

    async fn read_string(&mut self) -> crate::Result<String> {
        (**self).read_string().await
    }

    async fn read_list_begin(&mut self) -> crate::Result<TListIdentifier> {
        (**self).read_list_begin().await
    }

    async fn read_list_end(&mut self) -> crate::Result<()> {
        (**self).read_list_end().await
    }

    async fn read_set_begin(&mut self) -> crate::Result<TSetIdentifier> {
        (**self).read_set_begin().await
    }

    async fn read_set_end(&mut self) -> crate::Result<()> {
        (**self).read_set_end().await
    }

    async fn read_map_begin(&mut self) -> crate::Result<TMapIdentifier> {
        (**self).read_map_begin().await
    }

    async fn read_map_end(&mut self) -> crate::Result<()> {
        (**self).read_map_end().await
    }

    async fn read_byte(&mut self) -> crate::Result<u8> {
        (**self).read_byte().await
    }
}

#[async_trait]
impl<P> TAsyncOutputProtocol for Box<P>
    where
        P: TAsyncOutputProtocol + ?Sized + Send,
{
    async fn write_message_begin(&mut self, identifier: &TMessageIdentifier) -> crate::Result<()> {
        (**self).write_message_begin(identifier).await
    }

    async fn write_message_end(&mut self) -> crate::Result<()> {
        (**self).write_message_end().await
    }

    async fn write_struct_begin(&mut self, identifier: &TStructIdentifier) -> crate::Result<()> {
        (**self).write_struct_begin(identifier).await
    }

    async fn write_struct_end(&mut self) -> crate::Result<()> {
        (**self).write_struct_end().await
    }

    async fn write_field_begin(&mut self, identifier: &TFieldIdentifier) -> crate::Result<()> {
        (**self).write_field_begin(identifier).await
    }

    async fn write_field_end(&mut self) -> crate::Result<()> {
        (**self).write_field_end().await
    }

    async fn write_field_stop(&mut self) -> crate::Result<()> {
        (**self).write_field_stop().await
    }

    async fn write_bool(&mut self, b: bool) -> crate::Result<()> {
        (**self).write_bool(b).await
    }

    async fn write_bytes(&mut self, b: &[u8]) -> crate::Result<()> {
        (**self).write_bytes(b).await
    }

    async fn write_i8(&mut self, i: i8) -> crate::Result<()> {
        (**self).write_i8(i).await
    }

    async fn write_i16(&mut self, i: i16) -> crate::Result<()> {
        (**self).write_i16(i).await
    }

    async fn write_i32(&mut self, i: i32) -> crate::Result<()> {
        (**self).write_i32(i).await
    }

    async fn write_i64(&mut self, i: i64) -> crate::Result<()> {
        (**self).write_i64(i).await
    }

    async fn write_double(&mut self, d: f64) -> crate::Result<()> {
        (**self).write_double(d).await
    }

    async fn write_string(&mut self, s: &str) -> crate::Result<()> {
        (**self).write_string(s).await
    }

    async fn write_list_begin(&mut self, identifier: &TListIdentifier) -> crate::Result<()> {
        (**self).write_list_begin(identifier).await
    }

    async fn write_list_end(&mut self) -> crate::Result<()> {
        (**self).write_list_end().await
    }

    async fn write_set_begin(&mut self, identifier: &TSetIdentifier) -> crate::Result<()> {
        (**self).write_set_begin(identifier).await
    }

    async fn write_set_end(&mut self) -> crate::Result<()> {
        (**self).write_set_end().await
    }

    async fn write_map_begin(&mut self, identifier: &TMapIdentifier) -> crate::Result<()> {
        (**self).write_map_begin(identifier).await
    }

    async fn write_map_end(&mut self) -> crate::Result<()> {
        (**self).write_map_end().await
    }

    async fn flush(&mut self) -> crate::Result<()> {
        (**self).flush().await
    }

    async fn write_byte(&mut self, b: u8) -> crate::Result<()> {
        (**self).write_byte(b).await
    }
}

pub trait TAsyncInputProtocolFactory {
    // Create a `TAsyncInputProtocol` that reads bytes from `transport`.
    fn create(&self, transport: Box<dyn TAsyncReadTransport + Send>) -> Box<dyn TAsyncInputProtocol + Send>;
}

impl<T> TAsyncInputProtocolFactory for Box<T>
    where
        T: TAsyncInputProtocolFactory + ?Sized,
{
    fn create(&self, transport: Box<dyn TAsyncReadTransport + Send>) -> Box<dyn TAsyncInputProtocol + Send> {
        (**self).create(transport)
    }
}

pub trait TAsyncOutputProtocolFactory {
    /// Create a `TOutputProtocol` that writes bytes to `transport`.
    fn create(&self, transport: Box<dyn TAsyncWriteTransport + Send>) -> Box<dyn TAsyncOutputProtocol + Send>;
}

impl<T> TAsyncOutputProtocolFactory for Box<T>
    where
        T: TAsyncOutputProtocolFactory + ?Sized,
{
    fn create(&self, transport: Box<dyn TAsyncWriteTransport + Send>) -> Box<dyn TAsyncOutputProtocol + Send> {
        (**self).create(transport)
    }
}

/// Thrift message identifier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TMessageIdentifier {
    /// Service call the message is associated with.
    pub name: String,
    /// Message type.
    pub message_type: TMessageType,
    /// Ordered sequence number identifying the message.
    pub sequence_number: i32,
}

impl TMessageIdentifier {
    /// Create a `TMessageIdentifier` for a Thrift service-call named `name`
    /// with message type `message_type` and sequence number `sequence_number`.
    pub fn new<S: Into<String>>(
        name: S,
        message_type: TMessageType,
        sequence_number: i32,
    ) -> TMessageIdentifier {
        TMessageIdentifier {
            name: name.into(),
            message_type: message_type,
            sequence_number: sequence_number,
        }
    }
}

/// Thrift struct identifier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TStructIdentifier {
    /// Name of the encoded Thrift struct.
    pub name: String,
}

impl TStructIdentifier {
    /// Create a `TStructIdentifier` for a struct named `name`.
    pub fn new<S: Into<String>>(name: S) -> TStructIdentifier {
        TStructIdentifier { name: name.into() }
    }
}

/// Thrift field identifier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TFieldIdentifier {
    /// Name of the Thrift field.
    ///
    /// `None` if it's not sent over the wire.
    pub name: Option<String>,
    /// Field type.
    ///
    /// This may be a primitive, container, or a struct.
    pub field_type: TType,
    /// Thrift field id.
    ///
    /// `None` only if `field_type` is `TType::Stop`.
    pub id: Option<i16>,
}

impl TFieldIdentifier {
    /// Create a `TFieldIdentifier` for a field named `name` with type
    /// `field_type` and field id `id`.
    ///
    /// `id` should be `None` if `field_type` is `TType::Stop`.
    pub fn new<N, S, I>(name: N, field_type: TType, id: I) -> TFieldIdentifier
        where
            N: Into<Option<S>>,
            S: Into<String>,
            I: Into<Option<i16>>,
    {
        TFieldIdentifier {
            name: name.into().map(|n| n.into()),
            field_type: field_type,
            id: id.into(),
        }
    }
}

/// Thrift list identifier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TListIdentifier {
    /// Type of the elements in the list.
    pub element_type: TType,
    /// Number of elements in the list.
    pub size: i32,
}

impl TListIdentifier {
    /// Create a `TListIdentifier` for a list with `size` elements of type
    /// `element_type`.
    pub fn new(element_type: TType, size: i32) -> TListIdentifier {
        TListIdentifier {
            element_type: element_type,
            size: size,
        }
    }
}

/// Thrift set identifier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TSetIdentifier {
    /// Type of the elements in the set.
    pub element_type: TType,
    /// Number of elements in the set.
    pub size: i32,
}

impl TSetIdentifier {
    /// Create a `TSetIdentifier` for a set with `size` elements of type
    /// `element_type`.
    pub fn new(element_type: TType, size: i32) -> TSetIdentifier {
        TSetIdentifier {
            element_type: element_type,
            size: size,
        }
    }
}

/// Thrift map identifier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TMapIdentifier {
    /// Map key type.
    pub key_type: Option<TType>,
    /// Map value type.
    pub value_type: Option<TType>,
    /// Number of entries in the map.
    pub size: i32,
}

impl TMapIdentifier {
    /// Create a `TMapIdentifier` for a map with `size` entries of type
    /// `key_type -> value_type`.
    pub fn new<K, V>(key_type: K, value_type: V, size: i32) -> TMapIdentifier
        where
            K: Into<Option<TType>>,
            V: Into<Option<TType>>,
    {
        TMapIdentifier {
            key_type: key_type.into(),
            value_type: value_type.into(),
            size: size,
        }
    }
}

/// Thrift message types.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TMessageType {
    /// Service-call request.
    Call,
    /// Service-call response.
    Reply,
    /// Unexpected error in the remote service.
    Exception,
    /// One-way service-call request (no response is expected).
    OneWay,
}

/// Thrift struct-field types.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TType {
    /// Indicates that there are no more serialized fields in this Thrift struct.
    Stop,
    /// Void (`()`) field.
    Void,
    /// Boolean.
    Bool,
    /// Signed 8-bit int.
    I08,
    /// Double-precision number.
    Double,
    /// Signed 16-bit int.
    I16,
    /// Signed 32-bit int.
    I32,
    /// Signed 64-bit int.
    I64,
    /// UTF-8 string.
    String,
    /// UTF-7 string. *Unsupported*.
    Utf7,
    /// Thrift struct.
    Struct,
    /// Map.
    Map,
    /// Set.
    Set,
    /// List.
    List,
    /// UTF-8 string.
    Utf8,
    /// UTF-16 string. *Unsupported*.
    Utf16,
}

impl Display for TMessageType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            TMessageType::Call => write!(f, "Call"),
            TMessageType::Reply => write!(f, "Reply"),
            TMessageType::Exception => write!(f, "Exception"),
            TMessageType::OneWay => write!(f, "OneWay"),
        }
    }
}

impl From<TMessageType> for u8 {
    fn from(message_type: TMessageType) -> Self {
        match message_type {
            TMessageType::Call => 0x01,
            TMessageType::Reply => 0x02,
            TMessageType::Exception => 0x03,
            TMessageType::OneWay => 0x04,
        }
    }
}

impl TryFrom<u8> for TMessageType {
    type Error = crate::errors::Error;
    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            0x01 => Ok(TMessageType::Call),
            0x02 => Ok(TMessageType::Reply),
            0x03 => Ok(TMessageType::Exception),
            0x04 => Ok(TMessageType::OneWay),
            unkn => Err(crate::Error::Protocol(ProtocolError {
                kind: ProtocolErrorKind::InvalidData,
                message: format!("cannot convert {} to TMessageType", unkn),
            })),
        }
    }
}

/// Compare the expected message sequence number `expected` with the received
/// message sequence number `actual`.
///
/// Return `()` if `actual == expected`, `Err` otherwise.
pub fn verify_expected_sequence_number(expected: i32, actual: i32) -> crate::Result<()> {
    if expected == actual {
        Ok(())
    } else {
        Err(crate::Error::Application(crate::ApplicationError {
            kind: crate::ApplicationErrorKind::BadSequenceId,
            message: format!("expected {} got {}", expected, actual),
        }))
    }
}

/// Compare the expected service-call name `expected` with the received
/// service-call name `actual`.
///
/// Return `()` if `actual == expected`, `Err` otherwise.
pub fn verify_expected_service_call(expected: &str, actual: &str) -> crate::Result<()> {
    if expected == actual {
        Ok(())
    } else {
        Err(crate::Error::Application(crate::ApplicationError {
            kind: crate::ApplicationErrorKind::WrongMethodName,
            message: format!("expected {} got {}", expected, actual),
        }))
    }
}

/// Compare the expected message type `expected` with the received message type
/// `actual`.
///
/// Return `()` if `actual == expected`, `Err` otherwise.
pub fn verify_expected_message_type(expected: TMessageType, actual: TMessageType) -> crate::Result<()> {
    if expected == actual {
        Ok(())
    } else {
        Err(crate::Error::Application(crate::ApplicationError {
            kind: crate::ApplicationErrorKind::InvalidMessageType,
            message: format!("expected {} got {}", expected, actual),
        }))
    }
}

/// Check if a required Thrift struct field exists.
///
/// Return `()` if it does, `Err` otherwise.
pub fn verify_required_field_exists<T>(field_name: &str, field: &Option<T>) -> crate::Result<()> {
    match *field {
        Some(_) => Ok(()),
        None => Err(crate::Error::Protocol(crate::ProtocolError {
            kind: crate::ProtocolErrorKind::Unknown,
            message: format!("missing required field {}", field_name),
        })),
    }
}

/// Extract the field id from a Thrift field identifier.
///
/// `field_ident` must *not* have `TFieldIdentifier.field_type` of type `TType::Stop`.
///
/// Return `TFieldIdentifier.id` if an id exists, `Err` otherwise.
pub fn field_id(field_ident: &TFieldIdentifier) -> crate::Result<i16> {
    field_ident.id.ok_or_else(|| {
        crate::Error::Protocol(crate::ProtocolError {
            kind: crate::ProtocolErrorKind::Unknown,
            message: format!("missing field in in {:?}", field_ident),
        })
    })
}
