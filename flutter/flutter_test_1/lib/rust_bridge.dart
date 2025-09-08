import 'dart:ffi';
import 'package:path/path.dart' as path;
import 'dart:io';
import 'package:ffi/ffi.dart';

//definiere Signaturen

String getRustDllPath() {
  // Ordner dieser Dart-Datei
  final scriptDir = path.dirname(Platform.script.toFilePath());

  // Relativer Pfad zur DLL von hier
  final dllRelativePath = path.join(scriptDir, '../../tutorial2/target/release/tutorial2.dll');
  print('DLL Path: $dllRelativePath');
  
  // Print directory contents
  final directory = Directory(path.dirname(dllRelativePath));
  if (directory.existsSync()) {
    print('Directory contents:');
    directory.listSync().forEach((entity) {
      print('  - ${path.basename(entity.path)}');
    });
  } else {
    print('Directory does not exist: ${directory.path}');
  }

  return dllRelativePath;
}

typedef ReplyGreetingNative = Pointer<Utf8> Function(Pointer<Utf8>);
typedef ReplyGreetingDart = Pointer<Utf8> Function(Pointer<Utf8>);

typedef FreeRustStringNative = Void Function(Pointer<Utf8>);
typedef FreeRustStringDart = void Function(Pointer<Utf8>);

//loading DLL
final DynamicLibrary dylib = Platform.isWindows
    ? DynamicLibrary.open(getRustDllPath())
    : DynamicLibrary.process();

// get function references
final ReplyGreetingDart replyGreeting = dylib
    .lookup<NativeFunction<ReplyGreetingNative>>('reply_greeting')
    .asFunction();

final FreeRustStringDart freeRustString = dylib
    .lookup<NativeFunction<FreeRustStringNative>>('free_rust_string')
    .asFunction();

//wrapper for Dart
String getGreeting(String name) {
  final namePtr = name.toNativeUtf8();
  final resultPtr = replyGreeting(namePtr);
  calloc.free(namePtr);
  final result = resultPtr.toDartString();
  freeRustString(resultPtr);
  return result;
}