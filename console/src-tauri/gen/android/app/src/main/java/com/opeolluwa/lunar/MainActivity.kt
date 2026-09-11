package com.opeolluwa.lunar

import android.os.Bundle
import android.os.Handler
import android.os.Looper
import android.view.View
import android.webkit.WebView
import androidx.core.splashscreen.SplashScreen.Companion.installSplashScreen
import androidx.core.view.ViewCompat
import androidx.core.view.WindowInsetsCompat

class MainActivity : TauriActivity() {
    private var webViewReady = false
    private var webViewRef: WebView? = null
    private val readyTimeoutMs = 10_000L
    private val startedAt = System.currentTimeMillis()

    override fun onCreate(savedInstanceState: Bundle?) {
        val splash = installSplashScreen()
        splash.setKeepOnScreenCondition { !webViewReady }
        super.onCreate(savedInstanceState)
        applyImeInsets()
    }

    private fun applyImeInsets() {
        val content = findViewById<View>(android.R.id.content)
        ViewCompat.setOnApplyWindowInsetsListener(content) { view, windowInsets ->
            val imeBottom =
                if (windowInsets.isVisible(WindowInsetsCompat.Type.ime())) {
                    windowInsets.getInsets(WindowInsetsCompat.Type.ime()).bottom
                } else {
                    0
                }
            view.setPadding(0, 0, 0, imeBottom)
            windowInsets
        }
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