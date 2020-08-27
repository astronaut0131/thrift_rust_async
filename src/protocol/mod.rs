mod async_binary;

use crate::transport::{TAsyncReadTransport,TAsyncWriteTransport};

/// Empty traits just for abstraction, cauz we have to use async functions
pub trait TAsyncInputProtocol {}
pub trait TAsyncOutputProtocol{}

impl<P> TAsyncInputProtocol for Box<P>
    where
        P: TAsyncInputProtocol + ?Sized,
{}

impl<P> TAsyncOutputProtocol for Box<P>
    where
        P: TAsyncOutputProtocol + ?Sized,
{}

pub trait TAsyncInputProtocolFactory {
    // Create a `TAsyncInputProtocol` that reads bytes from `transport`.
    fn create(&self, transport: Box<dyn TAsyncReadTransport>) -> Box<dyn TAsyncInputProtocol>;
}

impl<T> TAsyncInputProtocolFactory for Box<T>
    where
        T: TAsyncInputProtocolFactory + ?Sized,
{
    fn create(&self, transport: Box<dyn TAsyncReadTransport>) -> Box<dyn TAsyncInputProtocol> {
        (**self).create(transport)
    }
}

pub trait TAsyncOutputProtocolFactory {
    /// Create a `TOutputProtocol` that writes bytes to `transport`.
    fn create(&self, transport: Box<dyn TAsyncWriteTransport>) -> Box<dyn TAsyncOutputProtocol>;
}

impl<T> TAsyncOutputProtocolFactory for Box<T>
    where
        T: TAsyncOutputProtocolFactory + ?Sized,
{
    fn create(&self, transport: Box<dyn TAsyncWriteTransport>) -> Box<dyn TAsyncOutputProtocol> {
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