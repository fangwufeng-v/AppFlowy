// This file is generated by rust-protobuf 2.25.2. Do not edit
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
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `dart_notification.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_2;

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum FolderNotification {
    Unknown = 0,
    UserCreateWorkspace = 10,
    UserDeleteWorkspace = 11,
    WorkspaceUpdated = 12,
    WorkspaceListUpdated = 13,
    WorkspaceAppsChanged = 14,
    AppUpdated = 21,
    AppViewsChanged = 24,
    ViewUpdated = 31,
    ViewDeleted = 32,
    ViewRestored = 33,
    UserUnauthorized = 100,
    TrashUpdated = 1000,
}

impl ::protobuf::ProtobufEnum for FolderNotification {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<FolderNotification> {
        match value {
            0 => ::std::option::Option::Some(FolderNotification::Unknown),
            10 => ::std::option::Option::Some(FolderNotification::UserCreateWorkspace),
            11 => ::std::option::Option::Some(FolderNotification::UserDeleteWorkspace),
            12 => ::std::option::Option::Some(FolderNotification::WorkspaceUpdated),
            13 => ::std::option::Option::Some(FolderNotification::WorkspaceListUpdated),
            14 => ::std::option::Option::Some(FolderNotification::WorkspaceAppsChanged),
            21 => ::std::option::Option::Some(FolderNotification::AppUpdated),
            24 => ::std::option::Option::Some(FolderNotification::AppViewsChanged),
            31 => ::std::option::Option::Some(FolderNotification::ViewUpdated),
            32 => ::std::option::Option::Some(FolderNotification::ViewDeleted),
            33 => ::std::option::Option::Some(FolderNotification::ViewRestored),
            100 => ::std::option::Option::Some(FolderNotification::UserUnauthorized),
            1000 => ::std::option::Option::Some(FolderNotification::TrashUpdated),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [FolderNotification] = &[
            FolderNotification::Unknown,
            FolderNotification::UserCreateWorkspace,
            FolderNotification::UserDeleteWorkspace,
            FolderNotification::WorkspaceUpdated,
            FolderNotification::WorkspaceListUpdated,
            FolderNotification::WorkspaceAppsChanged,
            FolderNotification::AppUpdated,
            FolderNotification::AppViewsChanged,
            FolderNotification::ViewUpdated,
            FolderNotification::ViewDeleted,
            FolderNotification::ViewRestored,
            FolderNotification::UserUnauthorized,
            FolderNotification::TrashUpdated,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<FolderNotification>("FolderNotification", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for FolderNotification {
}

impl ::std::default::Default for FolderNotification {
    fn default() -> Self {
        FolderNotification::Unknown
    }
}

impl ::protobuf::reflect::ProtobufValue for FolderNotification {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17dart_notification.proto*\x9f\x02\n\x12FolderNotification\x12\x0b\n\
    \x07Unknown\x10\0\x12\x17\n\x13UserCreateWorkspace\x10\n\x12\x17\n\x13Us\
    erDeleteWorkspace\x10\x0b\x12\x14\n\x10WorkspaceUpdated\x10\x0c\x12\x18\
    \n\x14WorkspaceListUpdated\x10\r\x12\x18\n\x14WorkspaceAppsChanged\x10\
    \x0e\x12\x0e\n\nAppUpdated\x10\x15\x12\x13\n\x0fAppViewsChanged\x10\x18\
    \x12\x0f\n\x0bViewUpdated\x10\x1f\x12\x0f\n\x0bViewDeleted\x10\x20\x12\
    \x10\n\x0cViewRestored\x10!\x12\x14\n\x10UserUnauthorized\x10d\x12\x11\n\
    \x0cTrashUpdated\x10\xe8\x07b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
