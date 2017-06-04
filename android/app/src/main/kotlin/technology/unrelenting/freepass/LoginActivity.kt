package technology.unrelenting.freepass

import android.app.Activity
import android.os.Bundle
import kotlinx.android.synthetic.main.login.*

class LoginActivity : Activity() {

	override fun onCreate(savedInstanceState: Bundle?) {
		super.onCreate(savedInstanceState)
		setContentView(R.layout.login)
		lgn_go.setOnClickListener {
			Vault.open("${getExternalFilesDir(null).path}/testvault", lgn_name.text.toString(), lgn_password.text.toString())
			println("${Vault.entryNames()}")
			// TODO
		}
//				setOnClickListener {
//					val permission = ActivityCompat.checkSelfPermission(ctx, Manifest.permission.READ_EXTERNAL_STORAGE)
//					if (permission == PackageManager.PERMISSION_GRANTED) {
//						storageReadAllowed = true
//					} else {
//						ActivityCompat.requestPermissions(this@LoginActivity, arrayOf(Manifest.permission.READ_EXTERNAL_STORAGE), 1)
//					}
//					if (!storageReadAllowed) {
//						ctx.toast("You must allow reading files!")
//					}
//				}
	}

//	var storageReadAllowed = false
//
//	override fun onRequestPermissionsResult(requestCode: Int, permissions: Array<out String>, grantResults: IntArray) {
//		super.onRequestPermissionsResult(requestCode, permissions, grantResults)
//		for ((perm, res) in permissions.zip(grantResults.toList())) {
//			if (perm == Manifest.permission.READ_EXTERNAL_STORAGE) {
//				storageReadAllowed = res == PackageManager.PERMISSION_GRANTED
//			}
//		}
//	}

}
