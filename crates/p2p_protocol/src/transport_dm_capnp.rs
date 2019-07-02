// Generated by the capnpc-rust plugin to the Cap'n Proto schema compiler.
// DO NOT EDIT.
// source: transport_dm.capnp


pub mod message {
  pub use self::Which::{MsgError,MsgChannelCreate,MsgChannelClose,MsgChannelMessage,MsgRelayRequest,MsgRelayAccept,MsgRelayChannelCreate,MsgRelayChannelClose,MsgRelayChannelMessage};

  #[derive(Copy, Clone)]
  pub struct Owned;
  impl <'a> ::capnp::traits::Owned<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
  impl <'a> ::capnp::traits::OwnedStruct<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
  impl ::capnp::traits::Pipelined for Owned { type Pipeline = Pipeline; }

  #[derive(Clone, Copy)]
  pub struct Reader<'a> { reader: ::capnp::private::layout::StructReader<'a> }

  impl <'a,> ::capnp::traits::HasTypeId for Reader<'a,>  {
    #[inline]
    fn type_id() -> u64 { _private::TYPE_ID }
  }
  impl <'a,> ::capnp::traits::FromStructReader<'a> for Reader<'a,>  {
    fn new(reader: ::capnp::private::layout::StructReader<'a>) -> Reader<'a,> {
      Reader { reader: reader,  }
    }
  }

  impl <'a,> ::capnp::traits::FromPointerReader<'a> for Reader<'a,>  {
    fn get_from_pointer(reader: &::capnp::private::layout::PointerReader<'a>, default: ::std::option::Option<&'a [::capnp::Word]>) -> ::capnp::Result<Reader<'a,>> {
      ::std::result::Result::Ok(::capnp::traits::FromStructReader::new(reader.get_struct(default)?))
    }
  }

  impl <'a,> ::capnp::traits::IntoInternalStructReader<'a> for Reader<'a,>  {
    fn into_internal_struct_reader(self) -> ::capnp::private::layout::StructReader<'a> {
      self.reader
    }
  }

  impl <'a,> ::capnp::traits::Imbue<'a> for Reader<'a,>  {
    fn imbue(&mut self, cap_table: &'a ::capnp::private::layout::CapTable) {
      self.reader.imbue(::capnp::private::layout::CapTableReader::Plain(cap_table))
    }
  }

  impl <'a,> Reader<'a,>  {
    pub fn reborrow(&self) -> Reader<> {
      Reader { .. *self }
    }

    pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
      self.reader.total_size()
    }
    pub fn has_msg_error(&self) -> bool {
      if self.reader.get_data_field::<u16>(0) != 0 { return false; }
      !self.reader.get_pointer_field(0).is_null()
    }
    pub fn has_msg_channel_create(&self) -> bool {
      if self.reader.get_data_field::<u16>(0) != 1 { return false; }
      !self.reader.get_pointer_field(0).is_null()
    }
    pub fn has_msg_channel_message(&self) -> bool {
      if self.reader.get_data_field::<u16>(0) != 3 { return false; }
      !self.reader.get_pointer_field(0).is_null()
    }
    pub fn has_msg_relay_channel_create(&self) -> bool {
      if self.reader.get_data_field::<u16>(0) != 6 { return false; }
      !self.reader.get_pointer_field(0).is_null()
    }
    pub fn has_msg_relay_channel_message(&self) -> bool {
      if self.reader.get_data_field::<u16>(0) != 8 { return false; }
      !self.reader.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn which(self) -> ::std::result::Result<WhichReader<'a,>, ::capnp::NotInSchema> {
      match self.reader.get_data_field::<u16>(0) {
        0 => {
          ::std::result::Result::Ok(MsgError(
            ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(0), ::std::option::Option::None)
          ))
        }
        1 => {
          ::std::result::Result::Ok(MsgChannelCreate(
            ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(0), ::std::option::Option::None)
          ))
        }
        2 => {
          ::std::result::Result::Ok(MsgChannelClose(
            self.reader.get_data_field::<u32>(1)
          ))
        }
        3 => {
          ::std::result::Result::Ok(MsgChannelMessage(
            ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(0), ::std::option::Option::None)
          ))
        }
        4 => {
          ::std::result::Result::Ok(MsgRelayRequest(
            ()
          ))
        }
        5 => {
          ::std::result::Result::Ok(MsgRelayAccept(
            ()
          ))
        }
        6 => {
          ::std::result::Result::Ok(MsgRelayChannelCreate(
            ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(0), ::std::option::Option::None)
          ))
        }
        7 => {
          ::std::result::Result::Ok(MsgRelayChannelClose(
            self.reader.get_data_field::<u32>(1)
          ))
        }
        8 => {
          ::std::result::Result::Ok(MsgRelayChannelMessage(
            ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(0), ::std::option::Option::None)
          ))
        }
        x => ::std::result::Result::Err(::capnp::NotInSchema(x))
      }
    }
  }

  pub struct Builder<'a> { builder: ::capnp::private::layout::StructBuilder<'a> }
  impl <'a,> ::capnp::traits::HasStructSize for Builder<'a,>  {
    #[inline]
    fn struct_size() -> ::capnp::private::layout::StructSize { _private::STRUCT_SIZE }
  }
  impl <'a,> ::capnp::traits::HasTypeId for Builder<'a,>  {
    #[inline]
    fn type_id() -> u64 { _private::TYPE_ID }
  }
  impl <'a,> ::capnp::traits::FromStructBuilder<'a> for Builder<'a,>  {
    fn new(builder: ::capnp::private::layout::StructBuilder<'a>) -> Builder<'a, > {
      Builder { builder: builder,  }
    }
  }

  impl <'a,> ::capnp::traits::ImbueMut<'a> for Builder<'a,>  {
    fn imbue_mut(&mut self, cap_table: &'a mut ::capnp::private::layout::CapTable) {
      self.builder.imbue(::capnp::private::layout::CapTableBuilder::Plain(cap_table))
    }
  }

  impl <'a,> ::capnp::traits::FromPointerBuilder<'a> for Builder<'a,>  {
    fn init_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, _size: u32) -> Builder<'a,> {
      ::capnp::traits::FromStructBuilder::new(builder.init_struct(_private::STRUCT_SIZE))
    }
    fn get_from_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, default: ::std::option::Option<&'a [::capnp::Word]>) -> ::capnp::Result<Builder<'a,>> {
      ::std::result::Result::Ok(::capnp::traits::FromStructBuilder::new(builder.get_struct(_private::STRUCT_SIZE, default)?))
    }
  }

  impl <'a,> ::capnp::traits::SetPointerBuilder<Builder<'a,>> for Reader<'a,>  {
    fn set_pointer_builder<'b>(pointer: ::capnp::private::layout::PointerBuilder<'b>, value: Reader<'a,>, canonicalize: bool) -> ::capnp::Result<()> { pointer.set_struct(&value.reader, canonicalize) }
  }

  impl <'a,> Builder<'a,>  {
    pub fn into_reader(self) -> Reader<'a,> {
      ::capnp::traits::FromStructReader::new(self.builder.into_reader())
    }
    pub fn reborrow(&mut self) -> Builder<> {
      Builder { .. *self }
    }
    pub fn reborrow_as_reader(&self) -> Reader<> {
      ::capnp::traits::FromStructReader::new(self.builder.into_reader())
    }

    pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
      self.builder.into_reader().total_size()
    }
    #[inline]
    pub fn set_msg_error<'b>(&mut self, value: crate::transport_dm_capnp::message::msg_error::Reader<'b>) -> ::capnp::Result<()> {
      self.builder.set_data_field::<u16>(0, 0);
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(0), value, false)
    }
    #[inline]
    pub fn init_msg_error(self, ) -> crate::transport_dm_capnp::message::msg_error::Builder<'a> {
      self.builder.set_data_field::<u16>(0, 0);
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(0), 0)
    }
    pub fn has_msg_error(&self) -> bool {
      if self.builder.get_data_field::<u16>(0) != 0 { return false; }
      !self.builder.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn set_msg_channel_create<'b>(&mut self, value: crate::transport_dm_capnp::message::msg_channel_create::Reader<'b>) -> ::capnp::Result<()> {
      self.builder.set_data_field::<u16>(0, 1);
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(0), value, false)
    }
    #[inline]
    pub fn init_msg_channel_create(self, ) -> crate::transport_dm_capnp::message::msg_channel_create::Builder<'a> {
      self.builder.set_data_field::<u16>(0, 1);
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(0), 0)
    }
    pub fn has_msg_channel_create(&self) -> bool {
      if self.builder.get_data_field::<u16>(0) != 1 { return false; }
      !self.builder.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn set_msg_channel_close(&mut self, value: u32)  {
      self.builder.set_data_field::<u16>(0, 2);
      self.builder.set_data_field::<u32>(1, value);
    }
    #[inline]
    pub fn set_msg_channel_message<'b>(&mut self, value: crate::transport_dm_capnp::message::msg_channel_message::Reader<'b>) -> ::capnp::Result<()> {
      self.builder.set_data_field::<u16>(0, 3);
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(0), value, false)
    }
    #[inline]
    pub fn init_msg_channel_message(self, ) -> crate::transport_dm_capnp::message::msg_channel_message::Builder<'a> {
      self.builder.set_data_field::<u16>(0, 3);
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(0), 0)
    }
    pub fn has_msg_channel_message(&self) -> bool {
      if self.builder.get_data_field::<u16>(0) != 3 { return false; }
      !self.builder.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn set_msg_relay_request(&mut self, _value: ())  {
      self.builder.set_data_field::<u16>(0, 4);
    }
    #[inline]
    pub fn set_msg_relay_accept(&mut self, _value: ())  {
      self.builder.set_data_field::<u16>(0, 5);
    }
    #[inline]
    pub fn set_msg_relay_channel_create<'b>(&mut self, value: crate::transport_dm_capnp::message::msg_channel_create::Reader<'b>) -> ::capnp::Result<()> {
      self.builder.set_data_field::<u16>(0, 6);
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(0), value, false)
    }
    #[inline]
    pub fn init_msg_relay_channel_create(self, ) -> crate::transport_dm_capnp::message::msg_channel_create::Builder<'a> {
      self.builder.set_data_field::<u16>(0, 6);
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(0), 0)
    }
    pub fn has_msg_relay_channel_create(&self) -> bool {
      if self.builder.get_data_field::<u16>(0) != 6 { return false; }
      !self.builder.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn set_msg_relay_channel_close(&mut self, value: u32)  {
      self.builder.set_data_field::<u16>(0, 7);
      self.builder.set_data_field::<u32>(1, value);
    }
    #[inline]
    pub fn set_msg_relay_channel_message<'b>(&mut self, value: crate::transport_dm_capnp::message::msg_channel_message::Reader<'b>) -> ::capnp::Result<()> {
      self.builder.set_data_field::<u16>(0, 8);
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(0), value, false)
    }
    #[inline]
    pub fn init_msg_relay_channel_message(self, ) -> crate::transport_dm_capnp::message::msg_channel_message::Builder<'a> {
      self.builder.set_data_field::<u16>(0, 8);
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(0), 0)
    }
    pub fn has_msg_relay_channel_message(&self) -> bool {
      if self.builder.get_data_field::<u16>(0) != 8 { return false; }
      !self.builder.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn which(self) -> ::std::result::Result<WhichBuilder<'a,>, ::capnp::NotInSchema> {
      match self.builder.get_data_field::<u16>(0) {
        0 => {
          ::std::result::Result::Ok(MsgError(
            ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(0), ::std::option::Option::None)
          ))
        }
        1 => {
          ::std::result::Result::Ok(MsgChannelCreate(
            ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(0), ::std::option::Option::None)
          ))
        }
        2 => {
          ::std::result::Result::Ok(MsgChannelClose(
            self.builder.get_data_field::<u32>(1)
          ))
        }
        3 => {
          ::std::result::Result::Ok(MsgChannelMessage(
            ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(0), ::std::option::Option::None)
          ))
        }
        4 => {
          ::std::result::Result::Ok(MsgRelayRequest(
            ()
          ))
        }
        5 => {
          ::std::result::Result::Ok(MsgRelayAccept(
            ()
          ))
        }
        6 => {
          ::std::result::Result::Ok(MsgRelayChannelCreate(
            ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(0), ::std::option::Option::None)
          ))
        }
        7 => {
          ::std::result::Result::Ok(MsgRelayChannelClose(
            self.builder.get_data_field::<u32>(1)
          ))
        }
        8 => {
          ::std::result::Result::Ok(MsgRelayChannelMessage(
            ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(0), ::std::option::Option::None)
          ))
        }
        x => ::std::result::Result::Err(::capnp::NotInSchema(x))
      }
    }
  }

  pub struct Pipeline { _typeless: ::capnp::any_pointer::Pipeline }
  impl ::capnp::capability::FromTypelessPipeline for Pipeline {
    fn new(typeless: ::capnp::any_pointer::Pipeline) -> Pipeline {
      Pipeline { _typeless: typeless,  }
    }
  }
  impl Pipeline  {
  }
  mod _private {
    use capnp::private::layout;
    pub const STRUCT_SIZE: layout::StructSize = layout::StructSize { data: 1, pointers: 1 };
    pub const TYPE_ID: u64 = 0xc615_f70a_18ed_a8fe;
  }
  pub enum Which<A0,A1,A2,A3,A4> {
    MsgError(A0),
    MsgChannelCreate(A1),
    MsgChannelClose(u32),
    MsgChannelMessage(A2),
    MsgRelayRequest(()),
    MsgRelayAccept(()),
    MsgRelayChannelCreate(A3),
    MsgRelayChannelClose(u32),
    MsgRelayChannelMessage(A4),
  }
  pub type WhichReader<'a,> = Which<::capnp::Result<crate::transport_dm_capnp::message::msg_error::Reader<'a>>,::capnp::Result<crate::transport_dm_capnp::message::msg_channel_create::Reader<'a>>,::capnp::Result<crate::transport_dm_capnp::message::msg_channel_message::Reader<'a>>,::capnp::Result<crate::transport_dm_capnp::message::msg_channel_create::Reader<'a>>,::capnp::Result<crate::transport_dm_capnp::message::msg_channel_message::Reader<'a>>>;
  pub type WhichBuilder<'a,> = Which<::capnp::Result<crate::transport_dm_capnp::message::msg_error::Builder<'a>>,::capnp::Result<crate::transport_dm_capnp::message::msg_channel_create::Builder<'a>>,::capnp::Result<crate::transport_dm_capnp::message::msg_channel_message::Builder<'a>>,::capnp::Result<crate::transport_dm_capnp::message::msg_channel_create::Builder<'a>>,::capnp::Result<crate::transport_dm_capnp::message::msg_channel_message::Builder<'a>>>;

  pub mod msg_error {
    #[derive(Copy, Clone)]
    pub struct Owned;
    impl <'a> ::capnp::traits::Owned<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
    impl <'a> ::capnp::traits::OwnedStruct<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
    impl ::capnp::traits::Pipelined for Owned { type Pipeline = Pipeline; }

    #[derive(Clone, Copy)]
    pub struct Reader<'a> { reader: ::capnp::private::layout::StructReader<'a> }

    impl <'a,> ::capnp::traits::HasTypeId for Reader<'a,>  {
      #[inline]
      fn type_id() -> u64 { _private::TYPE_ID }
    }
    impl <'a,> ::capnp::traits::FromStructReader<'a> for Reader<'a,>  {
      fn new(reader: ::capnp::private::layout::StructReader<'a>) -> Reader<'a,> {
        Reader { reader: reader,  }
      }
    }

    impl <'a,> ::capnp::traits::FromPointerReader<'a> for Reader<'a,>  {
      fn get_from_pointer(reader: &::capnp::private::layout::PointerReader<'a>, default: ::std::option::Option<&'a [::capnp::Word]>) -> ::capnp::Result<Reader<'a,>> {
        ::std::result::Result::Ok(::capnp::traits::FromStructReader::new(reader.get_struct(default)?))
      }
    }

    impl <'a,> ::capnp::traits::IntoInternalStructReader<'a> for Reader<'a,>  {
      fn into_internal_struct_reader(self) -> ::capnp::private::layout::StructReader<'a> {
        self.reader
      }
    }

    impl <'a,> ::capnp::traits::Imbue<'a> for Reader<'a,>  {
      fn imbue(&mut self, cap_table: &'a ::capnp::private::layout::CapTable) {
        self.reader.imbue(::capnp::private::layout::CapTableReader::Plain(cap_table))
      }
    }

    impl <'a,> Reader<'a,>  {
      pub fn reborrow(&self) -> Reader<> {
        Reader { .. *self }
      }

      pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
        self.reader.total_size()
      }
      #[inline]
      pub fn get_channel_id(self) -> u32 {
        self.reader.get_data_field::<u32>(0)
      }
      #[inline]
      pub fn get_error_code(self) -> ::std::result::Result<crate::transport_dm_capnp::message::msg_error::ErrorCode,::capnp::NotInSchema> {
        ::capnp::traits::FromU16::from_u16(self.reader.get_data_field::<u16>(2))
      }
      #[inline]
      pub fn get_error_text(self) -> ::capnp::Result<::capnp::text::Reader<'a>> {
        ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(0), ::std::option::Option::None)
      }
      pub fn has_error_text(&self) -> bool {
        !self.reader.get_pointer_field(0).is_null()
      }
    }

    pub struct Builder<'a> { builder: ::capnp::private::layout::StructBuilder<'a> }
    impl <'a,> ::capnp::traits::HasStructSize for Builder<'a,>  {
      #[inline]
      fn struct_size() -> ::capnp::private::layout::StructSize { _private::STRUCT_SIZE }
    }
    impl <'a,> ::capnp::traits::HasTypeId for Builder<'a,>  {
      #[inline]
      fn type_id() -> u64 { _private::TYPE_ID }
    }
    impl <'a,> ::capnp::traits::FromStructBuilder<'a> for Builder<'a,>  {
      fn new(builder: ::capnp::private::layout::StructBuilder<'a>) -> Builder<'a, > {
        Builder { builder: builder,  }
      }
    }

    impl <'a,> ::capnp::traits::ImbueMut<'a> for Builder<'a,>  {
      fn imbue_mut(&mut self, cap_table: &'a mut ::capnp::private::layout::CapTable) {
        self.builder.imbue(::capnp::private::layout::CapTableBuilder::Plain(cap_table))
      }
    }

    impl <'a,> ::capnp::traits::FromPointerBuilder<'a> for Builder<'a,>  {
      fn init_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, _size: u32) -> Builder<'a,> {
        ::capnp::traits::FromStructBuilder::new(builder.init_struct(_private::STRUCT_SIZE))
      }
      fn get_from_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, default: ::std::option::Option<&'a [::capnp::Word]>) -> ::capnp::Result<Builder<'a,>> {
        ::std::result::Result::Ok(::capnp::traits::FromStructBuilder::new(builder.get_struct(_private::STRUCT_SIZE, default)?))
      }
    }

    impl <'a,> ::capnp::traits::SetPointerBuilder<Builder<'a,>> for Reader<'a,>  {
      fn set_pointer_builder<'b>(pointer: ::capnp::private::layout::PointerBuilder<'b>, value: Reader<'a,>, canonicalize: bool) -> ::capnp::Result<()> { pointer.set_struct(&value.reader, canonicalize) }
    }

    impl <'a,> Builder<'a,>  {
      pub fn into_reader(self) -> Reader<'a,> {
        ::capnp::traits::FromStructReader::new(self.builder.into_reader())
      }
      pub fn reborrow(&mut self) -> Builder<> {
        Builder { .. *self }
      }
      pub fn reborrow_as_reader(&self) -> Reader<> {
        ::capnp::traits::FromStructReader::new(self.builder.into_reader())
      }

      pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
        self.builder.into_reader().total_size()
      }
      #[inline]
      pub fn get_channel_id(self) -> u32 {
        self.builder.get_data_field::<u32>(0)
      }
      #[inline]
      pub fn set_channel_id(&mut self, value: u32)  {
        self.builder.set_data_field::<u32>(0, value);
      }
      #[inline]
      pub fn get_error_code(self) -> ::std::result::Result<crate::transport_dm_capnp::message::msg_error::ErrorCode,::capnp::NotInSchema> {
        ::capnp::traits::FromU16::from_u16(self.builder.get_data_field::<u16>(2))
      }
      #[inline]
      pub fn set_error_code(&mut self, value: crate::transport_dm_capnp::message::msg_error::ErrorCode)  {
        self.builder.set_data_field::<u16>(2, value as u16)
      }
      #[inline]
      pub fn get_error_text(self) -> ::capnp::Result<::capnp::text::Builder<'a>> {
        ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(0), ::std::option::Option::None)
      }
      #[inline]
      pub fn set_error_text(&mut self, value: ::capnp::text::Reader)  {
        self.builder.get_pointer_field(0).set_text(value);
      }
      #[inline]
      pub fn init_error_text(self, size: u32) -> ::capnp::text::Builder<'a> {
        self.builder.get_pointer_field(0).init_text(size)
      }
      pub fn has_error_text(&self) -> bool {
        !self.builder.get_pointer_field(0).is_null()
      }
    }

    pub struct Pipeline { _typeless: ::capnp::any_pointer::Pipeline }
    impl ::capnp::capability::FromTypelessPipeline for Pipeline {
      fn new(typeless: ::capnp::any_pointer::Pipeline) -> Pipeline {
        Pipeline { _typeless: typeless,  }
      }
    }
    impl Pipeline  {
    }
    mod _private {
      use capnp::private::layout;
      pub const STRUCT_SIZE: layout::StructSize = layout::StructSize { data: 1, pointers: 1 };
      pub const TYPE_ID: u64 = 0xc407_02eb_ad56_8959;
    }

    #[repr(u16)]
    #[derive(Clone, Copy, PartialEq)]
    pub enum ErrorCode {
      Unknown = 0,
      BadChannelId = 1,
      BadSpaceHash = 2,
      BadToId = 3,
      BadFromId = 4,
    }
    impl ::capnp::traits::FromU16 for ErrorCode {
      #[inline]
      fn from_u16(value: u16) -> ::std::result::Result<ErrorCode, ::capnp::NotInSchema> {
        match value {
          0 => ::std::result::Result::Ok(ErrorCode::Unknown),
          1 => ::std::result::Result::Ok(ErrorCode::BadChannelId),
          2 => ::std::result::Result::Ok(ErrorCode::BadSpaceHash),
          3 => ::std::result::Result::Ok(ErrorCode::BadToId),
          4 => ::std::result::Result::Ok(ErrorCode::BadFromId),
          n => ::std::result::Result::Err(::capnp::NotInSchema(n)),
        }
      }
    }
    impl ::capnp::traits::ToU16 for ErrorCode {
      #[inline]
      fn to_u16(self) -> u16 { self as u16 }
    }
    impl ::capnp::traits::HasTypeId for ErrorCode {
      #[inline]
      fn type_id() -> u64 { 0xda15_75b0_a8e1_488fu64 }
    }
  }

  pub mod msg_channel_create {
    #[derive(Copy, Clone)]
    pub struct Owned;
    impl <'a> ::capnp::traits::Owned<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
    impl <'a> ::capnp::traits::OwnedStruct<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
    impl ::capnp::traits::Pipelined for Owned { type Pipeline = Pipeline; }

    #[derive(Clone, Copy)]
    pub struct Reader<'a> { reader: ::capnp::private::layout::StructReader<'a> }

    impl <'a,> ::capnp::traits::HasTypeId for Reader<'a,>  {
      #[inline]
      fn type_id() -> u64 { _private::TYPE_ID }
    }
    impl <'a,> ::capnp::traits::FromStructReader<'a> for Reader<'a,>  {
      fn new(reader: ::capnp::private::layout::StructReader<'a>) -> Reader<'a,> {
        Reader { reader: reader,  }
      }
    }

    impl <'a,> ::capnp::traits::FromPointerReader<'a> for Reader<'a,>  {
      fn get_from_pointer(reader: &::capnp::private::layout::PointerReader<'a>, default: ::std::option::Option<&'a [::capnp::Word]>) -> ::capnp::Result<Reader<'a,>> {
        ::std::result::Result::Ok(::capnp::traits::FromStructReader::new(reader.get_struct(default)?))
      }
    }

    impl <'a,> ::capnp::traits::IntoInternalStructReader<'a> for Reader<'a,>  {
      fn into_internal_struct_reader(self) -> ::capnp::private::layout::StructReader<'a> {
        self.reader
      }
    }

    impl <'a,> ::capnp::traits::Imbue<'a> for Reader<'a,>  {
      fn imbue(&mut self, cap_table: &'a ::capnp::private::layout::CapTable) {
        self.reader.imbue(::capnp::private::layout::CapTableReader::Plain(cap_table))
      }
    }

    impl <'a,> Reader<'a,>  {
      pub fn reborrow(&self) -> Reader<> {
        Reader { .. *self }
      }

      pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
        self.reader.total_size()
      }
      #[inline]
      pub fn get_channel_id(self) -> u32 {
        self.reader.get_data_field::<u32>(0)
      }
      #[inline]
      pub fn get_space_hash(self) -> ::capnp::Result<::capnp::data::Reader<'a>> {
        ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(0), ::std::option::Option::None)
      }
      pub fn has_space_hash(&self) -> bool {
        !self.reader.get_pointer_field(0).is_null()
      }
      #[inline]
      pub fn get_to_id(self) -> ::capnp::Result<::capnp::data::Reader<'a>> {
        ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(1), ::std::option::Option::None)
      }
      pub fn has_to_id(&self) -> bool {
        !self.reader.get_pointer_field(1).is_null()
      }
      #[inline]
      pub fn get_from_id(self) -> ::capnp::Result<::capnp::data::Reader<'a>> {
        ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(2), ::std::option::Option::None)
      }
      pub fn has_from_id(&self) -> bool {
        !self.reader.get_pointer_field(2).is_null()
      }
    }

    pub struct Builder<'a> { builder: ::capnp::private::layout::StructBuilder<'a> }
    impl <'a,> ::capnp::traits::HasStructSize for Builder<'a,>  {
      #[inline]
      fn struct_size() -> ::capnp::private::layout::StructSize { _private::STRUCT_SIZE }
    }
    impl <'a,> ::capnp::traits::HasTypeId for Builder<'a,>  {
      #[inline]
      fn type_id() -> u64 { _private::TYPE_ID }
    }
    impl <'a,> ::capnp::traits::FromStructBuilder<'a> for Builder<'a,>  {
      fn new(builder: ::capnp::private::layout::StructBuilder<'a>) -> Builder<'a, > {
        Builder { builder: builder,  }
      }
    }

    impl <'a,> ::capnp::traits::ImbueMut<'a> for Builder<'a,>  {
      fn imbue_mut(&mut self, cap_table: &'a mut ::capnp::private::layout::CapTable) {
        self.builder.imbue(::capnp::private::layout::CapTableBuilder::Plain(cap_table))
      }
    }

    impl <'a,> ::capnp::traits::FromPointerBuilder<'a> for Builder<'a,>  {
      fn init_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, _size: u32) -> Builder<'a,> {
        ::capnp::traits::FromStructBuilder::new(builder.init_struct(_private::STRUCT_SIZE))
      }
      fn get_from_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, default: ::std::option::Option<&'a [::capnp::Word]>) -> ::capnp::Result<Builder<'a,>> {
        ::std::result::Result::Ok(::capnp::traits::FromStructBuilder::new(builder.get_struct(_private::STRUCT_SIZE, default)?))
      }
    }

    impl <'a,> ::capnp::traits::SetPointerBuilder<Builder<'a,>> for Reader<'a,>  {
      fn set_pointer_builder<'b>(pointer: ::capnp::private::layout::PointerBuilder<'b>, value: Reader<'a,>, canonicalize: bool) -> ::capnp::Result<()> { pointer.set_struct(&value.reader, canonicalize) }
    }

    impl <'a,> Builder<'a,>  {
      pub fn into_reader(self) -> Reader<'a,> {
        ::capnp::traits::FromStructReader::new(self.builder.into_reader())
      }
      pub fn reborrow(&mut self) -> Builder<> {
        Builder { .. *self }
      }
      pub fn reborrow_as_reader(&self) -> Reader<> {
        ::capnp::traits::FromStructReader::new(self.builder.into_reader())
      }

      pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
        self.builder.into_reader().total_size()
      }
      #[inline]
      pub fn get_channel_id(self) -> u32 {
        self.builder.get_data_field::<u32>(0)
      }
      #[inline]
      pub fn set_channel_id(&mut self, value: u32)  {
        self.builder.set_data_field::<u32>(0, value);
      }
      #[inline]
      pub fn get_space_hash(self) -> ::capnp::Result<::capnp::data::Builder<'a>> {
        ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(0), ::std::option::Option::None)
      }
      #[inline]
      pub fn set_space_hash(&mut self, value: ::capnp::data::Reader)  {
        self.builder.get_pointer_field(0).set_data(value);
      }
      #[inline]
      pub fn init_space_hash(self, size: u32) -> ::capnp::data::Builder<'a> {
        self.builder.get_pointer_field(0).init_data(size)
      }
      pub fn has_space_hash(&self) -> bool {
        !self.builder.get_pointer_field(0).is_null()
      }
      #[inline]
      pub fn get_to_id(self) -> ::capnp::Result<::capnp::data::Builder<'a>> {
        ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(1), ::std::option::Option::None)
      }
      #[inline]
      pub fn set_to_id(&mut self, value: ::capnp::data::Reader)  {
        self.builder.get_pointer_field(1).set_data(value);
      }
      #[inline]
      pub fn init_to_id(self, size: u32) -> ::capnp::data::Builder<'a> {
        self.builder.get_pointer_field(1).init_data(size)
      }
      pub fn has_to_id(&self) -> bool {
        !self.builder.get_pointer_field(1).is_null()
      }
      #[inline]
      pub fn get_from_id(self) -> ::capnp::Result<::capnp::data::Builder<'a>> {
        ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(2), ::std::option::Option::None)
      }
      #[inline]
      pub fn set_from_id(&mut self, value: ::capnp::data::Reader)  {
        self.builder.get_pointer_field(2).set_data(value);
      }
      #[inline]
      pub fn init_from_id(self, size: u32) -> ::capnp::data::Builder<'a> {
        self.builder.get_pointer_field(2).init_data(size)
      }
      pub fn has_from_id(&self) -> bool {
        !self.builder.get_pointer_field(2).is_null()
      }
    }

    pub struct Pipeline { _typeless: ::capnp::any_pointer::Pipeline }
    impl ::capnp::capability::FromTypelessPipeline for Pipeline {
      fn new(typeless: ::capnp::any_pointer::Pipeline) -> Pipeline {
        Pipeline { _typeless: typeless,  }
      }
    }
    impl Pipeline  {
    }
    mod _private {
      use capnp::private::layout;
      pub const STRUCT_SIZE: layout::StructSize = layout::StructSize { data: 1, pointers: 3 };
      pub const TYPE_ID: u64 = 0xa5bb_8832_6fab_b257;
    }
  }

  pub mod msg_channel_message {
    #[derive(Copy, Clone)]
    pub struct Owned;
    impl <'a> ::capnp::traits::Owned<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
    impl <'a> ::capnp::traits::OwnedStruct<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
    impl ::capnp::traits::Pipelined for Owned { type Pipeline = Pipeline; }

    #[derive(Clone, Copy)]
    pub struct Reader<'a> { reader: ::capnp::private::layout::StructReader<'a> }

    impl <'a,> ::capnp::traits::HasTypeId for Reader<'a,>  {
      #[inline]
      fn type_id() -> u64 { _private::TYPE_ID }
    }
    impl <'a,> ::capnp::traits::FromStructReader<'a> for Reader<'a,>  {
      fn new(reader: ::capnp::private::layout::StructReader<'a>) -> Reader<'a,> {
        Reader { reader: reader,  }
      }
    }

    impl <'a,> ::capnp::traits::FromPointerReader<'a> for Reader<'a,>  {
      fn get_from_pointer(reader: &::capnp::private::layout::PointerReader<'a>, default: ::std::option::Option<&'a [::capnp::Word]>) -> ::capnp::Result<Reader<'a,>> {
        ::std::result::Result::Ok(::capnp::traits::FromStructReader::new(reader.get_struct(default)?))
      }
    }

    impl <'a,> ::capnp::traits::IntoInternalStructReader<'a> for Reader<'a,>  {
      fn into_internal_struct_reader(self) -> ::capnp::private::layout::StructReader<'a> {
        self.reader
      }
    }

    impl <'a,> ::capnp::traits::Imbue<'a> for Reader<'a,>  {
      fn imbue(&mut self, cap_table: &'a ::capnp::private::layout::CapTable) {
        self.reader.imbue(::capnp::private::layout::CapTableReader::Plain(cap_table))
      }
    }

    impl <'a,> Reader<'a,>  {
      pub fn reborrow(&self) -> Reader<> {
        Reader { .. *self }
      }

      pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
        self.reader.total_size()
      }
      #[inline]
      pub fn get_channel_id(self) -> u32 {
        self.reader.get_data_field::<u32>(0)
      }
      #[inline]
      pub fn get_content(self) -> ::capnp::Result<::capnp::data::Reader<'a>> {
        ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(0), ::std::option::Option::None)
      }
      pub fn has_content(&self) -> bool {
        !self.reader.get_pointer_field(0).is_null()
      }
    }

    pub struct Builder<'a> { builder: ::capnp::private::layout::StructBuilder<'a> }
    impl <'a,> ::capnp::traits::HasStructSize for Builder<'a,>  {
      #[inline]
      fn struct_size() -> ::capnp::private::layout::StructSize { _private::STRUCT_SIZE }
    }
    impl <'a,> ::capnp::traits::HasTypeId for Builder<'a,>  {
      #[inline]
      fn type_id() -> u64 { _private::TYPE_ID }
    }
    impl <'a,> ::capnp::traits::FromStructBuilder<'a> for Builder<'a,>  {
      fn new(builder: ::capnp::private::layout::StructBuilder<'a>) -> Builder<'a, > {
        Builder { builder: builder,  }
      }
    }

    impl <'a,> ::capnp::traits::ImbueMut<'a> for Builder<'a,>  {
      fn imbue_mut(&mut self, cap_table: &'a mut ::capnp::private::layout::CapTable) {
        self.builder.imbue(::capnp::private::layout::CapTableBuilder::Plain(cap_table))
      }
    }

    impl <'a,> ::capnp::traits::FromPointerBuilder<'a> for Builder<'a,>  {
      fn init_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, _size: u32) -> Builder<'a,> {
        ::capnp::traits::FromStructBuilder::new(builder.init_struct(_private::STRUCT_SIZE))
      }
      fn get_from_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, default: ::std::option::Option<&'a [::capnp::Word]>) -> ::capnp::Result<Builder<'a,>> {
        ::std::result::Result::Ok(::capnp::traits::FromStructBuilder::new(builder.get_struct(_private::STRUCT_SIZE, default)?))
      }
    }

    impl <'a,> ::capnp::traits::SetPointerBuilder<Builder<'a,>> for Reader<'a,>  {
      fn set_pointer_builder<'b>(pointer: ::capnp::private::layout::PointerBuilder<'b>, value: Reader<'a,>, canonicalize: bool) -> ::capnp::Result<()> { pointer.set_struct(&value.reader, canonicalize) }
    }

    impl <'a,> Builder<'a,>  {
      pub fn into_reader(self) -> Reader<'a,> {
        ::capnp::traits::FromStructReader::new(self.builder.into_reader())
      }
      pub fn reborrow(&mut self) -> Builder<> {
        Builder { .. *self }
      }
      pub fn reborrow_as_reader(&self) -> Reader<> {
        ::capnp::traits::FromStructReader::new(self.builder.into_reader())
      }

      pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
        self.builder.into_reader().total_size()
      }
      #[inline]
      pub fn get_channel_id(self) -> u32 {
        self.builder.get_data_field::<u32>(0)
      }
      #[inline]
      pub fn set_channel_id(&mut self, value: u32)  {
        self.builder.set_data_field::<u32>(0, value);
      }
      #[inline]
      pub fn get_content(self) -> ::capnp::Result<::capnp::data::Builder<'a>> {
        ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(0), ::std::option::Option::None)
      }
      #[inline]
      pub fn set_content(&mut self, value: ::capnp::data::Reader)  {
        self.builder.get_pointer_field(0).set_data(value);
      }
      #[inline]
      pub fn init_content(self, size: u32) -> ::capnp::data::Builder<'a> {
        self.builder.get_pointer_field(0).init_data(size)
      }
      pub fn has_content(&self) -> bool {
        !self.builder.get_pointer_field(0).is_null()
      }
    }

    pub struct Pipeline { _typeless: ::capnp::any_pointer::Pipeline }
    impl ::capnp::capability::FromTypelessPipeline for Pipeline {
      fn new(typeless: ::capnp::any_pointer::Pipeline) -> Pipeline {
        Pipeline { _typeless: typeless,  }
      }
    }
    impl Pipeline  {
    }
    mod _private {
      use capnp::private::layout;
      pub const STRUCT_SIZE: layout::StructSize = layout::StructSize { data: 1, pointers: 1 };
      pub const TYPE_ID: u64 = 0xf7ed_e896_a2a8_d3ea;
    }
  }
}
