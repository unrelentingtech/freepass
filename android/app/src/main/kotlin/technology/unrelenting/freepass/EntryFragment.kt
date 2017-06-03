package technology.unrelenting.freepass

import android.os.Bundle
import android.support.v4.app.Fragment
import android.text.InputType
import android.text.SpannableStringBuilder
import android.util.Log
import android.view.*
import android.widget.BaseAdapter
import android.widget.RadioButton
import android.widget.RadioGroup
import com.jakewharton.rxbinding2.widget.*

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
			if (parent == null) return null
			if (v != null) return v
			val model = getItem(i)
			return with(parent.context) {
				relativeLayout {
					val nameEdit = editText {
						text = SpannableStringBuilder(model.field_name.get())
						textChanges().subscribe { model.field_name.set(it.toString()) }
					}.lparams { width = matchParent }
					val typeRadio = radioGroup {
						orientation = RadioGroup.HORIZONTAL
						val der = radioButton {  text = "Derived" }
						val stor = radioButton {  text = "Stored" }
						Log.w("WTF", "WAT")
						check(if (model.field_type.get() == FieldViewModel.FieldType.Derived) der.id else stor.id)
						checkedChanges().subscribe() {
							Log.w("Check", it.toString())
							model.field_type.set(if (it == der.id) FieldViewModel.FieldType.Derived else FieldViewModel.FieldType.Stored)
						}
					}.lparams { width = matchParent; below(nameEdit) }
					val counterLabel = textView {
						text = "Counter"
					}.lparams { below(typeRadio) }
					val counterEdit = editText {
						inputType = InputType.TYPE_CLASS_NUMBER or InputType.TYPE_NUMBER_FLAG_DECIMAL
						filters = arrayOf(NumberLimitsInputFilter(0, 4294967295))
					}.lparams { below(typeRadio); rightOf(counterLabel) }
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