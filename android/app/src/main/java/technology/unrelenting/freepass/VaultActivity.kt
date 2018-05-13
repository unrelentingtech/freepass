package technology.unrelenting.freepass

import android.app.Activity
import android.os.Bundle

class VaultActivity: Activity() {

	override fun onCreate(savedInstanceState: Bundle?) {
		super.onCreate(savedInstanceState)

		// XXX TESTING
        //Vault.open("${getExternalFilesDir(null).path}/testvault", "Test", "test")

		if (!Vault.isOpen) {
			finish()
		}

		setContentView(R.layout.vault_wrapper)
	}

}