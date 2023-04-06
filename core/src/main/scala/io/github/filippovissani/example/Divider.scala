package io.github.filippovissani.example

import com.github.sbt.jni.syntax.NativeLoader

class Divider(val numerator: Int) extends NativeLoader("divider") {
  @native def divideBy(denominator: Int): Int // implemented in libdivider.so
}
