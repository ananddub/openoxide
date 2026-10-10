import 'package:flutter/material.dart';
import 'package:mobile/src/app/openoxide_app.dart';
import 'package:mobile/src/core/di/deps.dart';
import 'package:mobile/src/core/tokens/motion.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  Motion.install();
  final deps = await Deps.live();
  runApp(OpenoxideApp(deps: deps));
}
