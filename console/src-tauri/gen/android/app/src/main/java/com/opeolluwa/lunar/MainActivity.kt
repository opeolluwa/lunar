package com.opeolluwa.lunar

import android.os.Bundle
import android.os.Handler
import android.os.Looper
import android.webkit.WebView
import androidx.core.splashscreen.SplashScreen.Companion.installSplashScreen

class MainActivity : TauriActivity() {
    private var webViewReady = false
    private var webViewRef: WebView? = null
    private val readyTimeoutMs = 10_000L
    private val startedAt = System.currentTimeMillis()

    override fun onCreate(savedInstanceState: Bundle?) {
        val splash = installSplashScreen()
        splash.setKeepOnScreenCondition { !webViewReady }
        super.onCreate(savedInstanceState)
    }

    override fun onWebViewCreate(webView: WebView) {
        super.onWebViewCreate(webView)
        webViewRef = webView
        pollWebViewReady()
    }

    private fun pollWebViewReady() {
        val webView = webViewRef ?: return
        if (System.currentTimeMillis() - startedAt >= readyTimeoutMs || webView.progress >= 100) {
            webViewReady = true
            return
        }
        Handler(Looper.getMainLooper()).postDelayed({ pollWebViewReady() }, 100)
    }
}