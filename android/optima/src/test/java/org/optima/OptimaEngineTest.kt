package org.optima

import org.junit.Assert.assertTrue
import org.junit.Test

class OptimaEngineTest {

    @Test
    fun versionConstantIsPresent() {
        assertTrue(OptimaVersion.VERSION.isNotBlank())
        assertTrue(OptimaVersion.VERSION.startsWith("0."))
    }

    @Test
    fun builderBuildsConfigWithDefaults() {
        val config = OptimaEngineBuilder().config()
        assertTrue(config.javaScriptEnabled)
        assertTrue(config.cacheEnabled)
        assertTrue(config.defaultFontSize > 0)
    }

    @Test
    fun builderOverridesSettings() {
        val config = OptimaEngineBuilder()
            .javaScriptEnabled(false)
            .enableWebGl(true)
            .defaultFontSize(20)
            .config()
        assertTrue(!config.javaScriptEnabled)
        assertTrue(config.enableWebGl)
        assertTrue(config.defaultFontSize == 20)
    }

    @Test
    fun callbackInterfaceIsInvocable() {
        val callback = object : OptimaCallback {
            var finished = false
            override fun onPageStarted(url: String) = Unit
            override fun onPageFinished(url: String) { finished = true }
            override fun onTitleChanged(title: String) = Unit
            override fun onLoadError(url: String, message: String) = Unit
        }
        callback.onPageFinished("about:blank")
        assertTrue(callback.finished)
    }
}
