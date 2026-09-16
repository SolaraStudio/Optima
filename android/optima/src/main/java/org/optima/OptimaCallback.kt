package org.optima

interface OptimaCallback {
    fun onPageStarted(url: String)
    fun onPageFinished(url: String)
    fun onTitleChanged(title: String)
    fun onLoadError(url: String, message: String)
}
