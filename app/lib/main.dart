// SPDX-License-Identifier: GPL-3.0-or-later

import 'package:flutter/material.dart';

void main() {
  runApp(const MainApp());
}

class MainApp extends StatelessWidget {
  const MainApp({super.key});

  @override
  Widget build(BuildContext context) {
    return const MaterialApp(
      title: 'Samare',
      home: Scaffold(body: Center(child: Text('Samare'))),
    );
  }
}
