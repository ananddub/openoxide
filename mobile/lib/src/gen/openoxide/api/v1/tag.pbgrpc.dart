// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/tag.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:async' as $async;
import 'dart:core' as $core;

import 'package:grpc/service_api.dart' as $grpc;
import 'package:protobuf/protobuf.dart' as $pb;

import 'common.pb.dart' as $1;
import 'tag.pb.dart' as $0;

export 'tag.pb.dart';

@$pb.GrpcServiceName('openoxide.api.v1.TagService')
class TagServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  TagServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.ListTagsResponse> listTags(
    $0.ListTagsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listTags, request, options: options);
  }

  $grpc.ResponseFuture<$0.GetTagResponse> getTag(
    $0.GetTagRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getTag, request, options: options);
  }

  $grpc.ResponseFuture<$0.CreateTagResponse> createTag(
    $0.CreateTagRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createTag, request, options: options);
  }

  $grpc.ResponseFuture<$0.UpdateTagResponse> updateTag(
    $0.UpdateTagRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateTag, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteTag(
    $0.DeleteTagRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteTag, request, options: options);
  }

  $grpc.ResponseFuture<$0.AddTagResponse> addTag(
    $0.AddTagRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$addTag, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> removeTag(
    $0.RemoveTagRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$removeTag, request, options: options);
  }

  $grpc.ResponseFuture<$0.ListProjectTagsResponse> listProjectTags(
    $0.ListProjectTagsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listProjectTags, request, options: options);
  }

  $grpc.ResponseFuture<$0.AttachProjectTagResponse> attachProjectTag(
    $0.AttachProjectTagRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$attachProjectTag, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> detachProjectTag(
    $0.DetachProjectTagRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$detachProjectTag, request, options: options);
  }

  // method descriptors

  static final _$listTags =
      $grpc.ClientMethod<$0.ListTagsRequest, $0.ListTagsResponse>(
          '/openoxide.api.v1.TagService/ListTags',
          ($0.ListTagsRequest value) => value.writeToBuffer(),
          $0.ListTagsResponse.fromBuffer);
  static final _$getTag =
      $grpc.ClientMethod<$0.GetTagRequest, $0.GetTagResponse>(
          '/openoxide.api.v1.TagService/GetTag',
          ($0.GetTagRequest value) => value.writeToBuffer(),
          $0.GetTagResponse.fromBuffer);
  static final _$createTag =
      $grpc.ClientMethod<$0.CreateTagRequest, $0.CreateTagResponse>(
          '/openoxide.api.v1.TagService/CreateTag',
          ($0.CreateTagRequest value) => value.writeToBuffer(),
          $0.CreateTagResponse.fromBuffer);
  static final _$updateTag =
      $grpc.ClientMethod<$0.UpdateTagRequest, $0.UpdateTagResponse>(
          '/openoxide.api.v1.TagService/UpdateTag',
          ($0.UpdateTagRequest value) => value.writeToBuffer(),
          $0.UpdateTagResponse.fromBuffer);
  static final _$deleteTag = $grpc.ClientMethod<$0.DeleteTagRequest, $1.Empty>(
      '/openoxide.api.v1.TagService/DeleteTag',
      ($0.DeleteTagRequest value) => value.writeToBuffer(),
      $1.Empty.fromBuffer);
  static final _$addTag =
      $grpc.ClientMethod<$0.AddTagRequest, $0.AddTagResponse>(
          '/openoxide.api.v1.TagService/AddTag',
          ($0.AddTagRequest value) => value.writeToBuffer(),
          $0.AddTagResponse.fromBuffer);
  static final _$removeTag = $grpc.ClientMethod<$0.RemoveTagRequest, $1.Empty>(
      '/openoxide.api.v1.TagService/RemoveTag',
      ($0.RemoveTagRequest value) => value.writeToBuffer(),
      $1.Empty.fromBuffer);
  static final _$listProjectTags =
      $grpc.ClientMethod<$0.ListProjectTagsRequest, $0.ListProjectTagsResponse>(
          '/openoxide.api.v1.TagService/ListProjectTags',
          ($0.ListProjectTagsRequest value) => value.writeToBuffer(),
          $0.ListProjectTagsResponse.fromBuffer);
  static final _$attachProjectTag = $grpc.ClientMethod<
          $0.AttachProjectTagRequest, $0.AttachProjectTagResponse>(
      '/openoxide.api.v1.TagService/AttachProjectTag',
      ($0.AttachProjectTagRequest value) => value.writeToBuffer(),
      $0.AttachProjectTagResponse.fromBuffer);
  static final _$detachProjectTag =
      $grpc.ClientMethod<$0.DetachProjectTagRequest, $1.Empty>(
          '/openoxide.api.v1.TagService/DetachProjectTag',
          ($0.DetachProjectTagRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.TagService')
abstract class TagServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.TagService';

  TagServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.ListTagsRequest, $0.ListTagsResponse>(
        'ListTags',
        listTags_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.ListTagsRequest.fromBuffer(value),
        ($0.ListTagsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GetTagRequest, $0.GetTagResponse>(
        'GetTag',
        getTag_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.GetTagRequest.fromBuffer(value),
        ($0.GetTagResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.CreateTagRequest, $0.CreateTagResponse>(
        'CreateTag',
        createTag_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.CreateTagRequest.fromBuffer(value),
        ($0.CreateTagResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.UpdateTagRequest, $0.UpdateTagResponse>(
        'UpdateTag',
        updateTag_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.UpdateTagRequest.fromBuffer(value),
        ($0.UpdateTagResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DeleteTagRequest, $1.Empty>(
        'DeleteTag',
        deleteTag_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.DeleteTagRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.AddTagRequest, $0.AddTagResponse>(
        'AddTag',
        addTag_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.AddTagRequest.fromBuffer(value),
        ($0.AddTagResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RemoveTagRequest, $1.Empty>(
        'RemoveTag',
        removeTag_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.RemoveTagRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ListProjectTagsRequest,
            $0.ListProjectTagsResponse>(
        'ListProjectTags',
        listProjectTags_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListProjectTagsRequest.fromBuffer(value),
        ($0.ListProjectTagsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.AttachProjectTagRequest,
            $0.AttachProjectTagResponse>(
        'AttachProjectTag',
        attachProjectTag_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.AttachProjectTagRequest.fromBuffer(value),
        ($0.AttachProjectTagResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DetachProjectTagRequest, $1.Empty>(
        'DetachProjectTag',
        detachProjectTag_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DetachProjectTagRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
  }

  $async.Future<$0.ListTagsResponse> listTags_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ListTagsRequest> $request) async {
    return listTags($call, await $request);
  }

  $async.Future<$0.ListTagsResponse> listTags(
      $grpc.ServiceCall call, $0.ListTagsRequest request);

  $async.Future<$0.GetTagResponse> getTag_Pre(
      $grpc.ServiceCall $call, $async.Future<$0.GetTagRequest> $request) async {
    return getTag($call, await $request);
  }

  $async.Future<$0.GetTagResponse> getTag(
      $grpc.ServiceCall call, $0.GetTagRequest request);

  $async.Future<$0.CreateTagResponse> createTag_Pre($grpc.ServiceCall $call,
      $async.Future<$0.CreateTagRequest> $request) async {
    return createTag($call, await $request);
  }

  $async.Future<$0.CreateTagResponse> createTag(
      $grpc.ServiceCall call, $0.CreateTagRequest request);

  $async.Future<$0.UpdateTagResponse> updateTag_Pre($grpc.ServiceCall $call,
      $async.Future<$0.UpdateTagRequest> $request) async {
    return updateTag($call, await $request);
  }

  $async.Future<$0.UpdateTagResponse> updateTag(
      $grpc.ServiceCall call, $0.UpdateTagRequest request);

  $async.Future<$1.Empty> deleteTag_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DeleteTagRequest> $request) async {
    return deleteTag($call, await $request);
  }

  $async.Future<$1.Empty> deleteTag(
      $grpc.ServiceCall call, $0.DeleteTagRequest request);

  $async.Future<$0.AddTagResponse> addTag_Pre(
      $grpc.ServiceCall $call, $async.Future<$0.AddTagRequest> $request) async {
    return addTag($call, await $request);
  }

  $async.Future<$0.AddTagResponse> addTag(
      $grpc.ServiceCall call, $0.AddTagRequest request);

  $async.Future<$1.Empty> removeTag_Pre($grpc.ServiceCall $call,
      $async.Future<$0.RemoveTagRequest> $request) async {
    return removeTag($call, await $request);
  }

  $async.Future<$1.Empty> removeTag(
      $grpc.ServiceCall call, $0.RemoveTagRequest request);

  $async.Future<$0.ListProjectTagsResponse> listProjectTags_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListProjectTagsRequest> $request) async {
    return listProjectTags($call, await $request);
  }

  $async.Future<$0.ListProjectTagsResponse> listProjectTags(
      $grpc.ServiceCall call, $0.ListProjectTagsRequest request);

  $async.Future<$0.AttachProjectTagResponse> attachProjectTag_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.AttachProjectTagRequest> $request) async {
    return attachProjectTag($call, await $request);
  }

  $async.Future<$0.AttachProjectTagResponse> attachProjectTag(
      $grpc.ServiceCall call, $0.AttachProjectTagRequest request);

  $async.Future<$1.Empty> detachProjectTag_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DetachProjectTagRequest> $request) async {
    return detachProjectTag($call, await $request);
  }

  $async.Future<$1.Empty> detachProjectTag(
      $grpc.ServiceCall call, $0.DetachProjectTagRequest request);
}
