package technology.unrelenting.freepass

import com.upokecenter.cbor.CBORObject

data class Entry(var fields: Map<String, Field> = emptyMap()) {
	companion object {
        fun fromCbor(cborObject: CBORObject): Entry {
			val result = mutableMapOf<String, Field>()
			val fields = cborObject["fields"]
			fields.keys.forEach {
				result.put(it.AsString(), Field.fromCbor(fields[it]))
			}
			return Entry(result)
        }
	}
}