package technology.unrelenting.freepass

import android.app.Activity
import android.os.Bundle

class EntryActivity: Activity() {

	override fun onCreate(savedInstanceState: Bundle?) {
		super.onCreate(savedInstanceState)

		if (!Vault.isOpen) {
			finish()
		}

		setContentView(R.layout.entry_wrapper)
		if (savedInstanceState == null) {
			fragmentManager.beginTransaction()
					.add(R.id.entry_wrapper_l, EntryFragment().let { it.arguments = intent.extras; it })
					.commit()
		}
	}

}