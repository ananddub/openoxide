// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/notification.proto.

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
import 'notification.pb.dart' as $0;

export 'notification.pb.dart';

/// Sole notification owner service; absorbs the former NotificationOperationsService (delivery history and resource bindings included).
@$pb.GrpcServiceName('openoxide.api.v1.NotificationService')
class NotificationServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  NotificationServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.ListNotificationsResponse> listNotifications(
    $0.ListNotificationsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listNotifications, request, options: options);
  }

  $grpc.ResponseFuture<$0.GetNotificationResponse> getNotification(
    $0.GetNotificationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getNotification, request, options: options);
  }

  $grpc.ResponseFuture<$0.CreateNotificationResponse> createNotification(
    $0.CreateNotificationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createNotification, request, options: options);
  }

  $grpc.ResponseFuture<$0.UpdateNotificationResponse> updateNotification(
    $0.UpdateNotificationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateNotification, request, options: options);
  }

  $grpc.ResponseFuture<$0.ReplaceNotificationResponse> replaceNotification(
    $0.ReplaceNotificationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$replaceNotification, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteNotification(
    $0.DeleteNotificationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteNotification, request, options: options);
  }

  $grpc.ResponseFuture<$0.TestNotificationResponse> testNotification(
    $0.TestNotificationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$testNotification, request, options: options);
  }

  $grpc.ResponseFuture<$0.ListNotificationDeliveryHistoryResponse>
      listDeliveryHistory(
    $0.ListNotificationDeliveryHistoryRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listDeliveryHistory, request, options: options);
  }

  $grpc.ResponseFuture<$0.CreateNotificationBindingResponse> createBinding(
    $0.CreateNotificationBindingRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createBinding, request, options: options);
  }

  $grpc.ResponseFuture<$0.ListNotificationBindingsResponse> listBindings(
    $0.ListNotificationBindingsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listBindings, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteBinding(
    $0.DeleteNotificationBindingRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteBinding, request, options: options);
  }

  // method descriptors

  static final _$listNotifications = $grpc.ClientMethod<
          $0.ListNotificationsRequest, $0.ListNotificationsResponse>(
      '/openoxide.api.v1.NotificationService/ListNotifications',
      ($0.ListNotificationsRequest value) => value.writeToBuffer(),
      $0.ListNotificationsResponse.fromBuffer);
  static final _$getNotification =
      $grpc.ClientMethod<$0.GetNotificationRequest, $0.GetNotificationResponse>(
          '/openoxide.api.v1.NotificationService/GetNotification',
          ($0.GetNotificationRequest value) => value.writeToBuffer(),
          $0.GetNotificationResponse.fromBuffer);
  static final _$createNotification = $grpc.ClientMethod<
          $0.CreateNotificationRequest, $0.CreateNotificationResponse>(
      '/openoxide.api.v1.NotificationService/CreateNotification',
      ($0.CreateNotificationRequest value) => value.writeToBuffer(),
      $0.CreateNotificationResponse.fromBuffer);
  static final _$updateNotification = $grpc.ClientMethod<
          $0.UpdateNotificationRequest, $0.UpdateNotificationResponse>(
      '/openoxide.api.v1.NotificationService/UpdateNotification',
      ($0.UpdateNotificationRequest value) => value.writeToBuffer(),
      $0.UpdateNotificationResponse.fromBuffer);
  static final _$replaceNotification = $grpc.ClientMethod<
          $0.ReplaceNotificationRequest, $0.ReplaceNotificationResponse>(
      '/openoxide.api.v1.NotificationService/ReplaceNotification',
      ($0.ReplaceNotificationRequest value) => value.writeToBuffer(),
      $0.ReplaceNotificationResponse.fromBuffer);
  static final _$deleteNotification =
      $grpc.ClientMethod<$0.DeleteNotificationRequest, $1.Empty>(
          '/openoxide.api.v1.NotificationService/DeleteNotification',
          ($0.DeleteNotificationRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$testNotification = $grpc.ClientMethod<
          $0.TestNotificationRequest, $0.TestNotificationResponse>(
      '/openoxide.api.v1.NotificationService/TestNotification',
      ($0.TestNotificationRequest value) => value.writeToBuffer(),
      $0.TestNotificationResponse.fromBuffer);
  static final _$listDeliveryHistory = $grpc.ClientMethod<
          $0.ListNotificationDeliveryHistoryRequest,
          $0.ListNotificationDeliveryHistoryResponse>(
      '/openoxide.api.v1.NotificationService/ListDeliveryHistory',
      ($0.ListNotificationDeliveryHistoryRequest value) =>
          value.writeToBuffer(),
      $0.ListNotificationDeliveryHistoryResponse.fromBuffer);
  static final _$createBinding = $grpc.ClientMethod<
          $0.CreateNotificationBindingRequest,
          $0.CreateNotificationBindingResponse>(
      '/openoxide.api.v1.NotificationService/CreateBinding',
      ($0.CreateNotificationBindingRequest value) => value.writeToBuffer(),
      $0.CreateNotificationBindingResponse.fromBuffer);
  static final _$listBindings = $grpc.ClientMethod<
          $0.ListNotificationBindingsRequest,
          $0.ListNotificationBindingsResponse>(
      '/openoxide.api.v1.NotificationService/ListBindings',
      ($0.ListNotificationBindingsRequest value) => value.writeToBuffer(),
      $0.ListNotificationBindingsResponse.fromBuffer);
  static final _$deleteBinding =
      $grpc.ClientMethod<$0.DeleteNotificationBindingRequest, $1.Empty>(
          '/openoxide.api.v1.NotificationService/DeleteBinding',
          ($0.DeleteNotificationBindingRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
}

@$pb.GrpcServiceName('openoxide.api.v1.NotificationService')
abstract class NotificationServiceBase extends $grpc.Service {
  $core.String get $name => 'openoxide.api.v1.NotificationService';

  NotificationServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.ListNotificationsRequest,
            $0.ListNotificationsResponse>(
        'ListNotifications',
        listNotifications_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListNotificationsRequest.fromBuffer(value),
        ($0.ListNotificationsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GetNotificationRequest,
            $0.GetNotificationResponse>(
        'GetNotification',
        getNotification_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GetNotificationRequest.fromBuffer(value),
        ($0.GetNotificationResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.CreateNotificationRequest,
            $0.CreateNotificationResponse>(
        'CreateNotification',
        createNotification_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.CreateNotificationRequest.fromBuffer(value),
        ($0.CreateNotificationResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.UpdateNotificationRequest,
            $0.UpdateNotificationResponse>(
        'UpdateNotification',
        updateNotification_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.UpdateNotificationRequest.fromBuffer(value),
        ($0.UpdateNotificationResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ReplaceNotificationRequest,
            $0.ReplaceNotificationResponse>(
        'ReplaceNotification',
        replaceNotification_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ReplaceNotificationRequest.fromBuffer(value),
        ($0.ReplaceNotificationResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DeleteNotificationRequest, $1.Empty>(
        'DeleteNotification',
        deleteNotification_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.DeleteNotificationRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.TestNotificationRequest,
            $0.TestNotificationResponse>(
        'TestNotification',
        testNotification_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.TestNotificationRequest.fromBuffer(value),
        ($0.TestNotificationResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ListNotificationDeliveryHistoryRequest,
            $0.ListNotificationDeliveryHistoryResponse>(
        'ListDeliveryHistory',
        listDeliveryHistory_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListNotificationDeliveryHistoryRequest.fromBuffer(value),
        ($0.ListNotificationDeliveryHistoryResponse value) =>
            value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.CreateNotificationBindingRequest,
            $0.CreateNotificationBindingResponse>(
        'CreateBinding',
        createBinding_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.CreateNotificationBindingRequest.fromBuffer(value),
        ($0.CreateNotificationBindingResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ListNotificationBindingsRequest,
            $0.ListNotificationBindingsResponse>(
        'ListBindings',
        listBindings_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListNotificationBindingsRequest.fromBuffer(value),
        ($0.ListNotificationBindingsResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.DeleteNotificationBindingRequest, $1.Empty>(
            'DeleteBinding',
            deleteBinding_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.DeleteNotificationBindingRequest.fromBuffer(value),
            ($1.Empty value) => value.writeToBuffer()));
  }

  $async.Future<$0.ListNotificationsResponse> listNotifications_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListNotificationsRequest> $request) async {
    return listNotifications($call, await $request);
  }

  $async.Future<$0.ListNotificationsResponse> listNotifications(
      $grpc.ServiceCall call, $0.ListNotificationsRequest request);

  $async.Future<$0.GetNotificationResponse> getNotification_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.GetNotificationRequest> $request) async {
    return getNotification($call, await $request);
  }

  $async.Future<$0.GetNotificationResponse> getNotification(
      $grpc.ServiceCall call, $0.GetNotificationRequest request);

  $async.Future<$0.CreateNotificationResponse> createNotification_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.CreateNotificationRequest> $request) async {
    return createNotification($call, await $request);
  }

  $async.Future<$0.CreateNotificationResponse> createNotification(
      $grpc.ServiceCall call, $0.CreateNotificationRequest request);

  $async.Future<$0.UpdateNotificationResponse> updateNotification_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.UpdateNotificationRequest> $request) async {
    return updateNotification($call, await $request);
  }

  $async.Future<$0.UpdateNotificationResponse> updateNotification(
      $grpc.ServiceCall call, $0.UpdateNotificationRequest request);

  $async.Future<$0.ReplaceNotificationResponse> replaceNotification_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ReplaceNotificationRequest> $request) async {
    return replaceNotification($call, await $request);
  }

  $async.Future<$0.ReplaceNotificationResponse> replaceNotification(
      $grpc.ServiceCall call, $0.ReplaceNotificationRequest request);

  $async.Future<$1.Empty> deleteNotification_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DeleteNotificationRequest> $request) async {
    return deleteNotification($call, await $request);
  }

  $async.Future<$1.Empty> deleteNotification(
      $grpc.ServiceCall call, $0.DeleteNotificationRequest request);

  $async.Future<$0.TestNotificationResponse> testNotification_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.TestNotificationRequest> $request) async {
    return testNotification($call, await $request);
  }

  $async.Future<$0.TestNotificationResponse> testNotification(
      $grpc.ServiceCall call, $0.TestNotificationRequest request);

  $async.Future<$0.ListNotificationDeliveryHistoryResponse>
      listDeliveryHistory_Pre(
          $grpc.ServiceCall $call,
          $async.Future<$0.ListNotificationDeliveryHistoryRequest>
              $request) async {
    return listDeliveryHistory($call, await $request);
  }

  $async.Future<$0.ListNotificationDeliveryHistoryResponse> listDeliveryHistory(
      $grpc.ServiceCall call,
      $0.ListNotificationDeliveryHistoryRequest request);

  $async.Future<$0.CreateNotificationBindingResponse> createBinding_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.CreateNotificationBindingRequest> $request) async {
    return createBinding($call, await $request);
  }

  $async.Future<$0.CreateNotificationBindingResponse> createBinding(
      $grpc.ServiceCall call, $0.CreateNotificationBindingRequest request);

  $async.Future<$0.ListNotificationBindingsResponse> listBindings_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListNotificationBindingsRequest> $request) async {
    return listBindings($call, await $request);
  }

  $async.Future<$0.ListNotificationBindingsResponse> listBindings(
      $grpc.ServiceCall call, $0.ListNotificationBindingsRequest request);

  $async.Future<$1.Empty> deleteBinding_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DeleteNotificationBindingRequest> $request) async {
    return deleteBinding($call, await $request);
  }

  $async.Future<$1.Empty> deleteBinding(
      $grpc.ServiceCall call, $0.DeleteNotificationBindingRequest request);
}
