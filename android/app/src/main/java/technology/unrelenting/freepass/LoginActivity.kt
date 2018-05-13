package technology.unrelenting.freepass

import android.Manifest
import android.app.Activity
import android.content.Context
import android.content.Intent
import android.content.pm.PackageManager
import android.os.AsyncTask
import android.os.Bundle
import android.widget.Toast
import kotlinx.android.synthetic.main.login.*

class LoginActivity : Activity() {

	inner class OpenTask(private val ctx: Context) : AsyncTask<String, Unit, Unit>() {
		override fun doInBackground(vararg args: String) {
			Vault.open(args[0], args[1], args[2])
			println("Entry names: ${Vault.entryNames()}")
		}

		override fun onPostExecute(result: Unit?) {
			ctx.startActivity(Intent(ctx, VaultActivity::class.java))
		}
	}

	override fun onCreate(savedInstanceState: Bundle?) {
		super.onCreate(savedInstanceState)
		setContentView(R.layout.login)
		lgn_go.setOnClickListener {
			val permission = checkSelfPermission(Manifest.permission.READ_EXTERNAL_STORAGE)
			if (permission == PackageManager.PERMISSION_GRANTED) {
				storageReadAllowed = true
			} else {
				requestPermissions(arrayOf(Manifest.permission.READ_EXTERNAL_STORAGE), 1)
			}
			if (!storageReadAllowed) {
				Toast.makeText(this, "You must allow reading files!", Toast.LENGTH_SHORT).show()
			}
			println("Dir: ${getExternalFilesDir(null).path}")
			OpenTask(this).execute("${getExternalFilesDir(null).path}/testvault", lgn_name.text.toString(), lgn_password.text.toString())
		}
	}

	var storageReadAllowed = false

	override fun onRequestPermissionsResult(requestCode: Int, permissions: Array<out String>, grantResults: IntArray) {
		super.onRequestPermissionsResult(requestCode, permissions, grantResults)
		for ((perm, res) in permissions.zip(grantResults.toList())) {
			if (perm == Manifest.permission.READ_EXTERNAL_STORAGE) {
				storageReadAllowed = res == PackageManager.PERMISSION_GRANTED
			}
		}
	}

}
