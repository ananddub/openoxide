/// Where the app stands between launch and a usable screen.
enum AppStage {
  /// Asking the panel whether it exists and whether it has an owner yet.
  probing,

  /// The address is wrong or the panel is down. Retriable.
  unreachable,

  /// The panel answers but nobody owns it: there is no account to sign in to.
  needsSetup,

  signedOut,
  signedIn,
}
