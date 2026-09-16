package org.optima

import android.content.Context
import android.opengl.GLSurfaceView
import android.util.AttributeSet
import android.view.ViewGroup
import android.widget.FrameLayout

class OptimaWebView @JvmOverloads constructor(
    context: Context,
    attrs: AttributeSet? = null
) : FrameLayout(context, attrs) {

    private var engine: OptimaEngine? = null
    private val surfaceView: GLSurfaceView

    init {
        surfaceView = GLSurfaceView(context)
        surfaceView.setEGLContextClientVersion(2)
        addView(surfaceView, ViewGroup.LayoutParams(
            ViewGroup.LayoutParams.MATCH_PARENT,
            ViewGroup.LayoutParams.MATCH_PARENT
        ))
    }

    fun initialize(callback: OptimaCallback? = null) {
        if (engine != null) return
        engine = OptimaEngine.create()
    }

    fun loadHtml(html: String) {
        requireEngine().loadHtml(html)
    }

    fun loadUrl(url: String) {
        requireEngine().loadUrl(url)
    }

    fun loadCss(css: String) {
        requireEngine().loadCss(css)
    }

    fun reload() {
        requireEngine().reload()
    }

    fun goBack() {
        requireEngine().goBack()
    }

    fun goForward() {
        requireEngine().goForward()
    }

    fun renderFrame() {
        requireEngine().tick()
        requireEngine().render()
    }

    override fun onSizeChanged(w: Int, h: Int, oldw: Int, oldh: Int) {
        super.onSizeChanged(w, h, oldw, oldh)
        engine?.setViewport(w, h)
    }

    fun destroy() {
        engine?.close()
        engine = null
    }

    private fun requireEngine(): OptimaEngine =
        engine ?: throw OptimaException("OptimaWebView has not been initialized")
}
