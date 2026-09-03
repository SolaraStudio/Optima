# Build output artifacts
#
# This folder documents the distributable artifacts produced by Optima builds:
#
#   Android AAR (webview library)
#     android/app/build/outputs/aar/optima-release.aar
#
#   Native library (Android, per ABI, packaged inside the AAR)
#     android/app/src/main/jniLibs/<abi>/liboptima.so
#
#   CI artifacts
#     Published to GitHub Packages as org.optima:optima:maven (see
#     .github/workflows/publish.yml).
