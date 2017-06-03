package technology.unrelenting.freepass

import android.os.Bundle
import android.os.Environment
import android.support.v7.app.AppCompatActivity
import android.util.Log
import org.jetbrains.anko.frameLayout

class VaultActivity: AppCompatActivity() {

	val rootId = 1001

	override fun onCreate(savedInstanceState: Bundle?) {
		super.onCreate(savedInstanceState)

		// XXX TESTING
        Vault.open("${getExternalFilesDir(null).path}/testvault", "Test", "test")

		if (!Vault.isOpen) {
			finish()
		}

		frameLayout { id = rootId }

		if (savedInstanceState == null) {
			supportFragmentManager.beginTransaction()
					.add(rootId, VaultFragment())
					.commit()
		}
	}

}