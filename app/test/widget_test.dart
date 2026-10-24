// SPDX-License-Identifier: GPL-3.0-or-later

import 'package:flutter_test/flutter_test.dart';
import 'package:samare/main.dart';

void main() {
  testWidgets('app starts', (tester) async {
    await tester.pumpWidget(const MainApp());
    expect(find.text('Samare'), findsOneWidget);
  });
}
