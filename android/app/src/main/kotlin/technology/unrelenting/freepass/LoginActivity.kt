package technology.unrelenting.freepass

import android.Manifest
import android.content.pm.PackageManager
import android.os.Bundle
import android.os.Environment
import android.support.v4.app.ActivityCompat
import android.support.v7.app.AppCompatActivity
import android.text.Editable
import android.text.InputType

import org.jetbrains.anko.*

class LoginActivity : AppCompatActivity() {

	override fun onCreate(savedInstanceState: Bundle?) {
		super.onCreate(savedInstanceState)
		Vault.freepass_init()
		verticalLayout {
			padding = dip(30)
			val name = editText {
				hint = "Name"
				text = Editable.Factory.getInstance().newEditable("Test")
				textSize = 24f
			}
			val password = editText {
				hint = "Password"
				textSize = 24f
				text = Editable.Factory.getInstance().newEditable("test")
				inputType = InputType.TYPE_CLASS_TEXT or InputType.TYPE_TEXT_VARIATION_PASSWORD
			}
			button("Login") {
				textSize = 26f
				onClick {
					val permission = ActivityCompat.checkSelfPermission(ctx, Manifest.permission.READ_EXTERNAL_STORAGE)
					if (permission == PackageManager.PERMISSION_GRANTED) {
						storageReadAllowed = true
					} else {
						ActivityCompat.requestPermissions(this@LoginActivity, arrayOf(Manifest.permission.READ_EXTERNAL_STORAGE), 1)
					}
					if (!storageReadAllowed) {
						ctx.toast("You must allow reading files!")
					} else {
						async() {
							try {
								Vault.open(ctx.cacheDir, "${Environment.getExternalStorageDirectory().path}/testvault", name.text.toString(), password.text.toString())
							} catch (e: Exception) {
								uiThread {
									ctx.toast("Error: ${e.message}")
									e.printStackTrace()
								}
							}
							uiThread {
								ctx.toast(if (Vault.isOpen) "Opened" else "Failed")
							}
						}
					}
				}
			}
		}
	}

	var storageReadAllowed = false

	override fun onRequestPermissionsResult(requestCode: Int, permissions: Array<out String>, grantResults: IntArray) {
		super.onRequestPermissionsResult(requestCode, permissions, grantResults)
		for ((perm, res) in permissions.zip(grantResults.toList())) {
			if (perm.equals(Manifest.permission.READ_EXTERNAL_STORAGE)) {
				storageReadAllowed = res == PackageManager.PERMISSION_GRANTED
			}
		}
	}

}
