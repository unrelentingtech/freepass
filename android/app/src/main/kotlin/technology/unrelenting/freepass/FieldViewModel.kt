package technology.unrelenting.freepass

import rx.lang.kotlin.ReplaySubject
import java.nio.charset.Charset

class FieldViewModel(name: String, field: Field?) {
	companion object {
		val UTF8 = Charset.forName("UTF-8")
	}

	enum class FieldType {
		Stored, Derived
	}

	val field_type = ReplaySubject<FieldType?>()
	val field_name = ReplaySubject<String>()
	val derived_counter = ReplaySubject<Int>()
	val derived_site_name = ReplaySubject<String>()
	val derived_usage = ReplaySubject<DerivedUsage?>()
	val stored_data = ReplaySubject<ByteArray?>()
	val stored_data_string = ReplaySubject<String>()
	val stored_usage = ReplaySubject<StoredUsage?>()

	fun init_stored_data_conversion() {
		var updatingFromSelf = false
		stored_data.subscribe {
			if (!updatingFromSelf && it != null) {
				try {
					stored_data_string.onNext(String(it, UTF8))
				} catch (e: Exception) {}
			}
		}
		stored_data_string.subscribe {
			updatingFromSelf = true
			stored_data.onNext(UTF8.encode(it).array())
			updatingFromSelf = false
		}
	}

	init {
		init_stored_data_conversion()
		field_name.onNext(name)
		if (field is Field.Derived) {
			field_type.onNext(FieldType.Derived)
			derived_counter.onNext(field.counter)
			derived_site_name.onNext(field.site_name)
			derived_usage.onNext(field.usage)
		} else if (field is Field.Stored) {
			field_type.onNext(FieldType.Stored)
			stored_data.onNext(field.data)
			stored_usage.onNext(field.usage)
		}
	}

	constructor(name: String) : this(name, null) { }
}
