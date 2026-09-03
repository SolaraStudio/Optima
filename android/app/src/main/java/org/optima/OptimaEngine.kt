package org.optima

class OptimaEngine {
    private var nativePtr: Long = 0

    init {
        nativePtr = nativeInit()
    }

    fun loadHtml(html: String) {
        nativeLoadHtml(nativePtr, html)
    }

    fun loadCss(css: String) {
        nativeLoadCss(nativePtr, css)
    }

    fun render() {
        nativeRender(nativePtr)
    }

    fun destroy() {
        if (nativePtr != 0L) {
            nativeDestroy(nativePtr)
            nativePtr = 0
        }
    }

    private external fun nativeInit(): Long
    private external fun nativeLoadHtml(ptr: Long, html: String)
    private external fun nativeLoadCss(ptr: Long, css: String)
    private external fun nativeRender(ptr: Long)
    private external fun nativeDestroy(ptr: Long)

    companion object {
        init {
            System.loadLibrary("optima")
        }
    }
}
