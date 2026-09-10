package com.opeolluwa.lunar

import android.os.Bundle
import android.os.Looper
import android.os.Handler
import android.view.WindowManager
import android.content.Intent
import androidx.appcompat.app.AppCompatActivity
import android.view.animation.Animation;
import android.view.animation.AnimationUtils;
import android.widget.ImageView;

class SplashScreen : AppCompatActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    super.onCreate(savedInstanceState)
    setContentView(R.layout.activity_splash_screen)

    val imageView: ImageView = findViewById(R.id.loading_icon)
    val rotateAnimation: Animation = AnimationUtils.loadAnimation(this, R.anim.rotate)
    imageView.startAnimation(rotateAnimation)
    
    Handler(Looper.getMainLooper()).postDelayed({
      val intent = Intent(this, MainActivity::class.java)
      startActivity(intent)
      finish()
    }, 2000)
  }
}