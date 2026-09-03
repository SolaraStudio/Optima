package org.optima

class OptimaLocalHost(private val engine: OptimaEngine) {

    fun registerAsset(path: String, contentType: String, text: String) {
        engine.registerAssetText(path, contentType, text)
    }

    fun hasAsset(path: String): Boolean {
        return engine.hasLocalAsset(path)
    }

    fun assetCount(): Int {
        return engine.localAssetCount()
    }

    fun loadAsset(path: String) {
        val url = "http://localhost/$path"
        engine.loadUrl(url)
    }
}
