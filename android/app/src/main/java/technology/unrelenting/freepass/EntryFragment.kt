package technology.unrelenting.freepass

import android.app.Activity
import android.app.Fragment
import android.databinding.DataBindingUtil
import android.os.Bundle
import android.view.*
import android.widget.BaseAdapter
//import com.jakewharton.rxbinding2.widget.*
import kotlinx.android.synthetic.main.entry.*
import technology.unrelenting.freepass.databinding.FieldBinding

import java.util.*

class EntryFragment: Fragment() {

	companion object {
		const val ENTRY_NAME = "entry_name"
	}

	var entryName = "New entry"
	var fieldModels = ArrayList<FieldViewModel>()
	val fieldListAdapter = FieldListAdapter(this)

	override fun onCreateView(inflater: LayoutInflater?, container: ViewGroup?, savedInstanceState: Bundle?): View? {
		super.onCreateView(inflater, container, savedInstanceState)
		entryName = arguments.getString(ENTRY_NAME)
		val entryCode = Vault.getEntry(entryName)
		if (entryCode != null) {
			val entry = Entry.fromCbor(entryCode!!)
			entry.fields.forEach {
				fieldModels.add(FieldViewModel(it.key, it.value))
			}
		}
		setHasOptionsMenu(true)
		return inflater?.inflate(R.layout.entry, container, false)
	}

	override fun onViewCreated(view: View?, savedInstanceState: Bundle?) {
		super.onViewCreated(view, savedInstanceState)
		fields_list.adapter = fieldListAdapter
	}

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

	class FieldListAdapter(val fragment: EntryFragment): BaseAdapter() {
		var list = fragment.fieldModels
		var inflater: LayoutInflater? = null

		override fun getView(i: Int, v: View?, parent: ViewGroup?): View? {
			if (parent == null) return null
            inflater = inflater ?: (parent.context as? Activity)?.layoutInflater
			val binding = DataBindingUtil.getBinding<FieldBinding>(v!!)
				?: DataBindingUtil.inflate<FieldBinding>(inflater!!, R.layout.field, parent, false)
			binding.vm = getItem(i)
			return binding.root
//					val typeRadio = radioGroup {
//						orientation = RadioGroup.HORIZONTAL
//						val der = radioButton {  text = "Derived" }
//						val stor = radioButton {  text = "Stored" }
//						Log.w("WTF", "WAT")
//						check(if (model.field_type.get() == FieldViewModel.FieldType.Derived) der.id else stor.id)
//						checkedChanges().subscribe() {
//							Log.w("Check", it.toString())
//							model.field_type.set(if (it == der.id) FieldViewModel.FieldType.Derived else FieldViewModel.FieldType.Stored)
//						}
//					}.lparams { width = matchParent; below(nameEdit) }
//					val counterLabel = textView {
//						text = "Counter"
//					}.lparams { below(typeRadio) }
//					val counterEdit = editText {
//						inputType = InputType.TYPE_CLASS_NUMBER or InputType.TYPE_NUMBER_FLAG_DECIMAL
//						filters = arrayOf(NumberLimitsInputFilter(0, 4294967295))
//					}.lparams { below(typeRadio); rightOf(counterLabel) }
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