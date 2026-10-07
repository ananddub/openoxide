ROOT := ..
# adb-visible emulator serial; override with `make -C mobile dtd DEVICE=<id>` for a
# physical device (see `flutter devices`).
DEVICE ?= emulator-5554

.PHONY: gen analyze test apk dtd fonts

# Generated Dart stubs land in lib/src/gen and are committed: the build must not
# depend on buf/protoc being installed. Regenerate after proto/ changes only.
gen:
	cd $(ROOT) && buf generate --path proto/openoxide/api/v1 --template mobile/buf.gen.dart.yaml

# Rebuild the bundled Inter + JetBrains Mono subsets. Committed output: an
# ordinary build needs neither python nor network, and this is only for changing
# the unicode ranges.
fonts:
	sh tool/build-fonts.sh

analyze:
	flutter analyze

test:
	flutter test

apk:
	flutter build apk --debug

# Run on the emulator with the Dart Tooling Daemon address printed, which is what
# the `dart` MCP server (hot_reload / get_runtime_errors / widget_inspector) attaches to.
dtd:
	flutter run -d $(DEVICE) --print-dtd --dart-define=PANEL_URL=10.0.2.2 --dart-define=DEV_ALLOW_ANY_CERT=true
