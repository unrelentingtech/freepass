package technology.unrelenting.freepass

import android.os.Bundle
import android.support.v4.app.Fragment
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup

import org.jetbrains.anko.*
import org.jetbrains.anko.support.v4.UI

class EntryFragment: Fragment() {

	companion object {
		const val ENTRY_NAME = "entry_name"
	}

	override fun onCreateView(inflater: LayoutInflater?, container: ViewGroup?, savedInstanceState: Bundle?): View? {
		super.onCreateView(inflater, container, savedInstanceState)
		setHasOptionsMenu(true)
		return UI {
			verticalLayout {
				textView {
					text = arguments.getString(ENTRY_NAME)
				}
			}
		}.view
	}

}