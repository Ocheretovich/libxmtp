// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `message_contents/composite.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

///  Composite is used to implement xmtp.org/composite content type
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:xmtp.message_contents.Composite)
pub struct Composite {
    // message fields
    // @@protoc_insertion_point(field:xmtp.message_contents.Composite.parts)
    pub parts: ::std::vec::Vec<composite::Part>,
    // special fields
    // @@protoc_insertion_point(special_field:xmtp.message_contents.Composite.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Composite {
    fn default() -> &'a Composite {
        <Composite as ::protobuf::Message>::default_instance()
    }
}

impl Composite {
    pub fn new() -> Composite {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "parts",
            |m: &Composite| { &m.parts },
            |m: &mut Composite| { &mut m.parts },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Composite>(
            "Composite",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Composite {
    const NAME: &'static str = "Composite";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.parts.push(is.read_message()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        for value in &self.parts {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.parts {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Composite {
        Composite::new()
    }

    fn clear(&mut self) {
        self.parts.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Composite {
        static instance: Composite = Composite {
            parts: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Composite {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Composite").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Composite {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Composite {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `Composite`
pub mod composite {
    ///  Part represents one section of a composite message
    #[derive(PartialEq,Clone,Default,Debug)]
    // @@protoc_insertion_point(message:xmtp.message_contents.Composite.Part)
    pub struct Part {
        // message oneof groups
        pub element: ::std::option::Option<part::Element>,
        // special fields
        // @@protoc_insertion_point(special_field:xmtp.message_contents.Composite.Part.special_fields)
        pub special_fields: ::protobuf::SpecialFields,
    }

    impl<'a> ::std::default::Default for &'a Part {
        fn default() -> &'a Part {
            <Part as ::protobuf::Message>::default_instance()
        }
    }

    impl Part {
        pub fn new() -> Part {
            ::std::default::Default::default()
        }

        // .xmtp.message_contents.EncodedContent part = 1;

        pub fn part(&self) -> &super::super::content::EncodedContent {
            match self.element {
                ::std::option::Option::Some(part::Element::Part(ref v)) => v,
                _ => <super::super::content::EncodedContent as ::protobuf::Message>::default_instance(),
            }
        }

        pub fn clear_part(&mut self) {
            self.element = ::std::option::Option::None;
        }

        pub fn has_part(&self) -> bool {
            match self.element {
                ::std::option::Option::Some(part::Element::Part(..)) => true,
                _ => false,
            }
        }

        // Param is passed by value, moved
        pub fn set_part(&mut self, v: super::super::content::EncodedContent) {
            self.element = ::std::option::Option::Some(part::Element::Part(v))
        }

        // Mutable pointer to the field.
        pub fn mut_part(&mut self) -> &mut super::super::content::EncodedContent {
            if let ::std::option::Option::Some(part::Element::Part(_)) = self.element {
            } else {
                self.element = ::std::option::Option::Some(part::Element::Part(super::super::content::EncodedContent::new()));
            }
            match self.element {
                ::std::option::Option::Some(part::Element::Part(ref mut v)) => v,
                _ => panic!(),
            }
        }

        // Take field
        pub fn take_part(&mut self) -> super::super::content::EncodedContent {
            if self.has_part() {
                match self.element.take() {
                    ::std::option::Option::Some(part::Element::Part(v)) => v,
                    _ => panic!(),
                }
            } else {
                super::super::content::EncodedContent::new()
            }
        }

        // .xmtp.message_contents.Composite composite = 2;

        pub fn composite(&self) -> &super::Composite {
            match self.element {
                ::std::option::Option::Some(part::Element::Composite(ref v)) => v,
                _ => <super::Composite as ::protobuf::Message>::default_instance(),
            }
        }

        pub fn clear_composite(&mut self) {
            self.element = ::std::option::Option::None;
        }

        pub fn has_composite(&self) -> bool {
            match self.element {
                ::std::option::Option::Some(part::Element::Composite(..)) => true,
                _ => false,
            }
        }

        // Param is passed by value, moved
        pub fn set_composite(&mut self, v: super::Composite) {
            self.element = ::std::option::Option::Some(part::Element::Composite(v))
        }

        // Mutable pointer to the field.
        pub fn mut_composite(&mut self) -> &mut super::Composite {
            if let ::std::option::Option::Some(part::Element::Composite(_)) = self.element {
            } else {
                self.element = ::std::option::Option::Some(part::Element::Composite(super::Composite::new()));
            }
            match self.element {
                ::std::option::Option::Some(part::Element::Composite(ref mut v)) => v,
                _ => panic!(),
            }
        }

        // Take field
        pub fn take_composite(&mut self) -> super::Composite {
            if self.has_composite() {
                match self.element.take() {
                    ::std::option::Option::Some(part::Element::Composite(v)) => v,
                    _ => panic!(),
                }
            } else {
                super::Composite::new()
            }
        }

        pub(in super) fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
            let mut fields = ::std::vec::Vec::with_capacity(2);
            let mut oneofs = ::std::vec::Vec::with_capacity(1);
            fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::super::content::EncodedContent>(
                "part",
                Part::has_part,
                Part::part,
                Part::mut_part,
                Part::set_part,
            ));
            fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::Composite>(
                "composite",
                Part::has_composite,
                Part::composite,
                Part::mut_composite,
                Part::set_composite,
            ));
            oneofs.push(part::Element::generated_oneof_descriptor_data());
            ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Part>(
                "Composite.Part",
                fields,
                oneofs,
            )
        }
    }

    impl ::protobuf::Message for Part {
        const NAME: &'static str = "Part";

        fn is_initialized(&self) -> bool {
            true
        }

        fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
            while let Some(tag) = is.read_raw_tag_or_eof()? {
                match tag {
                    10 => {
                        self.element = ::std::option::Option::Some(part::Element::Part(is.read_message()?));
                    },
                    18 => {
                        self.element = ::std::option::Option::Some(part::Element::Composite(is.read_message()?));
                    },
                    tag => {
                        ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                    },
                };
            }
            ::std::result::Result::Ok(())
        }

        // Compute sizes of nested messages
        #[allow(unused_variables)]
        fn compute_size(&self) -> u64 {
            let mut my_size = 0;
            if let ::std::option::Option::Some(ref v) = self.element {
                match v {
                    &part::Element::Part(ref v) => {
                        let len = v.compute_size();
                        my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                    },
                    &part::Element::Composite(ref v) => {
                        let len = v.compute_size();
                        my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                    },
                };
            }
            my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
            self.special_fields.cached_size().set(my_size as u32);
            my_size
        }

        fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
            if let ::std::option::Option::Some(ref v) = self.element {
                match v {
                    &part::Element::Part(ref v) => {
                        ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
                    },
                    &part::Element::Composite(ref v) => {
                        ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
                    },
                };
            }
            os.write_unknown_fields(self.special_fields.unknown_fields())?;
            ::std::result::Result::Ok(())
        }

        fn special_fields(&self) -> &::protobuf::SpecialFields {
            &self.special_fields
        }

        fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
            &mut self.special_fields
        }

        fn new() -> Part {
            Part::new()
        }

        fn clear(&mut self) {
            self.element = ::std::option::Option::None;
            self.element = ::std::option::Option::None;
            self.special_fields.clear();
        }

        fn default_instance() -> &'static Part {
            static instance: Part = Part {
                element: ::std::option::Option::None,
                special_fields: ::protobuf::SpecialFields::new(),
            };
            &instance
        }
    }

    impl ::protobuf::MessageFull for Part {
        fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().message_by_package_relative_name("Composite.Part").unwrap()).clone()
        }
    }

    impl ::std::fmt::Display for Part {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            ::protobuf::text_format::fmt(self, f)
        }
    }

    impl ::protobuf::reflect::ProtobufValue for Part {
        type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
    }

    /// Nested message and enums of message `Part`
    pub mod part {

        #[derive(Clone,PartialEq,Debug)]
        #[non_exhaustive]
        // @@protoc_insertion_point(oneof:xmtp.message_contents.Composite.Part.element)
        pub enum Element {
            // @@protoc_insertion_point(oneof_field:xmtp.message_contents.Composite.Part.part)
            Part(super::super::super::content::EncodedContent),
            // @@protoc_insertion_point(oneof_field:xmtp.message_contents.Composite.Part.composite)
            Composite(super::super::Composite),
        }

        impl ::protobuf::Oneof for Element {
        }

        impl ::protobuf::OneofFull for Element {
            fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
                static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
                descriptor.get(|| <super::Part as ::protobuf::MessageFull>::descriptor().oneof_by_name("element").unwrap()).clone()
            }
        }

        impl Element {
            pub(in super::super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
                ::protobuf::reflect::GeneratedOneofDescriptorData::new::<Element>("element")
            }
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20message_contents/composite.proto\x12\x15xmtp.message_contents\x1a\
    \x1emessage_contents/content.proto\"\xdb\x01\n\tComposite\x12;\n\x05part\
    s\x18\x01\x20\x03(\x0b2%.xmtp.message_contents.Composite.PartR\x05parts\
    \x1a\x90\x01\n\x04Part\x12;\n\x04part\x18\x01\x20\x01(\x0b2%.xmtp.messag\
    e_contents.EncodedContentH\0R\x04part\x12@\n\tcomposite\x18\x02\x20\x01(\
    \x0b2\x20.xmtp.message_contents.CompositeH\0R\tcompositeB\t\n\x07element\
    BO\n\x1forg.xmtp.proto.message.contentsZ,github.com/xmtp/proto/v3/go/mes\
    sage_contentsJ\xf9\x03\n\x06\x12\x04\x01\0\x15\x01\n!\n\x01\x0c\x12\x03\
    \x01\0\x12\x1a\x17\x20Composite\x20ContentType\n\n\x08\n\x01\x02\x12\x03\
    \x03\0\x1e\n\t\n\x02\x03\0\x12\x03\x05\0(\n\x08\n\x01\x08\x12\x03\x07\0C\
    \n\t\n\x02\x08\x0b\x12\x03\x07\0C\n\x08\n\x01\x08\x12\x03\x08\08\n\t\n\
    \x02\x08\x01\x12\x03\x08\08\nL\n\x02\x04\0\x12\x04\x0b\0\x15\x01\x1a@\
    \x20Composite\x20is\x20used\x20to\x20implement\x20xmtp.org/composite\x20\
    content\x20type\n\n\n\n\x03\x04\0\x01\x12\x03\x0b\x08\x11\nB\n\x04\x04\0\
    \x03\0\x12\x04\r\x04\x12\x05\x1a4\x20Part\x20represents\x20one\x20sectio\
    n\x20of\x20a\x20composite\x20message\n\n\x0c\n\x05\x04\0\x03\0\x01\x12\
    \x03\r\x0c\x10\n\x0e\n\x06\x04\0\x03\0\x08\0\x12\x04\x0e\x08\x11\t\n\x0e\
    \n\x07\x04\0\x03\0\x08\0\x01\x12\x03\x0e\x0e\x15\n\r\n\x06\x04\0\x03\0\
    \x02\0\x12\x03\x0f\x0c$\n\x0e\n\x07\x04\0\x03\0\x02\0\x06\x12\x03\x0f\
    \x0c\x1a\n\x0e\n\x07\x04\0\x03\0\x02\0\x01\x12\x03\x0f\x1b\x1f\n\x0e\n\
    \x07\x04\0\x03\0\x02\0\x03\x12\x03\x0f\"#\n\r\n\x06\x04\0\x03\0\x02\x01\
    \x12\x03\x10\x0c$\n\x0e\n\x07\x04\0\x03\0\x02\x01\x06\x12\x03\x10\x0c\
    \x15\n\x0e\n\x07\x04\0\x03\0\x02\x01\x01\x12\x03\x10\x16\x1f\n\x0e\n\x07\
    \x04\0\x03\0\x02\x01\x03\x12\x03\x10\"#\n\x0b\n\x04\x04\0\x02\0\x12\x03\
    \x14\x04\x1c\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x14\x04\x0c\n\x0c\n\x05\
    \x04\0\x02\0\x06\x12\x03\x14\r\x11\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\
    \x14\x12\x17\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x14\x1a\x1bb\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::content::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(Composite::generated_message_descriptor_data());
            messages.push(composite::Part::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
