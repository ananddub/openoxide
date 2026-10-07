// This is a generated file - do not edit.
//
// Generated from openoxide/api/v1/application.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class ApplicationSourceType extends $pb.ProtobufEnum {
  static const ApplicationSourceType APPLICATION_SOURCE_TYPE_UNSPECIFIED =
      ApplicationSourceType._(
          0, _omitEnumNames ? '' : 'APPLICATION_SOURCE_TYPE_UNSPECIFIED');
  static const ApplicationSourceType APPLICATION_SOURCE_TYPE_GITHUB =
      ApplicationSourceType._(
          1, _omitEnumNames ? '' : 'APPLICATION_SOURCE_TYPE_GITHUB');
  static const ApplicationSourceType APPLICATION_SOURCE_TYPE_GITLAB =
      ApplicationSourceType._(
          2, _omitEnumNames ? '' : 'APPLICATION_SOURCE_TYPE_GITLAB');
  static const ApplicationSourceType APPLICATION_SOURCE_TYPE_BITBUCKET =
      ApplicationSourceType._(
          3, _omitEnumNames ? '' : 'APPLICATION_SOURCE_TYPE_BITBUCKET');
  static const ApplicationSourceType APPLICATION_SOURCE_TYPE_GITEA =
      ApplicationSourceType._(
          4, _omitEnumNames ? '' : 'APPLICATION_SOURCE_TYPE_GITEA');
  static const ApplicationSourceType APPLICATION_SOURCE_TYPE_CUSTOM_GIT =
      ApplicationSourceType._(
          5, _omitEnumNames ? '' : 'APPLICATION_SOURCE_TYPE_CUSTOM_GIT');
  static const ApplicationSourceType APPLICATION_SOURCE_TYPE_DOCKER =
      ApplicationSourceType._(
          6, _omitEnumNames ? '' : 'APPLICATION_SOURCE_TYPE_DOCKER');
  static const ApplicationSourceType APPLICATION_SOURCE_TYPE_DROP =
      ApplicationSourceType._(
          7, _omitEnumNames ? '' : 'APPLICATION_SOURCE_TYPE_DROP');

  static const $core.List<ApplicationSourceType> values =
      <ApplicationSourceType>[
    APPLICATION_SOURCE_TYPE_UNSPECIFIED,
    APPLICATION_SOURCE_TYPE_GITHUB,
    APPLICATION_SOURCE_TYPE_GITLAB,
    APPLICATION_SOURCE_TYPE_BITBUCKET,
    APPLICATION_SOURCE_TYPE_GITEA,
    APPLICATION_SOURCE_TYPE_CUSTOM_GIT,
    APPLICATION_SOURCE_TYPE_DOCKER,
    APPLICATION_SOURCE_TYPE_DROP,
  ];

  static final $core.List<ApplicationSourceType?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 7);
  static ApplicationSourceType? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const ApplicationSourceType._(super.value, super.name);
}

class ApplicationBuildType extends $pb.ProtobufEnum {
  static const ApplicationBuildType APPLICATION_BUILD_TYPE_UNSPECIFIED =
      ApplicationBuildType._(
          0, _omitEnumNames ? '' : 'APPLICATION_BUILD_TYPE_UNSPECIFIED');
  static const ApplicationBuildType APPLICATION_BUILD_TYPE_BUILDPACKS =
      ApplicationBuildType._(
          1, _omitEnumNames ? '' : 'APPLICATION_BUILD_TYPE_BUILDPACKS');
  static const ApplicationBuildType APPLICATION_BUILD_TYPE_DOCKERFILE =
      ApplicationBuildType._(
          2, _omitEnumNames ? '' : 'APPLICATION_BUILD_TYPE_DOCKERFILE');
  static const ApplicationBuildType APPLICATION_BUILD_TYPE_NIXPACKS =
      ApplicationBuildType._(
          3, _omitEnumNames ? '' : 'APPLICATION_BUILD_TYPE_NIXPACKS');
  static const ApplicationBuildType APPLICATION_BUILD_TYPE_HEROKU =
      ApplicationBuildType._(
          4, _omitEnumNames ? '' : 'APPLICATION_BUILD_TYPE_HEROKU');
  static const ApplicationBuildType APPLICATION_BUILD_TYPE_PAKETO =
      ApplicationBuildType._(
          5, _omitEnumNames ? '' : 'APPLICATION_BUILD_TYPE_PAKETO');
  static const ApplicationBuildType APPLICATION_BUILD_TYPE_RAILPACK =
      ApplicationBuildType._(
          6, _omitEnumNames ? '' : 'APPLICATION_BUILD_TYPE_RAILPACK');
  static const ApplicationBuildType APPLICATION_BUILD_TYPE_STATIC =
      ApplicationBuildType._(
          7, _omitEnumNames ? '' : 'APPLICATION_BUILD_TYPE_STATIC');

  static const $core.List<ApplicationBuildType> values = <ApplicationBuildType>[
    APPLICATION_BUILD_TYPE_UNSPECIFIED,
    APPLICATION_BUILD_TYPE_BUILDPACKS,
    APPLICATION_BUILD_TYPE_DOCKERFILE,
    APPLICATION_BUILD_TYPE_NIXPACKS,
    APPLICATION_BUILD_TYPE_HEROKU,
    APPLICATION_BUILD_TYPE_PAKETO,
    APPLICATION_BUILD_TYPE_RAILPACK,
    APPLICATION_BUILD_TYPE_STATIC,
  ];

  static final $core.List<ApplicationBuildType?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 7);
  static ApplicationBuildType? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const ApplicationBuildType._(super.value, super.name);
}

class ApplicationPortProtocol extends $pb.ProtobufEnum {
  static const ApplicationPortProtocol APPLICATION_PORT_PROTOCOL_UNSPECIFIED =
      ApplicationPortProtocol._(
          0, _omitEnumNames ? '' : 'APPLICATION_PORT_PROTOCOL_UNSPECIFIED');
  static const ApplicationPortProtocol APPLICATION_PORT_PROTOCOL_TCP =
      ApplicationPortProtocol._(
          1, _omitEnumNames ? '' : 'APPLICATION_PORT_PROTOCOL_TCP');
  static const ApplicationPortProtocol APPLICATION_PORT_PROTOCOL_UDP =
      ApplicationPortProtocol._(
          2, _omitEnumNames ? '' : 'APPLICATION_PORT_PROTOCOL_UDP');

  static const $core.List<ApplicationPortProtocol> values =
      <ApplicationPortProtocol>[
    APPLICATION_PORT_PROTOCOL_UNSPECIFIED,
    APPLICATION_PORT_PROTOCOL_TCP,
    APPLICATION_PORT_PROTOCOL_UDP,
  ];

  static final $core.List<ApplicationPortProtocol?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 2);
  static ApplicationPortProtocol? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const ApplicationPortProtocol._(super.value, super.name);
}

class ApplicationPortPublishMode extends $pb.ProtobufEnum {
  static const ApplicationPortPublishMode
      APPLICATION_PORT_PUBLISH_MODE_UNSPECIFIED = ApplicationPortPublishMode._(
          0, _omitEnumNames ? '' : 'APPLICATION_PORT_PUBLISH_MODE_UNSPECIFIED');
  static const ApplicationPortPublishMode APPLICATION_PORT_PUBLISH_MODE_HOST =
      ApplicationPortPublishMode._(
          1, _omitEnumNames ? '' : 'APPLICATION_PORT_PUBLISH_MODE_HOST');
  static const ApplicationPortPublishMode
      APPLICATION_PORT_PUBLISH_MODE_INGRESS = ApplicationPortPublishMode._(
          2, _omitEnumNames ? '' : 'APPLICATION_PORT_PUBLISH_MODE_INGRESS');

  static const $core.List<ApplicationPortPublishMode> values =
      <ApplicationPortPublishMode>[
    APPLICATION_PORT_PUBLISH_MODE_UNSPECIFIED,
    APPLICATION_PORT_PUBLISH_MODE_HOST,
    APPLICATION_PORT_PUBLISH_MODE_INGRESS,
  ];

  static final $core.List<ApplicationPortPublishMode?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 2);
  static ApplicationPortPublishMode? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const ApplicationPortPublishMode._(super.value, super.name);
}

class ApplicationMountType extends $pb.ProtobufEnum {
  static const ApplicationMountType APPLICATION_MOUNT_TYPE_UNSPECIFIED =
      ApplicationMountType._(
          0, _omitEnumNames ? '' : 'APPLICATION_MOUNT_TYPE_UNSPECIFIED');
  static const ApplicationMountType APPLICATION_MOUNT_TYPE_VOLUME =
      ApplicationMountType._(
          1, _omitEnumNames ? '' : 'APPLICATION_MOUNT_TYPE_VOLUME');
  static const ApplicationMountType APPLICATION_MOUNT_TYPE_BIND =
      ApplicationMountType._(
          2, _omitEnumNames ? '' : 'APPLICATION_MOUNT_TYPE_BIND');
  static const ApplicationMountType APPLICATION_MOUNT_TYPE_FILE =
      ApplicationMountType._(
          3, _omitEnumNames ? '' : 'APPLICATION_MOUNT_TYPE_FILE');

  static const $core.List<ApplicationMountType> values = <ApplicationMountType>[
    APPLICATION_MOUNT_TYPE_UNSPECIFIED,
    APPLICATION_MOUNT_TYPE_VOLUME,
    APPLICATION_MOUNT_TYPE_BIND,
    APPLICATION_MOUNT_TYPE_FILE,
  ];

  static final $core.List<ApplicationMountType?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 3);
  static ApplicationMountType? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const ApplicationMountType._(super.value, super.name);
}

class ApplicationPatchType extends $pb.ProtobufEnum {
  static const ApplicationPatchType APPLICATION_PATCH_TYPE_UNSPECIFIED =
      ApplicationPatchType._(
          0, _omitEnumNames ? '' : 'APPLICATION_PATCH_TYPE_UNSPECIFIED');
  static const ApplicationPatchType APPLICATION_PATCH_TYPE_CREATE =
      ApplicationPatchType._(
          1, _omitEnumNames ? '' : 'APPLICATION_PATCH_TYPE_CREATE');
  static const ApplicationPatchType APPLICATION_PATCH_TYPE_UPDATE =
      ApplicationPatchType._(
          2, _omitEnumNames ? '' : 'APPLICATION_PATCH_TYPE_UPDATE');
  static const ApplicationPatchType APPLICATION_PATCH_TYPE_DELETE =
      ApplicationPatchType._(
          3, _omitEnumNames ? '' : 'APPLICATION_PATCH_TYPE_DELETE');

  static const $core.List<ApplicationPatchType> values = <ApplicationPatchType>[
    APPLICATION_PATCH_TYPE_UNSPECIFIED,
    APPLICATION_PATCH_TYPE_CREATE,
    APPLICATION_PATCH_TYPE_UPDATE,
    APPLICATION_PATCH_TYPE_DELETE,
  ];

  static final $core.List<ApplicationPatchType?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 3);
  static ApplicationPatchType? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const ApplicationPatchType._(super.value, super.name);
}

class ApplicationMiddlewareType extends $pb.ProtobufEnum {
  static const ApplicationMiddlewareType
      APPLICATION_MIDDLEWARE_TYPE_UNSPECIFIED = ApplicationMiddlewareType._(
          0, _omitEnumNames ? '' : 'APPLICATION_MIDDLEWARE_TYPE_UNSPECIFIED');
  static const ApplicationMiddlewareType APPLICATION_MIDDLEWARE_TYPE_COMPRESS =
      ApplicationMiddlewareType._(
          1, _omitEnumNames ? '' : 'APPLICATION_MIDDLEWARE_TYPE_COMPRESS');
  static const ApplicationMiddlewareType APPLICATION_MIDDLEWARE_TYPE_HEADERS =
      ApplicationMiddlewareType._(
          2, _omitEnumNames ? '' : 'APPLICATION_MIDDLEWARE_TYPE_HEADERS');
  static const ApplicationMiddlewareType
      APPLICATION_MIDDLEWARE_TYPE_RATE_LIMIT = ApplicationMiddlewareType._(
          3, _omitEnumNames ? '' : 'APPLICATION_MIDDLEWARE_TYPE_RATE_LIMIT');
  static const ApplicationMiddlewareType
      APPLICATION_MIDDLEWARE_TYPE_IP_ALLOWLIST = ApplicationMiddlewareType._(
          4, _omitEnumNames ? '' : 'APPLICATION_MIDDLEWARE_TYPE_IP_ALLOWLIST');

  static const $core.List<ApplicationMiddlewareType> values =
      <ApplicationMiddlewareType>[
    APPLICATION_MIDDLEWARE_TYPE_UNSPECIFIED,
    APPLICATION_MIDDLEWARE_TYPE_COMPRESS,
    APPLICATION_MIDDLEWARE_TYPE_HEADERS,
    APPLICATION_MIDDLEWARE_TYPE_RATE_LIMIT,
    APPLICATION_MIDDLEWARE_TYPE_IP_ALLOWLIST,
  ];

  static final $core.List<ApplicationMiddlewareType?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 4);
  static ApplicationMiddlewareType? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const ApplicationMiddlewareType._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');
