package io.github.filippovissani.example

import org.scalatest.funsuite.AnyFunSuite
import org.scalatest.matchers.should.Matchers.shouldBe

import scala.language.postfixOps

class DividerTest extends AnyFunSuite {
  test("100 / 10 should be 10") {
    val numerator = 100
    val denominator = 10
    val divider = new Divider(numerator)
    val fraction = divider.divideBy(denominator)
    fraction shouldBe numerator / denominator
  }
}
