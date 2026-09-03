package org.optima

import android.graphics.Typeface
import android.os.Build
import java.io.File
import java.util.HashMap

object SystemFontHelper {
    @JvmStatic
    fun getSystemFonts(): Map<String, String> {
        val map = HashMap<String, String>()
        val fontDirs = arrayOf(
            "/system/fonts",
            "/system/fonts/googlefonts",
            "/product/fonts",
            "/vendor/fonts"
        )

        val extensions = arrayOf(".ttf", ".otf")

        for (dirPath in fontDirs) {
            val dir = File(dirPath)
            if (!dir.exists() || !dir.isDirectory) continue

            dir.listFiles { file ->
                val name = file.name
                extensions.any { name.endsWith(it, ignoreCase = true) }
            }?.forEach { file ->
                val name = file.nameWithoutExtension
                try {
                    val typeface = Typeface.createFromFile(file)
                    val familyName = getFamilyName(typeface) ?: name
                    map[familyName] = file.absolutePath
                } catch (e: Exception) {
                    map[name] = file.absolutePath
                }
            }
        }

        val defaultTypeface = Typeface.DEFAULT
        val defaultFamily = getFamilyName(defaultTypeface) ?: "sans-serif"
        val defaultPath = "/system/fonts/Roboto-Regular.ttf"
        if (File(defaultPath).exists()) {
            map[defaultFamily] = defaultPath
        }

        return map
    }

    private fun getFamilyName(typeface: Typeface): String? {
        return if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.P) {
            // For Android 9+ we use the family property
            typeface.family
        } else {
            // Use reflection for older versions
            try {
                val field = typeface.javaClass.getDeclaredField("familyName")
                field.isAccessible = true
                field.get(typeface) as? String
            } catch (e: Exception) {
                null
            }
        }
    }
}
