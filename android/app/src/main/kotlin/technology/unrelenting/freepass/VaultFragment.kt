package technology.unrelenting.freepass

import android.app.Fragment
import android.content.Intent
import android.os.Bundle
import android.view.*
import android.widget.ArrayAdapter
import kotlinx.android.synthetic.main.vault.*

class VaultFragment: Fragment() {

	override fun onCreateView(inflater: LayoutInflater?, container: ViewGroup?, savedInstanceState: Bundle?): View? {
		super.onCreateView(inflater, container, savedInstanceState)
		setHasOptionsMenu(true)
		return inflater?.inflate(R.layout.vault, container, false)
	}

	override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
		super.onViewCreated(view, savedInstanceState)
		val entryNames = Vault.entryNames().toTypedArray()
		entries_list.adapter = ArrayAdapter<String>(this.context,
				android.R.layout.simple_list_item_1,
				entryNames)
		entries_list.setOnItemClickListener { adapterView, view, i, l ->
			openEntry(entryNames[i])
		}
	}

	override fun onCreateOptionsMenu(menu: Menu?, inflater: MenuInflater?) {
		inflater?.inflate(R.menu.vault, menu)
	}

	override fun onOptionsItemSelected(item: MenuItem?): Boolean {
		when (item?.itemId) {
			R.id.add_entry -> {
				openEntry("New entry")
			}
		}
		return false
	}

	fun openEntry(name: String) {
		println(name)
		startActivity(Intent(this.context, EntryActivity::class.java)
				.let { it.putExtra(EntryFragment.ENTRY_NAME, name); it })
	}

}