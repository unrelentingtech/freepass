package technology.unrelenting.freepass

import com.upokecenter.cbor.CBORObject
import com.upokecenter.cbor.CBORType

sealed class DerivedUsage {
	data class Password(val template: String) : DerivedUsage()
	data class Ed25519Key(val usage: String) : DerivedUsage()
	object RawKey : DerivedUsage()
	object Unreadable : DerivedUsage()

	companion object {
		fun fromCbor(cborObject: CBORObject): DerivedUsage {
			return when (cborObject.type) {
				CBORType.TextString -> when (cborObject.AsString()) {
					"RawKey" -> RawKey
					else -> null
				}
				CBORType.Array -> {
					val (variant, argsObj) = cborObject.values.toList()
					when (variant.AsString()) {
						"Password" -> Password(argsObj.AsNullableString() ?: "Maximum")
						"Ed25519Key" -> Ed25519Key(argsObj.AsNullableString() ?: "SSH")
						else -> null
					}
				}
				else -> null
			} ?: Unreadable
		}
	}
}

enum class StoredUsage {
	Unreadable, Text, Password
}

sealed class Field {
	data class Derived(val counter: Int, val site_name: String?, val usage: DerivedUsage) : Field()
	data class Stored(val data: ByteArray, val usage: StoredUsage) : Field()
	object Unreadable : Field()

	companion object {
        fun fromCbor(cborObject: CBORObject): Field {
			val (variant, argsObj) = cborObject.values.toList()
			return when (variant.AsString()) {
				"Derived" -> {
					val counter = argsObj.get("counter").AsInt32()
					val site_name = argsObj.get("site_name").AsNullableString()
					val usage = DerivedUsage.fromCbor(argsObj.get("usage"))
					Derived(counter, site_name, usage)
				}
				"Stored" -> {
					val data = argsObj.get("data").GetByteString()
					val usage = StoredUsage.valueOf(argsObj.get("usage").AsString())
					Stored(data, usage)
				}
				else -> null
			} ?: Unreadable
		}
	}
}