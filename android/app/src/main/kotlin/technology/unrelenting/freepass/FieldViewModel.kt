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

	val field_type = RxProperty<FieldType?>()
	val field_name = RxProperty<String>()
	val derived_counter = RxProperty<Int>()
	val derived_site_name = RxProperty<String>()
	val derived_usage = RxProperty<DerivedUsage?>()
	val stored_data = RxProperty<ByteArray?>()
	val stored_data_string = RxProperty<String>()
	val stored_usage = RxProperty<StoredUsage?>()

	fun init_stored_data_conversion() {
		var updatingFromSelf = false
		stored_data.subscribe {
			if (!updatingFromSelf && it != null) {
				try {
					stored_data_string.set(String(it, UTF8))
				} catch (e: Exception) {}
			}
		}
		stored_data_string.subscribe {
			updatingFromSelf = true
			stored_data.set(UTF8.encode(it).array())
			updatingFromSelf = false
		}
	}

	init {
		init_stored_data_conversion()
		field_name.set(name)
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

	constructor(name: String) : this(name, null) { }
}
