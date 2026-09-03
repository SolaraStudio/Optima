package org.optima

data class OptimaConfig(
    val userAgent: String = "Optima/0.150.10-dev",
    val javaScriptEnabled: Boolean = true,
    val imagesEnabled: Boolean = true,
    val cssEnabled: Boolean = true,
    val defaultFontSize: Int = 16,
    val defaultFontFamily: String = "sans-serif",
    val enableWebGl: Boolean = false,
    val enableWebRtc: Boolean = false,
    val cacheEnabled: Boolean = true,
    val cacheSizeBytes: Long = 50L * 1024 * 1024,
    val timeoutMillis: Long = 30_000,
    val maxConnections: Int = 6,
    val allowFileProtocol: Boolean = false
)

class OptimaEngineBuilder {
    private var config = OptimaConfig()

    fun userAgent(value: String) = apply { config = config.copy(userAgent = value) }
    fun javaScriptEnabled(value: Boolean) = apply { config = config.copy(javaScriptEnabled = value) }
    fun imagesEnabled(value: Boolean) = apply { config = config.copy(imagesEnabled = value) }
    fun cssEnabled(value: Boolean) = apply { config = config.copy(cssEnabled = value) }
    fun defaultFontSize(value: Int) = apply { config = config.copy(defaultFontSize = value) }
    fun defaultFontFamily(value: String) = apply { config = config.copy(defaultFontFamily = value) }
    fun enableWebGl(value: Boolean) = apply { config = config.copy(enableWebGl = value) }
    fun enableWebRtc(value: Boolean) = apply { config = config.copy(enableWebRtc = value) }
    fun cacheEnabled(value: Boolean) = apply { config = config.copy(cacheEnabled = value) }
    fun cacheSizeBytes(value: Long) = apply { config = config.copy(cacheSizeBytes = value) }
    fun timeoutMillis(value: Long) = apply { config = config.copy(timeoutMillis = value) }
    fun maxConnections(value: Int) = apply { config = config.copy(maxConnections = value) }
    fun allowFileProtocol(value: Boolean) = apply { config = config.copy(allowFileProtocol = value) }

    fun build(): OptimaEngine = OptimaEngine.create()

    fun config(): OptimaConfig = config
}
