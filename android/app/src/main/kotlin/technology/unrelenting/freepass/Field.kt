package technology.unrelenting.freepass

import com.fasterxml.jackson.annotation.JsonCreator
import com.fasterxml.jackson.annotation.JsonProperty

sealed class DerivedUsage {
	class Password(val template: String) : DerivedUsage()
	class Ed25519Key(val usage: String) : DerivedUsage()
	object RawKey : DerivedUsage()
	object Unreadable : DerivedUsage()

	companion object {
		@JsonCreator @JvmStatic fun create(@JsonProperty("variant") variant: String,
										   @JsonProperty("fields") fields: Array<String>): DerivedUsage {
			return when (variant) {
				"Password" -> fields.getOrNull(0)?.let { Password(it) }
				"Ed25519Key" -> fields.getOrNull(0)?.let { Ed25519Key(it) }
				"RawKey" -> RawKey
				else -> null
			} ?: Unreadable
		}
	}
}

enum class StoredUsage {
	Unreadable, Text, Password
}

sealed class Field {
	class Derived(val counter: Int, val site_name: String?, val usage: DerivedUsage) : Field()
	class Stored(val data: ByteArray, val usage: StoredUsage) : Field()
	object Unreadable : Field()

	companion object {
		@JsonCreator @JvmStatic fun create(@JsonProperty("variant") variant: String,
										   @JsonProperty("fields") fields: Array<Any>): Field {
			return when (variant) {
				"Derived" -> {
					val counter = fields.getOrNull(0) as? Int
					val site_name = fields.getOrNull(1) as? String
					val usage = objMapper.convertValue(fields.getOrNull(2), DerivedUsage::class.java)
					if (counter != null && usage != null) Derived(counter, site_name, usage) else null
				}
				"Stored" -> {
					val data = fields.getOrNull(0) as? ByteArray
					val usage = objMapper.convertValue(fields.getOrNull(1), StoredUsage::class.java) ?: StoredUsage.Unreadable
					if (data != null) Stored(data, usage) else null
				}
				else -> null
			} ?: Unreadable
		}
	}
}