package org.optima

class OptimaEngine private constructor(ptr: Long) : AutoCloseable {
    private var nativePtr: Long = ptr

    fun loadHtml(html: String) {
        checkAlive()
        nativeLoadHtml(nativePtr, html)
    }

    fun loadUrl(url: String) {
        checkAlive()
        nativeLoadUrl(nativePtr, url)
    }

    fun loadCss(css: String) {
        checkAlive()
        nativeLoadCss(nativePtr, css)
    }

    fun setViewport(width: Int, height: Int) {
        checkAlive()
        nativeSetViewport(nativePtr, width, height)
    }

    fun render() {
        checkAlive()
        nativeRender(nativePtr)
    }

    fun reload() {
        checkAlive()
        nativeReload(nativePtr)
    }

    fun goBack() {
        checkAlive()
        nativeGoBack(nativePtr)
    }

    fun goForward() {
        checkAlive()
        nativeGoForward(nativePtr)
    }

    fun tick() {
        checkAlive()
        nativeTick(nativePtr)
    }

    fun callNativeHandler(name: String, arg: String): String {
        checkAlive()
        return nativeCallNativeHandler(nativePtr, name, arg)
    }

    fun hasNativeHandler(name: String): Boolean {
        checkAlive()
        return nativeHasNativeHandler(nativePtr, name) != 0
    }

    fun registerAssetText(path: String, contentType: String, text: String) {
        checkAlive()
        nativeRegisterAssetText(nativePtr, path, contentType, text)
    }

    fun hasLocalAsset(path: String): Boolean {
        checkAlive()
        return nativeHasLocalAsset(nativePtr, path) != 0
    }

    fun localAssetCount(): Int {
        checkAlive()
        return nativeLocalAssetCount(nativePtr)
    }

    fun handlerNames(): List<String> {
        checkAlive()
        val csv = nativeHandlerNames(nativePtr)
        return if (csv.isEmpty()) emptyList() else csv.split(",")
    }

    override fun close() {
        if (nativePtr != 0L) {
            nativeDestroy(nativePtr)
            nativePtr = 0
        }
    }

    private fun checkAlive() {
        if (nativePtr == 0L) {
            throw OptimaException("OptimaEngine has been destroyed")
        }
    }

    private external fun nativeLoadHtml(ptr: Long, html: String)
    private external fun nativeLoadUrl(ptr: Long, url: String)
    private external fun nativeLoadCss(ptr: Long, css: String)
    private external fun nativeSetViewport(ptr: Long, width: Int, height: Int)
    private external fun nativeRender(ptr: Long)
    private external fun nativeReload(ptr: Long)
    private external fun nativeGoBack(ptr: Long)
    private external fun nativeGoForward(ptr: Long)
    private external fun nativeTick(ptr: Long)
    private external fun nativeCallNativeHandler(ptr: Long, name: String, arg: String): String
    private external fun nativeHasNativeHandler(ptr: Long, name: String): Int
    private external fun nativeRegisterAssetText(ptr: Long, path: String, contentType: String, text: String)
    private external fun nativeHasLocalAsset(ptr: Long, path: String): Int
    private external fun nativeLocalAssetCount(ptr: Long): Int
    private external fun nativeHandlerNames(ptr: Long): String
    private external fun nativeDestroy(ptr: Long)

    companion object {
        init {
            System.loadLibrary("optima")
        }

        fun create(): OptimaEngine {
            val ptr = nativeInit()
            if (ptr == 0L) {
                throw OptimaException("Failed to initialize native Optima engine")
            }
            return OptimaEngine(ptr)
        }

        private external fun nativeInit(): Long
    }
}
