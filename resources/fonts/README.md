# Bundled fonts
#
# Optima uses fontdue for fallback metrics and rustybuzz/text for shaping.
# System/device fonts are enumerated at runtime by AndroidFonts
# (src/android/fonts) and SystemFontHelper.kt.
#
# To bundle a font for consistent rendering across devices, drop a `.ttf`
# or `.otf` file here and register it in src/resources (font loading) so it
# is shipped inside the AAR assets.
#
# No fonts are bundled by default to keep the AAR small.
