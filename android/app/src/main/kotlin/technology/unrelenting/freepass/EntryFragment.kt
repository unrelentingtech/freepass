package technology.unrelenting.freepass

import android.os.Bundle
import android.support.v4.app.Fragment
import android.text.SpannableStringBuilder
import android.util.Log
import android.view.*
import android.widget.BaseAdapter
import android.widget.RadioGroup
import com.jakewharton.rxbinding.widget.RxRadioGroup
import com.jakewharton.rxbinding.widget.RxTextView

import org.jetbrains.anko.*
import org.jetbrains.anko.support.v4.UI
import java.util.*

class EntryFragment: Fragment() {

	companion object {
		const val ENTRY_NAME = "entry_name"
	}

	var entryName = "New entry"
	var fieldModels = ArrayList<FieldViewModel>()
	val fieldListAdapter = FieldListAdapter(this)

	override fun onCreateOptionsMenu(menu: Menu?, inflater: MenuInflater?) {
		inflater?.inflate(R.menu.entry, menu)
	}

	override fun onOptionsItemSelected(item: MenuItem?): Boolean {
		when (item?.itemId) {
			R.id.add_field -> {
				fieldModels.add(FieldViewModel("New field"))
				fieldListAdapter.notifyDataSetChanged()
			}
		}
		return false
	}

	override fun onCreateView(inflater: LayoutInflater?, container: ViewGroup?, savedInstanceState: Bundle?): View? {
		super.onCreateView(inflater, container, savedInstanceState)
		entryName = arguments.getString(ENTRY_NAME)
		val entry = Vault.getEntry(entryName)
		if (entry != null) {
			entry.fields.forEach {
				fieldModels.add(FieldViewModel(it.key, it.value))
			}
		}

		setHasOptionsMenu(true)
		return UI {
			verticalLayout {
				listView {
					adapter = fieldListAdapter
				}
			}
		}.view
	}

	class FieldListAdapter(val fragment: EntryFragment): BaseAdapter() {
		var list = fragment.fieldModels

		override fun getView(i: Int, v: View?, parent: ViewGroup?): View? {
			if (parent == null) { return null }
			val model = getItem(i)
			return with(parent.context) {
				relativeLayout {
					val nameEditId = 1
					val typeRadioId = 2
					editText {
						id = nameEditId
						text = SpannableStringBuilder(model.field_name.value)
						RxTextView.textChanges(this).subscribe { model.field_name.onNext(it.toString()) }
					}.lparams { width = matchParent }
					val typeRadio = radioGroup {
						id = typeRadioId
						orientation = RadioGroup.HORIZONTAL
						val der = radioButton { text = "Derived" }
						val stor = radioButton { text = "Stored" }
						RxRadioGroup.checked(this).call(
                            if (model.field_type.value == FieldViewModel.FieldType.Derived) der.id else stor.id
						)
						RxRadioGroup.checkedChanges(this).subscribe {
							Log.w("Check", it.toString())
							model.field_type.onNext(if (it == der.id) FieldViewModel.FieldType.Derived else FieldViewModel.FieldType.Stored)
						}
					}.lparams { width = matchParent; below(nameEditId) }
				}
			}
		}

        override fun getItem(position: Int): FieldViewModel {
            return list[position]
        }

		override fun getCount(): Int {
			return list.size
		}

		override fun getItemId(position: Int): Long {
			return position.toLong()
		}
	}
}