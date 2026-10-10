/// Turns an enum's wire name into a label a reader can scan:
/// `REMOTE_SERVER_STATUS_ACTIVE` → "Active", `OVERVIEW_SERVICE_TYPE_DB` → "Db".
/// Anything without the prefix is already a label, so it is returned untouched
/// rather than hidden.
String wireLabel(String value, String prefix) {
  if (!value.startsWith(prefix) || value.length == prefix.length) return value;
  final words = value
      .substring(prefix.length)
      .split('_')
      .where((word) => word.isNotEmpty)
      .map((word) => word.toLowerCase())
      .map((word) => word[0].toUpperCase() + word.substring(1));
  final label = words.join(' ');
  return label.isEmpty ? value : label;
}
