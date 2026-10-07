/// Counts the variables in the `KEY=VALUE` blob the panel stores on a project
/// or environment.
///
/// The field is one newline-separated string, so its length is a character
/// count, not a number of variables. These are the same rules the web editor's
/// parser uses — blank lines, `#`/`;` comments, a line without `=` and an empty
/// key are not variables — so both clients show the same number for one project.
int envVariableCount(String blob) {
  var count = 0;
  for (final rawLine in blob.split('\n')) {
    final line = rawLine.trim();
    if (line.isEmpty || line.startsWith('#') || line.startsWith(';')) continue;
    final equals = line.indexOf('=');
    if (equals == -1) continue;
    final key = line
        .substring(0, equals)
        .replaceFirst(RegExp(r'^export\s+'), '')
        .trim();
    if (key.isEmpty) continue;
    count++;
  }
  return count;
}
