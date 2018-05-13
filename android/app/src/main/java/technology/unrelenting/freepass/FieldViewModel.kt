package technology.unrelenting.freepass

import jp.keita.kagurazaka.rxproperty.RxProperty
import java.nio.charset.Charset

class FieldViewModel(name: String, field: Field?) {
	companion object {
		val UTF8 = Charset.forName("UTF-8")
	}

	enum class FieldType {
		Stored, Derived
	}

	val field_name = RxProperty(name)
	val field_type = RxProperty<FieldType?>()
	val field_type_radio = RxProperty(field_type.map {
		when (it) {
			FieldType.Stored -> R.id.fld_type_stored
			else -> R.id.fld_type_derived
		}
	}).let {
		it.subscribe {
			when (it) {
				R.id.fld_type_derived -> field_type.set(FieldType.Derived)
				R.id.fld_type_stored -> field_type.set(FieldType.Stored)
			}
		}
		it
	}
	val derived_counter = RxProperty<Int>(0)
	val derived_site_name = RxProperty<String>()
	val derived_usage = RxProperty<DerivedUsage?>()
	val stored_data = RxProperty<ByteArray?>()
	val stored_data_string = RxProperty<String>(stored_data.map {
		if (it == null) ""
		else try {
			String(it, UTF8)
		} catch (e: Exception) {
			""
		}
	}).let {
		it.subscribe {
			if (it != null && it.isNotEmpty())
				stored_data.set((it as java.lang.String).getBytes("UTF-8"))
		}
		it
	}
	val stored_usage = RxProperty<StoredUsage?>()

	init {
		if (field is Field.Derived) {
			field_type.set(FieldType.Derived)
			derived_counter.set(field.counter)
			derived_site_name.set(field.site_name.orEmpty())
			derived_usage.set(field.usage)
		} else if (field is Field.Stored) {
			field_type.set(FieldType.Stored)
			stored_data.set(field.data)
			stored_usage.set(field.usage)
		}
	}

	constructor(name: String) : this(name, null)
}
