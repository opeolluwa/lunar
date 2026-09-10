package com.opeolluwa.lunar

import android.content.Intent
import android.os.Bundle

class MainActivity : TauriActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val intent = Intent(this, SplashScreen::class.java)
        startActivity(intent)
    }
}