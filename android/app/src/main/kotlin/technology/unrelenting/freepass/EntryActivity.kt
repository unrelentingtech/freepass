package technology.unrelenting.freepass

import android.os.Bundle
import android.support.v7.app.AppCompatActivity
import org.jetbrains.anko.frameLayout

class EntryActivity: AppCompatActivity() {
	val rootId = 1001

	override fun onCreate(savedInstanceState: Bundle?) {
		super.onCreate(savedInstanceState)

		if (!Vault.isOpen) {
			finish()
		}

		frameLayout { id = rootId }

		if (savedInstanceState == null) {
			supportFragmentManager.beginTransaction()
					.add(rootId, EntryFragment().let { it.arguments = intent.extras; it })
					.commit()
		}
	}

}