import 'package:mobile/src/gen/openoxide/api/v1/deployment.pb.dart';

/// What the UI may ask of a live log stream.
abstract class LogFeedEvent {
  const LogFeedEvent();
}

/// Start following. Returns once the subscription is up, not when data arrives.
class LogFeedStarted extends LogFeedEvent {
  const LogFeedStarted();
}

/// Stop listening. Safe to repeat, and after the stream already ended.
class LogFeedStopped extends LogFeedEvent {
  const LogFeedStopped();
}

/// Stop, drop the lines, and start again — the retry path for the same target.
class LogFeedRestarted extends LogFeedEvent {
  const LogFeedRestarted();
}

/// Lines arrived on the wire.
class LogFeedAppended extends LogFeedEvent {
  const LogFeedAppended(this.event);

  final DeploymentEvent event;
}

/// The panel closed the channel. The normal ending, not an error: it does this
/// once a deployment reaches a terminal state.
class LogFeedEnded extends LogFeedEvent {
  const LogFeedEnded();
}

/// The stream itself failed.
class LogFeedFailed extends LogFeedEvent {
  const LogFeedFailed(this.cause);

  final Object cause;
}
