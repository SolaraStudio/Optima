# Gradle build notes
#
# The Android library module lives in `android/` (root project) with the
# `:app` library module. The Rust `.so` files are copied into jniLibs by the
# `copyRustLibs` / `buildRustAll` gradle tasks and packaged into the AAR.
#
# Key tasks:
#   cd android
#   ./gradlew assembleRelease     # build the AAR
#   ./gradlew publish             # publish to GitHub Packages (needs GITHUB_TOKEN)
#
# Output:
#   android/app/build/outputs/aar/optima-release.aar
#
# Properties: see android/gradle.properties (AndroidX enabled, Jetifier on).
