// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.22.

// ignore_for_file: unused_import, unused_element, unnecessary_import, duplicate_ignore, invalid_use_of_internal_member, annotate_overrides, non_constant_identifier_names, curly_braces_in_flow_control_structures, prefer_const_literals_to_create_immutables, unused_field

import 'dart:async';
import 'dart:convert';
import 'dart:ffi' as ffi;
import 'frb_generated.dart';
import 'libxmtp_api.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_io.dart';

abstract class RustLibApiImplPlatform extends BaseApiImpl<RustLibWire> {
  RustLibApiImplPlatform({
    required super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
    required super.portManager,
  });

  CrossPlatformFinalizerArg get rust_arc_decrement_strong_count_ApiErrorPtr => wire
      ._rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockApiErrorPtr;

  CrossPlatformFinalizerArg
      get rust_arc_decrement_strong_count_ArcXmtpClientPtr => wire
          ._rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockArcXmtpClientPtr;

  CrossPlatformFinalizerArg get rust_arc_decrement_strong_count_ClientPtr => wire
      ._rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClientPtr;

  CrossPlatformFinalizerArg
      get rust_arc_decrement_strong_count_ClientBuilderErrorPtr => wire
          ._rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClientBuilderErrorPtr;

  CrossPlatformFinalizerArg
      get rust_arc_decrement_strong_count_StorageErrorPtr => wire
          ._rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockStorageErrorPtr;

  CrossPlatformFinalizerArg
      get rust_arc_decrement_strong_count_AnyhowErrorPtr => wire
          ._rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockanyhowErrorPtr;

  CrossPlatformFinalizerArg
      get rust_arc_decrement_strong_count_XmtpMlsClientClientErrorPtr => wire
          ._rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsclientClientErrorPtr;

  CrossPlatformFinalizerArg
      get rust_arc_decrement_strong_count_XmtpMlsGroupsGroupErrorPtr => wire
          ._rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsgroupsGroupErrorPtr;

  @protected
  ApiError
      dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockApiError(
          dynamic raw);

  @protected
  ArcXmtpClient
      dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockArcXmtpClient(
          dynamic raw);

  @protected
  Client
      dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClient(
          dynamic raw);

  @protected
  ClientBuilderError
      dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClientBuilderError(
          dynamic raw);

  @protected
  StorageError
      dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockStorageError(
          dynamic raw);

  @protected
  AnyhowError
      dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockanyhowError(
          dynamic raw);

  @protected
  XmtpMlsClientClientError
      dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsclientClientError(
          dynamic raw);

  @protected
  XmtpMlsGroupsGroupError
      dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsgroupsGroupError(
          dynamic raw);

  @protected
  Client
      dco_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClient(
          dynamic raw);

  @protected
  ApiError
      dco_decode_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockApiError(
          dynamic raw);

  @protected
  ArcXmtpClient
      dco_decode_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockArcXmtpClient(
          dynamic raw);

  @protected
  Client
      dco_decode_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClient(
          dynamic raw);

  @protected
  ClientBuilderError
      dco_decode_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClientBuilderError(
          dynamic raw);

  @protected
  StorageError
      dco_decode_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockStorageError(
          dynamic raw);

  @protected
  AnyhowError
      dco_decode_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockanyhowError(
          dynamic raw);

  @protected
  XmtpMlsClientClientError
      dco_decode_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsclientClientError(
          dynamic raw);

  @protected
  XmtpMlsGroupsGroupError
      dco_decode_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsgroupsGroupError(
          dynamic raw);

  @protected
  String dco_decode_String(dynamic raw);

  @protected
  bool dco_decode_bool(dynamic raw);

  @protected
  int dco_decode_box_autoadd_i_64(dynamic raw);

  @protected
  SignatureRequiredClient dco_decode_box_autoadd_signature_required_client(
      dynamic raw);

  @protected
  CreatedClient dco_decode_created_client(dynamic raw);

  @protected
  Group dco_decode_group(dynamic raw);

  @protected
  GroupMember dco_decode_group_member(dynamic raw);

  @protected
  int dco_decode_i_64(dynamic raw);

  @protected
  List<String> dco_decode_list_String(dynamic raw);

  @protected
  List<Group> dco_decode_list_group(dynamic raw);

  @protected
  List<GroupMember> dco_decode_list_group_member(dynamic raw);

  @protected
  List<Uint8List> dco_decode_list_list_prim_u_8_strict(dynamic raw);

  @protected
  List<Message> dco_decode_list_message(dynamic raw);

  @protected
  List<int> dco_decode_list_prim_u_8_loose(dynamic raw);

  @protected
  Uint8List dco_decode_list_prim_u_8_strict(dynamic raw);

  @protected
  Message dco_decode_message(dynamic raw);

  @protected
  int? dco_decode_opt_box_autoadd_i_64(dynamic raw);

  @protected
  SignatureRequiredClient dco_decode_signature_required_client(dynamic raw);

  @protected
  int dco_decode_u_8(dynamic raw);

  @protected
  U8Array32 dco_decode_u_8_array_32(dynamic raw);

  @protected
  void dco_decode_unit(dynamic raw);

  @protected
  int dco_decode_usize(dynamic raw);

  @protected
  XmtpError dco_decode_xmtp_error(dynamic raw);

  @protected
  ApiError
      sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockApiError(
          SseDeserializer deserializer);

  @protected
  ArcXmtpClient
      sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockArcXmtpClient(
          SseDeserializer deserializer);

  @protected
  Client
      sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClient(
          SseDeserializer deserializer);

  @protected
  ClientBuilderError
      sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClientBuilderError(
          SseDeserializer deserializer);

  @protected
  StorageError
      sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockStorageError(
          SseDeserializer deserializer);

  @protected
  AnyhowError
      sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockanyhowError(
          SseDeserializer deserializer);

  @protected
  XmtpMlsClientClientError
      sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsclientClientError(
          SseDeserializer deserializer);

  @protected
  XmtpMlsGroupsGroupError
      sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsgroupsGroupError(
          SseDeserializer deserializer);

  @protected
  Client
      sse_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClient(
          SseDeserializer deserializer);

  @protected
  ApiError
      sse_decode_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockApiError(
          SseDeserializer deserializer);

  @protected
  ArcXmtpClient
      sse_decode_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockArcXmtpClient(
          SseDeserializer deserializer);

  @protected
  Client
      sse_decode_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClient(
          SseDeserializer deserializer);

  @protected
  ClientBuilderError
      sse_decode_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClientBuilderError(
          SseDeserializer deserializer);

  @protected
  StorageError
      sse_decode_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockStorageError(
          SseDeserializer deserializer);

  @protected
  AnyhowError
      sse_decode_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockanyhowError(
          SseDeserializer deserializer);

  @protected
  XmtpMlsClientClientError
      sse_decode_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsclientClientError(
          SseDeserializer deserializer);

  @protected
  XmtpMlsGroupsGroupError
      sse_decode_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsgroupsGroupError(
          SseDeserializer deserializer);

  @protected
  String sse_decode_String(SseDeserializer deserializer);

  @protected
  bool sse_decode_bool(SseDeserializer deserializer);

  @protected
  int sse_decode_box_autoadd_i_64(SseDeserializer deserializer);

  @protected
  SignatureRequiredClient sse_decode_box_autoadd_signature_required_client(
      SseDeserializer deserializer);

  @protected
  CreatedClient sse_decode_created_client(SseDeserializer deserializer);

  @protected
  Group sse_decode_group(SseDeserializer deserializer);

  @protected
  GroupMember sse_decode_group_member(SseDeserializer deserializer);

  @protected
  int sse_decode_i_64(SseDeserializer deserializer);

  @protected
  List<String> sse_decode_list_String(SseDeserializer deserializer);

  @protected
  List<Group> sse_decode_list_group(SseDeserializer deserializer);

  @protected
  List<GroupMember> sse_decode_list_group_member(SseDeserializer deserializer);

  @protected
  List<Uint8List> sse_decode_list_list_prim_u_8_strict(
      SseDeserializer deserializer);

  @protected
  List<Message> sse_decode_list_message(SseDeserializer deserializer);

  @protected
  List<int> sse_decode_list_prim_u_8_loose(SseDeserializer deserializer);

  @protected
  Uint8List sse_decode_list_prim_u_8_strict(SseDeserializer deserializer);

  @protected
  Message sse_decode_message(SseDeserializer deserializer);

  @protected
  int? sse_decode_opt_box_autoadd_i_64(SseDeserializer deserializer);

  @protected
  SignatureRequiredClient sse_decode_signature_required_client(
      SseDeserializer deserializer);

  @protected
  int sse_decode_u_8(SseDeserializer deserializer);

  @protected
  U8Array32 sse_decode_u_8_array_32(SseDeserializer deserializer);

  @protected
  void sse_decode_unit(SseDeserializer deserializer);

  @protected
  int sse_decode_usize(SseDeserializer deserializer);

  @protected
  XmtpError sse_decode_xmtp_error(SseDeserializer deserializer);

  @protected
  int sse_decode_i_32(SseDeserializer deserializer);

  @protected
  void
      sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockApiError(
          ApiError self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockArcXmtpClient(
          ArcXmtpClient self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClient(
          Client self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClientBuilderError(
          ClientBuilderError self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockStorageError(
          StorageError self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockanyhowError(
          AnyhowError self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsclientClientError(
          XmtpMlsClientClientError self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsgroupsGroupError(
          XmtpMlsGroupsGroupError self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClient(
          Client self, SseSerializer serializer);

  @protected
  void
      sse_encode_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockApiError(
          ApiError self, SseSerializer serializer);

  @protected
  void
      sse_encode_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockArcXmtpClient(
          ArcXmtpClient self, SseSerializer serializer);

  @protected
  void
      sse_encode_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClient(
          Client self, SseSerializer serializer);

  @protected
  void
      sse_encode_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClientBuilderError(
          ClientBuilderError self, SseSerializer serializer);

  @protected
  void
      sse_encode_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockStorageError(
          StorageError self, SseSerializer serializer);

  @protected
  void
      sse_encode_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockanyhowError(
          AnyhowError self, SseSerializer serializer);

  @protected
  void
      sse_encode_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsclientClientError(
          XmtpMlsClientClientError self, SseSerializer serializer);

  @protected
  void
      sse_encode_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsgroupsGroupError(
          XmtpMlsGroupsGroupError self, SseSerializer serializer);

  @protected
  void sse_encode_String(String self, SseSerializer serializer);

  @protected
  void sse_encode_bool(bool self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_i_64(int self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_signature_required_client(
      SignatureRequiredClient self, SseSerializer serializer);

  @protected
  void sse_encode_created_client(CreatedClient self, SseSerializer serializer);

  @protected
  void sse_encode_group(Group self, SseSerializer serializer);

  @protected
  void sse_encode_group_member(GroupMember self, SseSerializer serializer);

  @protected
  void sse_encode_i_64(int self, SseSerializer serializer);

  @protected
  void sse_encode_list_String(List<String> self, SseSerializer serializer);

  @protected
  void sse_encode_list_group(List<Group> self, SseSerializer serializer);

  @protected
  void sse_encode_list_group_member(
      List<GroupMember> self, SseSerializer serializer);

  @protected
  void sse_encode_list_list_prim_u_8_strict(
      List<Uint8List> self, SseSerializer serializer);

  @protected
  void sse_encode_list_message(List<Message> self, SseSerializer serializer);

  @protected
  void sse_encode_list_prim_u_8_loose(List<int> self, SseSerializer serializer);

  @protected
  void sse_encode_list_prim_u_8_strict(
      Uint8List self, SseSerializer serializer);

  @protected
  void sse_encode_message(Message self, SseSerializer serializer);

  @protected
  void sse_encode_opt_box_autoadd_i_64(int? self, SseSerializer serializer);

  @protected
  void sse_encode_signature_required_client(
      SignatureRequiredClient self, SseSerializer serializer);

  @protected
  void sse_encode_u_8(int self, SseSerializer serializer);

  @protected
  void sse_encode_u_8_array_32(U8Array32 self, SseSerializer serializer);

  @protected
  void sse_encode_unit(void self, SseSerializer serializer);

  @protected
  void sse_encode_usize(int self, SseSerializer serializer);

  @protected
  void sse_encode_xmtp_error(XmtpError self, SseSerializer serializer);

  @protected
  void sse_encode_i_32(int self, SseSerializer serializer);
}

// Section: wire_class

class RustLibWire implements BaseWire {
  factory RustLibWire.fromExternalLibrary(ExternalLibrary lib) =>
      RustLibWire(lib.ffiDynamicLibrary);

  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
      _lookup;

  /// The symbols are looked up in [dynamicLibrary].
  RustLibWire(ffi.DynamicLibrary dynamicLibrary)
      : _lookup = dynamicLibrary.lookup;

  void
      rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockApiError(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockApiError(
      ptr,
    );
  }

  late final _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockApiErrorPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_xmtp_bindings_flutter_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockApiError');
  late final _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockApiError =
      _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockApiErrorPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  void
      rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockApiError(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockApiError(
      ptr,
    );
  }

  late final _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockApiErrorPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_xmtp_bindings_flutter_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockApiError');
  late final _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockApiError =
      _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockApiErrorPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  void
      rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockArcXmtpClient(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockArcXmtpClient(
      ptr,
    );
  }

  late final _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockArcXmtpClientPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_xmtp_bindings_flutter_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockArcXmtpClient');
  late final _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockArcXmtpClient =
      _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockArcXmtpClientPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  void
      rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockArcXmtpClient(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockArcXmtpClient(
      ptr,
    );
  }

  late final _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockArcXmtpClientPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_xmtp_bindings_flutter_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockArcXmtpClient');
  late final _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockArcXmtpClient =
      _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockArcXmtpClientPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  void
      rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClient(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClient(
      ptr,
    );
  }

  late final _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClientPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_xmtp_bindings_flutter_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClient');
  late final _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClient =
      _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClientPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  void
      rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClient(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClient(
      ptr,
    );
  }

  late final _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClientPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_xmtp_bindings_flutter_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClient');
  late final _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClient =
      _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClientPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  void
      rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClientBuilderError(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClientBuilderError(
      ptr,
    );
  }

  late final _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClientBuilderErrorPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_xmtp_bindings_flutter_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClientBuilderError');
  late final _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClientBuilderError =
      _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClientBuilderErrorPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  void
      rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClientBuilderError(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClientBuilderError(
      ptr,
    );
  }

  late final _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClientBuilderErrorPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_xmtp_bindings_flutter_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClientBuilderError');
  late final _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClientBuilderError =
      _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockClientBuilderErrorPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  void
      rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockStorageError(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockStorageError(
      ptr,
    );
  }

  late final _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockStorageErrorPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_xmtp_bindings_flutter_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockStorageError');
  late final _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockStorageError =
      _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockStorageErrorPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  void
      rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockStorageError(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockStorageError(
      ptr,
    );
  }

  late final _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockStorageErrorPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_xmtp_bindings_flutter_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockStorageError');
  late final _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockStorageError =
      _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockStorageErrorPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  void
      rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockanyhowError(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockanyhowError(
      ptr,
    );
  }

  late final _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockanyhowErrorPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_xmtp_bindings_flutter_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockanyhowError');
  late final _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockanyhowError =
      _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockanyhowErrorPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  void
      rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockanyhowError(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockanyhowError(
      ptr,
    );
  }

  late final _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockanyhowErrorPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_xmtp_bindings_flutter_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockanyhowError');
  late final _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockanyhowError =
      _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockanyhowErrorPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  void
      rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsclientClientError(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsclientClientError(
      ptr,
    );
  }

  late final _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsclientClientErrorPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_xmtp_bindings_flutter_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsclientClientError');
  late final _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsclientClientError =
      _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsclientClientErrorPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  void
      rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsclientClientError(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsclientClientError(
      ptr,
    );
  }

  late final _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsclientClientErrorPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_xmtp_bindings_flutter_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsclientClientError');
  late final _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsclientClientError =
      _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsclientClientErrorPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  void
      rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsgroupsGroupError(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsgroupsGroupError(
      ptr,
    );
  }

  late final _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsgroupsGroupErrorPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_xmtp_bindings_flutter_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsgroupsGroupError');
  late final _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsgroupsGroupError =
      _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsgroupsGroupErrorPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  void
      rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsgroupsGroupError(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsgroupsGroupError(
      ptr,
    );
  }

  late final _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsgroupsGroupErrorPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_xmtp_bindings_flutter_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsgroupsGroupError');
  late final _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsgroupsGroupError =
      _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockxmtp_mlsgroupsGroupErrorPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();
}
