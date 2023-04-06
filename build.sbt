ThisBuild / turbo := true

lazy val root = project
  .in(file("."))
  .settings(commonSettings)
  .settings(
    name := "scala-rust-interoperability-example",
  )
  .aggregate(
    core,
    native,
  )

lazy val native = project
  .in(file("native"))
  .settings(commonSettings)
  .settings(
    name := "scala-rust-interoperability-example-native",
    // baseDirectory = <project_root>/native
    nativeCompile / sourceDirectory := baseDirectory.value,
    nativeBuildTool := com.github.sbt.jni.build.Cargo.make(Nil)
  )
  .enablePlugins(JniNative)

lazy val core = project
  .in(file("core"))
  .settings(commonSettings)
  .settings(
    name := "scala-rust-interoperability-example-core",
    Compile / mainClass := Some("io.github.filippovissani.example.Main"),
    libraryDependencies += "org.scalatest" %% "scalatest" % "3.2.15" % "test",
    sbtJniCoreScope := Compile, // because we use `NativeLoader`, not the `@nativeLoader` macro
    classLoaderLayeringStrategy := ClassLoaderLayeringStrategy.Flat, // because of `turbo := true`
  )
  .dependsOn(native % Runtime)

lazy val commonSettings: List[Def.Setting[_]] = List(
  organization := "io.github.filippovissani",
  scalaVersion := "3.2.2"
)
