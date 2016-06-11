package technology.unrelenting.freepass

import android.os.Bundle
import android.os.Environment
import android.support.v7.app.AppCompatActivity
import org.jetbrains.anko.frameLayout

class VaultActivity: AppCompatActivity() {

	val rootId = 1001

	override fun onCreate(savedInstanceState: Bundle?) {
		super.onCreate(savedInstanceState)
		frameLayout { id = rootId }

		// XXX TESTING
		Vault.open("${Environment.getExternalStorageDirectory().path}/testvault", "Test", "test")

		if (!Vault.isOpen) {
			finish()
		}

		if (savedInstanceState == null) {
			supportFragmentManager.beginTransaction()
					.add(rootId, VaultFragment())
					.commit()
		}
	}

}