package org.optima

import android.graphics.Typeface
import android.os.Build
import androidx.annotation.RequiresApi
import java.io.File
import java.util.HashMap

object SystemFontHelper {

    @JvmStatic
    fun getSystemFonts(): Map<String, String> {
        val map = HashMap<String, String>()
        
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
            return getFontsUsingSystemApi()
        }
        
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

            val files = dir.listFiles { file ->
                extensions.any { file.name.endsWith(it, ignoreCase = true) }
            } ?: continue

            for (file in files) {
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

        val defaultFamily = getFamilyName(Typeface.DEFAULT) ?: "sans-serif"
        val defaultPath = "/system/fonts/Roboto-Regular.ttf"
        if (!map.containsKey(defaultFamily) && File(defaultPath).exists()) {
            map[defaultFamily] = defaultPath
        }

        return map
    }

    @RequiresApi(Build.VERSION_CODES.Q)
    private fun getFontsUsingSystemApi(): Map<String, String> {
        val map = HashMap<String, String>()
        try {
            val systemFonts = android.graphics.fonts.SystemFonts.getAvailableFonts()
            for (font in systemFonts) {
                val file = font.file
                val name = file.nameWithoutExtension
                
                val typeface = Typeface.createFromFile(file)
                val familyName = getFamilyName(typeface) ?: name
                
                map[familyName] = file.absolutePath
            }
        } catch (e: Exception) {
        }
        return map
    }

    private fun getFamilyName(typeface: Typeface): String? {
        return try {
            val field = Typeface::class.java.getDeclaredField("familyName")
            field.isAccessible = true
            field.get(typeface) as? String
        } catch (e: Exception) {
            null
        }
    }
}
