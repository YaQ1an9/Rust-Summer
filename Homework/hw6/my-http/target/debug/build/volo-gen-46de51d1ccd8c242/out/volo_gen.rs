pub mod volo_gen {
    #![allow(warnings, clippy::all)]

    pub mod volo {

        pub mod redis {

            #[::async_trait::async_trait]
            pub trait ItemService {
                async fn get_item(
                    &self,
                    req: GetItemRequest,
                ) -> ::core::result::Result<GetItemResponse, ::volo_thrift::AnyhowError>;
            }
            pub struct ItemServiceServer<S> {
                inner: S, // handler
            }

            pub struct MkItemServiceGenericClient;

            pub type ItemServiceClient = ItemServiceGenericClient<
                ::volo::service::BoxCloneService<
                    ::volo_thrift::context::ClientContext,
                    ItemServiceRequestSend,
                    ::std::option::Option<ItemServiceResponseRecv>,
                    ::volo_thrift::Error,
                >,
            >;

            impl<S> ::volo::client::MkClient<::volo_thrift::Client<S>> for MkItemServiceGenericClient {
                type Target = ItemServiceGenericClient<S>;
                fn mk_client(&self, service: ::volo_thrift::Client<S>) -> Self::Target {
                    ItemServiceGenericClient(service)
                }
            }

            #[derive(Clone)]
            pub struct ItemServiceGenericClient<S>(pub ::volo_thrift::Client<S>);

            pub struct ItemServiceOneShotClient<S>(pub ::volo_thrift::Client<S>);

            impl<
                    S: ::volo::service::Service<
                            ::volo_thrift::context::ClientContext,
                            ItemServiceRequestSend,
                            Response = ::std::option::Option<ItemServiceResponseRecv>,
                            Error = ::volo_thrift::Error,
                        > + Send
                        + Sync
                        + 'static,
                > ItemServiceGenericClient<S>
            {
                pub fn with_callopt<
                    Opt: ::volo::client::Apply<::volo_thrift::context::ClientContext>,
                >(
                    self,
                    opt: Opt,
                ) -> ItemServiceOneShotClient<::volo::client::WithOptService<S, Opt>>
                {
                    ItemServiceOneShotClient(self.0.with_opt(opt))
                }

                pub async fn get_item(
                    &self,
                    req: GetItemRequest,
                ) -> ::std::result::Result<
                    GetItemResponse,
                    ::volo_thrift::error::ResponseError<std::convert::Infallible>,
                > {
                    let req = ItemServiceRequestSend::GetItem(ItemServiceGetItemArgsSend { req });
                    let mut cx = self.0.make_cx("GetItem", false);
                    #[allow(unreachable_patterns)]
                    let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
                        Some(ItemServiceResponseRecv::GetItem(
                            ItemServiceGetItemResultRecv::Ok(resp),
                        )) => Ok(resp),
                        None => unreachable!(),
                        _ => unreachable!(),
                    };
                    ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
                        let mut cache = cache.borrow_mut();
                        if cache.len() < cache.capacity() {
                            cache.push(cx);
                        }
                    });
                    resp
                }
            }

            impl<
                    S: ::volo::client::OneShotService<
                            ::volo_thrift::context::ClientContext,
                            ItemServiceRequestSend,
                            Response = ::std::option::Option<ItemServiceResponseRecv>,
                            Error = ::volo_thrift::Error,
                        > + Send
                        + Sync
                        + 'static,
                > ItemServiceOneShotClient<S>
            {
                pub async fn get_item(
                    self,
                    req: GetItemRequest,
                ) -> ::std::result::Result<
                    GetItemResponse,
                    ::volo_thrift::error::ResponseError<std::convert::Infallible>,
                > {
                    let req = ItemServiceRequestSend::GetItem(ItemServiceGetItemArgsSend { req });
                    let mut cx = self.0.make_cx("GetItem", false);
                    #[allow(unreachable_patterns)]
                    let resp =
                        match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
                            Some(ItemServiceResponseRecv::GetItem(
                                ItemServiceGetItemResultRecv::Ok(resp),
                            )) => Ok(resp),
                            None => unreachable!(),
                            _ => unreachable!(),
                        };
                    ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
                        let mut cache = cache.borrow_mut();
                        if cache.len() < cache.capacity() {
                            cache.push(cx);
                        }
                    });
                    resp
                }
            }

            pub struct ItemServiceClientBuilder {}

            impl ItemServiceClientBuilder {
                pub fn new(
                    service_name: impl AsRef<str>,
                ) -> ::volo_thrift::client::ClientBuilder<
                    ::volo::layer::Identity,
                    ::volo::layer::Identity,
                    MkItemServiceGenericClient,
                    ItemServiceRequestSend,
                    ItemServiceResponseRecv,
                    ::volo::net::dial::DefaultMakeTransport,
                    ::volo_thrift::codec::default::DefaultMakeCodec<
                        ::volo_thrift::codec::default::ttheader::MakeTTHeaderCodec<
                            ::volo_thrift::codec::default::framed::MakeFramedCodec<
                                ::volo_thrift::codec::default::thrift::MakeThriftCodec,
                            >,
                        >,
                    >,
                    ::volo::loadbalance::LbConfig<
                        ::volo::loadbalance::random::WeightedRandomBalance<()>,
                        ::volo::discovery::DummyDiscover,
                    >,
                > {
                    ::volo_thrift::client::ClientBuilder::new(
                        service_name,
                        MkItemServiceGenericClient,
                    )
                }
            }

            impl<S> ItemServiceServer<S>
            where
                S: ItemService + ::core::marker::Send + ::core::marker::Sync + 'static,
            {
                pub fn new(
                    inner: S,
                ) -> ::volo_thrift::server::Server<
                    Self,
                    ::volo::layer::Identity,
                    ItemServiceRequestRecv,
                    ::volo_thrift::codec::default::DefaultMakeCodec<
                        ::volo_thrift::codec::default::ttheader::MakeTTHeaderCodec<
                            ::volo_thrift::codec::default::framed::MakeFramedCodec<
                                ::volo_thrift::codec::default::thrift::MakeThriftCodec,
                            >,
                        >,
                    >,
                    ::volo_thrift::tracing::DefaultProvider,
                > {
                    ::volo_thrift::server::Server::new(Self { inner })
                }
            }

            impl<T>
                ::volo::service::Service<
                    ::volo_thrift::context::ServerContext,
                    ItemServiceRequestRecv,
                > for ItemServiceServer<T>
            where
                T: ItemService + Send + Sync + 'static,
            {
                type Response = ItemServiceResponseSend;
                type Error = ::anyhow::Error;

                type Future<'cx> = impl ::std::future::Future<
                        Output = ::std::result::Result<Self::Response, Self::Error>,
                    > + 'cx;

                fn call<'cx, 's>(
                    &'s self,
                    _cx: &'cx mut ::volo_thrift::context::ServerContext,
                    req: ItemServiceRequestRecv,
                ) -> Self::Future<'cx>
                where
                    's: 'cx,
                {
                    async move {
                        match req {
                            ItemServiceRequestRecv::GetItem(args) => {
                                Ok(ItemServiceResponseSend::GetItem(
                                    match self.inner.get_item(args.req).await {
                                        Ok(resp) => ItemServiceGetItemResultSend::Ok(resp),
                                        Err(err) => return Err(err),
                                    },
                                ))
                            }
                        }
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub enum ItemServiceRequestRecv {
                GetItem(ItemServiceGetItemArgsRecv),
            }

            #[derive(Debug, Clone)]
            pub enum ItemServiceRequestSend {
                GetItem(ItemServiceGetItemArgsSend),
            }

            #[derive(Debug, Clone)]
            pub enum ItemServiceResponseRecv {
                GetItem(ItemServiceGetItemResultRecv),
            }

            #[derive(Debug, Clone)]
            pub enum ItemServiceResponseSend {
                GetItem(ItemServiceGetItemResultSend),
            }

            #[::async_trait::async_trait]
            impl ::volo_thrift::EntryMessage for ItemServiceRequestRecv {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::core::result::Result<(), ::pilota::thrift::EncodeError> {
                    match self {
                        Self::GetItem(value) => ::pilota::thrift::Message::encode(value, protocol)
                            .map_err(|err| err.into()),
                    }
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                    msg_ident: &::pilota::thrift::TMessageIdentifier,
                ) -> ::core::result::Result<Self, ::pilota::thrift::DecodeError> {
                    Ok(match &*msg_ident.name {
                        "GetItem" => Self::GetItem(::pilota::thrift::Message::decode(protocol)?),
                        _ => {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::UnknownMethod,
                                format!("unknown method {}", msg_ident.name),
                            ));
                        }
                    })
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                    msg_ident: &::pilota::thrift::TMessageIdentifier,
                ) -> ::core::result::Result<Self, ::pilota::thrift::DecodeError> {
                    Ok(match &*msg_ident.name {
                        "GetItem" => {
                            Self::GetItem(::pilota::thrift::Message::decode_async(protocol).await?)
                        }
                        _ => {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::UnknownMethod,
                                format!("unknown method {}", msg_ident.name),
                            ));
                        }
                    })
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    match self {
                        Self::GetItem(value) => ::volo_thrift::Message::size(value, protocol),
                    }
                }
            }

            #[::async_trait::async_trait]
            impl ::volo_thrift::EntryMessage for ItemServiceRequestSend {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::core::result::Result<(), ::pilota::thrift::EncodeError> {
                    match self {
                        Self::GetItem(value) => ::pilota::thrift::Message::encode(value, protocol)
                            .map_err(|err| err.into()),
                    }
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                    msg_ident: &::pilota::thrift::TMessageIdentifier,
                ) -> ::core::result::Result<Self, ::pilota::thrift::DecodeError> {
                    Ok(match &*msg_ident.name {
                        "GetItem" => Self::GetItem(::pilota::thrift::Message::decode(protocol)?),
                        _ => {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::UnknownMethod,
                                format!("unknown method {}", msg_ident.name),
                            ));
                        }
                    })
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                    msg_ident: &::pilota::thrift::TMessageIdentifier,
                ) -> ::core::result::Result<Self, ::pilota::thrift::DecodeError> {
                    Ok(match &*msg_ident.name {
                        "GetItem" => {
                            Self::GetItem(::pilota::thrift::Message::decode_async(protocol).await?)
                        }
                        _ => {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::UnknownMethod,
                                format!("unknown method {}", msg_ident.name),
                            ));
                        }
                    })
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    match self {
                        Self::GetItem(value) => ::volo_thrift::Message::size(value, protocol),
                    }
                }
            }
            #[::async_trait::async_trait]
            impl ::volo_thrift::EntryMessage for ItemServiceResponseRecv {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::core::result::Result<(), ::pilota::thrift::EncodeError> {
                    match self {
                        Self::GetItem(value) => ::pilota::thrift::Message::encode(value, protocol)
                            .map_err(|err| err.into()),
                    }
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                    msg_ident: &::pilota::thrift::TMessageIdentifier,
                ) -> ::core::result::Result<Self, ::pilota::thrift::DecodeError> {
                    Ok(match &*msg_ident.name {
                        "GetItem" => Self::GetItem(::pilota::thrift::Message::decode(protocol)?),
                        _ => {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::UnknownMethod,
                                format!("unknown method {}", msg_ident.name),
                            ));
                        }
                    })
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                    msg_ident: &::pilota::thrift::TMessageIdentifier,
                ) -> ::core::result::Result<Self, ::pilota::thrift::DecodeError> {
                    Ok(match &*msg_ident.name {
                        "GetItem" => {
                            Self::GetItem(::pilota::thrift::Message::decode_async(protocol).await?)
                        }
                        _ => {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::UnknownMethod,
                                format!("unknown method {}", msg_ident.name),
                            ));
                        }
                    })
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    match self {
                        Self::GetItem(value) => ::volo_thrift::Message::size(value, protocol),
                    }
                }
            }

            #[::async_trait::async_trait]
            impl ::volo_thrift::EntryMessage for ItemServiceResponseSend {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::core::result::Result<(), ::pilota::thrift::EncodeError> {
                    match self {
                        Self::GetItem(value) => ::pilota::thrift::Message::encode(value, protocol)
                            .map_err(|err| err.into()),
                    }
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                    msg_ident: &::pilota::thrift::TMessageIdentifier,
                ) -> ::core::result::Result<Self, ::pilota::thrift::DecodeError> {
                    Ok(match &*msg_ident.name {
                        "GetItem" => Self::GetItem(::pilota::thrift::Message::decode(protocol)?),
                        _ => {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::UnknownMethod,
                                format!("unknown method {}", msg_ident.name),
                            ));
                        }
                    })
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                    msg_ident: &::pilota::thrift::TMessageIdentifier,
                ) -> ::core::result::Result<Self, ::pilota::thrift::DecodeError> {
                    Ok(match &*msg_ident.name {
                        "GetItem" => {
                            Self::GetItem(::pilota::thrift::Message::decode_async(protocol).await?)
                        }
                        _ => {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::UnknownMethod,
                                format!("unknown method {}", msg_ident.name),
                            ));
                        }
                    })
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    match self {
                        Self::GetItem(value) => ::volo_thrift::Message::size(value, protocol),
                    }
                }
            }
            impl ::std::convert::From<RedisCommand> for i32 {
                fn from(e: RedisCommand) -> Self {
                    e as _
                }
            }

            impl ::std::convert::TryFrom<i32> for RedisCommand {
                type Error = ::pilota::EnumConvertError<i32>;

                #[allow(non_upper_case_globals)]
                fn try_from(
                    v: i32,
                ) -> ::std::result::Result<Self, ::pilota::EnumConvertError<i32>> {
                    const Get: i32 = RedisCommand::Get as i32;
                    const Set: i32 = RedisCommand::Set as i32;
                    const Ping: i32 = RedisCommand::Ping as i32;
                    const Del: i32 = RedisCommand::Del as i32;
                    const Publish: i32 = RedisCommand::Publish as i32;
                    const Subscribe: i32 = RedisCommand::Subscribe as i32;
                    const Unkonwn: i32 = RedisCommand::Unkonwn as i32;
                    match v {
                        Get => ::std::result::Result::Ok(RedisCommand::Get),
                        Set => ::std::result::Result::Ok(RedisCommand::Set),
                        Ping => ::std::result::Result::Ok(RedisCommand::Ping),
                        Del => ::std::result::Result::Ok(RedisCommand::Del),
                        Publish => ::std::result::Result::Ok(RedisCommand::Publish),
                        Subscribe => ::std::result::Result::Ok(RedisCommand::Subscribe),
                        Unkonwn => ::std::result::Result::Ok(RedisCommand::Unkonwn),

                        _ => ::std::result::Result::Err(::pilota::EnumConvertError::InvalidNum(
                            v,
                            "RedisCommand",
                        )),
                    }
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
            #[derivative(Default)]
            #[derive(Clone, PartialEq)]
            #[repr(i32)]
            #[derive(Copy)]
            pub enum RedisCommand {
                #[derivative(Default)]
                Get = 0,

                Set = 1,

                Ping = 2,

                Del = 3,

                Publish = 4,

                Subscribe = 5,

                Unkonwn = 6,
            }

            #[::async_trait::async_trait]
            impl ::pilota::thrift::Message for RedisCommand {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    protocol.write_i32(*self as i32)?;
                    Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::{thrift::TLengthProtocolExt, Buf};
                    let value = protocol.read_i32()?;
                    Ok(::std::convert::TryFrom::try_from(value).map_err(|err| {
                        ::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            format!("invalid enum value for RedisCommand, value: {}", value),
                        )
                    })?)
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    let value = protocol.read_i32().await?;
                    Ok(::std::convert::TryFrom::try_from(value).map_err(|err| {
                        ::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            format!("invalid enum value for RedisCommand, value: {}", value),
                        )
                    })?)
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    protocol.i32_len(*self as i32)
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
            #[derivative(Default)]
            #[derive(Clone, PartialEq)]

            pub enum ItemServiceGetItemResultRecv {
                #[derivative(Default)]
                Ok(GetItemResponse),
            }

            #[::async_trait::async_trait]
            impl ::pilota::thrift::Message for ItemServiceGetItemResultRecv {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                        name: "ItemServiceGetItemResultRecv",
                    })?;
                    match self {
                        ItemServiceGetItemResultRecv::Ok(ref value) => {
                            protocol.write_struct_field(
                                0,
                                value,
                                ::pilota::thrift::TType::Struct,
                            )?;
                        }
                    }
                    protocol.write_field_stop()?;
                    protocol.write_struct_end()?;
                    Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::{thrift::TLengthProtocolExt, Buf};
                    let mut ret = None;
                    protocol.read_struct_begin()?;
                    loop {
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            protocol.field_stop_len();
                            break;
                        } else {
                            protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        match field_ident.id {
                            Some(0) => {
                                if ret.is_none() {
                                    let field_ident = ::pilota::thrift::Message::decode(protocol)?;
                                    protocol.struct_len(&field_ident);
                                    ret = Some(ItemServiceGetItemResultRecv::Ok(field_ident));
                                } else {
                                    return Err(::pilota::thrift::DecodeError::new(
                                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ));
                                }
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }
                    }
                    protocol.read_field_end()?;
                    protocol.read_struct_end()?;
                    if let Some(ret) = ret {
                        Ok(ret)
                    } else {
                        Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    let mut ret = None;
                    protocol.read_struct_begin().await?;
                    loop {
                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        match field_ident.id {
                            Some(0) => {
                                if ret.is_none() {
                                    let field_ident =
                                        ::pilota::thrift::Message::decode_async(protocol).await?;

                                    ret = Some(ItemServiceGetItemResultRecv::Ok(field_ident));
                                } else {
                                    return Err(::pilota::thrift::DecodeError::new(
                                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ));
                                }
                            }
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }
                    }
                    protocol.read_field_end().await?;
                    protocol.read_struct_end().await?;
                    if let Some(ret) = ret {
                        Ok(ret)
                    } else {
                        Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "ItemServiceGetItemResultRecv",
                    }) + match self {
                        ItemServiceGetItemResultRecv::Ok(ref value) => {
                            protocol.struct_field_len(Some(0), value)
                        }
                    } + protocol.field_stop_len()
                        + protocol.struct_end_len()
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
            pub struct ItemServiceGetItemArgsSend {
                pub req: GetItemRequest,
            }
            #[::async_trait::async_trait]
            impl ::pilota::thrift::Message for ItemServiceGetItemArgsSend {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    let struct_ident = ::pilota::thrift::TStructIdentifier {
                        name: "ItemServiceGetItemArgsSend",
                    };

                    protocol.write_struct_begin(&struct_ident)?;
                    protocol.write_struct_field(1, &self.req, ::pilota::thrift::TType::Struct)?;
                    protocol.write_field_stop()?;
                    protocol.write_struct_end()?;
                    Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::{thrift::TLengthProtocolExt, Buf};

                    let mut req = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin()?;
                    if let Err(err) = (|| {
                        loop {
                            let field_ident = protocol.read_field_begin()?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                protocol.field_stop_len();
                                break;
                            } else {
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    req = Some(::pilota::thrift::Message::decode(protocol)?);
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type)?;
                                }
                            }

                            protocol.read_field_end()?;
                            protocol.field_end_len();
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    })() {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::WithContext(
                                    ::std::boxed::Box::new(err),
                                ),
                                format!(
                                    "decode struct `ItemServiceGetItemArgsSend` field(#{}) failed",
                                    field_id
                                ),
                            ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end()?;

                    let Some(req) = req else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field req is required".to_string(),
                        ));
                    };

                    let data = Self { req };
                    Ok(data)
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    let mut req = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    req = Some(
                                        ::pilota::thrift::Message::decode_async(protocol).await?,
                                    );
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::WithContext(
                                    ::std::boxed::Box::new(err),
                                ),
                                format!(
                                    "decode struct `ItemServiceGetItemArgsSend` field(#{}) failed",
                                    field_id
                                ),
                            ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end().await?;

                    let Some(req) = req else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field req is required".to_string(),
                        ));
                    };

                    let data = Self { req };
                    Ok(data)
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "ItemServiceGetItemArgsSend",
                    }) + protocol.struct_field_len(Some(1), &self.req)
                        + protocol.field_stop_len()
                        + protocol.struct_end_len()
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
            pub struct GetItemRequest {
                pub cmd: RedisCommand,

                pub args: ::std::option::Option<::std::vec::Vec<::pilota::FastStr>>,
            }
            #[::async_trait::async_trait]
            impl ::pilota::thrift::Message for GetItemRequest {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    let struct_ident = ::pilota::thrift::TStructIdentifier {
                        name: "GetItemRequest",
                    };

                    protocol.write_struct_begin(&struct_ident)?;
                    protocol.write_i32_field(1, (*&self.cmd).into())?;
                    if let Some(value) = self.args.as_ref() {
                        protocol.write_list_field(
                            2,
                            ::pilota::thrift::TType::Binary,
                            &value,
                            |protocol, val| {
                                protocol.write_faststr((val).clone())?;
                                Ok(())
                            },
                        )?;
                    }
                    protocol.write_field_stop()?;
                    protocol.write_struct_end()?;
                    Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::{thrift::TLengthProtocolExt, Buf};

                    let mut cmd = None;
                    let mut args = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin()?;
                    if let Err(err) = (|| {
                        loop {
                            let field_ident = protocol.read_field_begin()?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                protocol.field_stop_len();
                                break;
                            } else {
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type == ::pilota::thrift::TType::I32 =>
                                {
                                    cmd = Some(::pilota::thrift::Message::decode(protocol)?);
                                }
                                Some(2)
                                    if field_ident.field_type == ::pilota::thrift::TType::List =>
                                {
                                    args = Some(unsafe {
                                        let list_ident = protocol.read_list_begin()?;
                                        let mut val: Vec<::pilota::FastStr> =
                                            Vec::with_capacity(list_ident.size);
                                        for i in 0..list_ident.size {
                                            val.as_mut_ptr()
                                                .offset(i as isize)
                                                .write(protocol.read_faststr()?);
                                        }
                                        val.set_len(list_ident.size);
                                        protocol.read_list_end()?;
                                        val
                                    });
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type)?;
                                }
                            }

                            protocol.read_field_end()?;
                            protocol.field_end_len();
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    })() {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::WithContext(
                                    ::std::boxed::Box::new(err),
                                ),
                                format!(
                                    "decode struct `GetItemRequest` field(#{}) failed",
                                    field_id
                                ),
                            ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end()?;

                    let Some(cmd) = cmd else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field cmd is required".to_string(),
                        ));
                    };

                    let data = Self { cmd, args };
                    Ok(data)
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    let mut cmd = None;
                    let mut args = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type == ::pilota::thrift::TType::I32 =>
                                {
                                    cmd = Some(
                                        ::pilota::thrift::Message::decode_async(protocol).await?,
                                    );
                                }
                                Some(2)
                                    if field_ident.field_type == ::pilota::thrift::TType::List =>
                                {
                                    args = Some({
                                        let list_ident = protocol.read_list_begin().await?;
                                        let mut val = Vec::with_capacity(list_ident.size);
                                        for _ in 0..list_ident.size {
                                            val.push(protocol.read_faststr().await?);
                                        }
                                        protocol.read_list_end().await?;
                                        val
                                    });
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::WithContext(
                                    ::std::boxed::Box::new(err),
                                ),
                                format!(
                                    "decode struct `GetItemRequest` field(#{}) failed",
                                    field_id
                                ),
                            ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end().await?;

                    let Some(cmd) = cmd else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field cmd is required".to_string(),
                        ));
                    };

                    let data = Self { cmd, args };
                    Ok(data)
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "GetItemRequest",
                    }) + protocol.i32_field_len(Some(1), (*&self.cmd).into())
                        + self.args.as_ref().map_or(0, |value| {
                            protocol.list_field_len(
                                Some(2),
                                ::pilota::thrift::TType::Binary,
                                value,
                                |protocol, el| protocol.faststr_len(el),
                            )
                        })
                        + protocol.field_stop_len()
                        + protocol.struct_end_len()
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
            pub struct ItemServiceGetItemArgsRecv {
                pub req: GetItemRequest,
            }
            #[::async_trait::async_trait]
            impl ::pilota::thrift::Message for ItemServiceGetItemArgsRecv {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    let struct_ident = ::pilota::thrift::TStructIdentifier {
                        name: "ItemServiceGetItemArgsRecv",
                    };

                    protocol.write_struct_begin(&struct_ident)?;
                    protocol.write_struct_field(1, &self.req, ::pilota::thrift::TType::Struct)?;
                    protocol.write_field_stop()?;
                    protocol.write_struct_end()?;
                    Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::{thrift::TLengthProtocolExt, Buf};

                    let mut req = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin()?;
                    if let Err(err) = (|| {
                        loop {
                            let field_ident = protocol.read_field_begin()?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                protocol.field_stop_len();
                                break;
                            } else {
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    req = Some(::pilota::thrift::Message::decode(protocol)?);
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type)?;
                                }
                            }

                            protocol.read_field_end()?;
                            protocol.field_end_len();
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    })() {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::WithContext(
                                    ::std::boxed::Box::new(err),
                                ),
                                format!(
                                    "decode struct `ItemServiceGetItemArgsRecv` field(#{}) failed",
                                    field_id
                                ),
                            ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end()?;

                    let Some(req) = req else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field req is required".to_string(),
                        ));
                    };

                    let data = Self { req };
                    Ok(data)
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    let mut req = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    req = Some(
                                        ::pilota::thrift::Message::decode_async(protocol).await?,
                                    );
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::WithContext(
                                    ::std::boxed::Box::new(err),
                                ),
                                format!(
                                    "decode struct `ItemServiceGetItemArgsRecv` field(#{}) failed",
                                    field_id
                                ),
                            ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end().await?;

                    let Some(req) = req else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field req is required".to_string(),
                        ));
                    };

                    let data = Self { req };
                    Ok(data)
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "ItemServiceGetItemArgsRecv",
                    }) + protocol.struct_field_len(Some(1), &self.req)
                        + protocol.field_stop_len()
                        + protocol.struct_end_len()
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
            #[derivative(Default)]
            #[derive(Clone, PartialEq)]

            pub enum ItemServiceGetItemResultSend {
                #[derivative(Default)]
                Ok(GetItemResponse),
            }

            #[::async_trait::async_trait]
            impl ::pilota::thrift::Message for ItemServiceGetItemResultSend {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                        name: "ItemServiceGetItemResultSend",
                    })?;
                    match self {
                        ItemServiceGetItemResultSend::Ok(ref value) => {
                            protocol.write_struct_field(
                                0,
                                value,
                                ::pilota::thrift::TType::Struct,
                            )?;
                        }
                    }
                    protocol.write_field_stop()?;
                    protocol.write_struct_end()?;
                    Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::{thrift::TLengthProtocolExt, Buf};
                    let mut ret = None;
                    protocol.read_struct_begin()?;
                    loop {
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            protocol.field_stop_len();
                            break;
                        } else {
                            protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        match field_ident.id {
                            Some(0) => {
                                if ret.is_none() {
                                    let field_ident = ::pilota::thrift::Message::decode(protocol)?;
                                    protocol.struct_len(&field_ident);
                                    ret = Some(ItemServiceGetItemResultSend::Ok(field_ident));
                                } else {
                                    return Err(::pilota::thrift::DecodeError::new(
                                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ));
                                }
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }
                    }
                    protocol.read_field_end()?;
                    protocol.read_struct_end()?;
                    if let Some(ret) = ret {
                        Ok(ret)
                    } else {
                        Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    let mut ret = None;
                    protocol.read_struct_begin().await?;
                    loop {
                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        match field_ident.id {
                            Some(0) => {
                                if ret.is_none() {
                                    let field_ident =
                                        ::pilota::thrift::Message::decode_async(protocol).await?;

                                    ret = Some(ItemServiceGetItemResultSend::Ok(field_ident));
                                } else {
                                    return Err(::pilota::thrift::DecodeError::new(
                                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ));
                                }
                            }
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }
                    }
                    protocol.read_field_end().await?;
                    protocol.read_struct_end().await?;
                    if let Some(ret) = ret {
                        Ok(ret)
                    } else {
                        Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "ItemServiceGetItemResultSend",
                    }) + match self {
                        ItemServiceGetItemResultSend::Ok(ref value) => {
                            protocol.struct_field_len(Some(0), value)
                        }
                    } + protocol.field_stop_len()
                        + protocol.struct_end_len()
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
            pub struct GetItemResponse {
                pub ok: bool,

                pub data: ::std::option::Option<::pilota::FastStr>,
            }
            #[::async_trait::async_trait]
            impl ::pilota::thrift::Message for GetItemResponse {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    let struct_ident = ::pilota::thrift::TStructIdentifier {
                        name: "GetItemResponse",
                    };

                    protocol.write_struct_begin(&struct_ident)?;
                    protocol.write_bool_field(1, *&self.ok)?;
                    if let Some(value) = self.data.as_ref() {
                        protocol.write_faststr_field(2, (value).clone())?;
                    }
                    protocol.write_field_stop()?;
                    protocol.write_struct_end()?;
                    Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    #[allow(unused_imports)]
                    use ::pilota::{thrift::TLengthProtocolExt, Buf};

                    let mut ok = None;
                    let mut data = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin()?;
                    if let Err(err) = (|| {
                        loop {
                            let field_ident = protocol.read_field_begin()?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                protocol.field_stop_len();
                                break;
                            } else {
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type == ::pilota::thrift::TType::Bool =>
                                {
                                    ok = Some(protocol.read_bool()?);
                                }
                                Some(2)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    data = Some(protocol.read_faststr()?);
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type)?;
                                }
                            }

                            protocol.read_field_end()?;
                            protocol.field_end_len();
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    })() {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::WithContext(
                                    ::std::boxed::Box::new(err),
                                ),
                                format!(
                                    "decode struct `GetItemResponse` field(#{}) failed",
                                    field_id
                                ),
                            ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end()?;

                    let Some(ok) = ok else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field ok is required".to_string(),
                        ));
                    };

                    let data = Self { ok, data };
                    Ok(data)
                }

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                    let mut ok = None;
                    let mut data = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type == ::pilota::thrift::TType::Bool =>
                                {
                                    ok = Some(protocol.read_bool().await?);
                                }
                                Some(2)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    data = Some(protocol.read_faststr().await?);
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::WithContext(
                                    ::std::boxed::Box::new(err),
                                ),
                                format!(
                                    "decode struct `GetItemResponse` field(#{}) failed",
                                    field_id
                                ),
                            ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end().await?;

                    let Some(ok) = ok else {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field ok is required".to_string(),
                        ));
                    };

                    let data = Self { ok, data };
                    Ok(data)
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "GetItemResponse",
                    }) + protocol.bool_field_len(Some(1), *&self.ok)
                        + self
                            .data
                            .as_ref()
                            .map_or(0, |value| protocol.faststr_field_len(Some(2), value))
                        + protocol.field_stop_len()
                        + protocol.struct_end_len()
                }
            }
        }
    }
}
