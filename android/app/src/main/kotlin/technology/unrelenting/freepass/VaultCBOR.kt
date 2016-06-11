package technology.unrelenting.freepass

// Separate file to prevent JavaCPP from loading Jackson

import com.fasterxml.jackson.databind.DeserializationFeature
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.dataformat.cbor.CBORFactory
import com.fasterxml.jackson.module.kotlin.KotlinModule

val cborFactory = CBORFactory()
val objMapper = ObjectMapper(cborFactory)
		.registerModule(KotlinModule())
		.disable(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES)
		.enable(DeserializationFeature.READ_UNKNOWN_ENUM_VALUES_AS_NULL)

fun Vault.getEntry(name: String): Entry? {
	if (vaultObj == null) {
		return null
	}
	return freepass_vault_get_entry_cbor(vaultObj!!, name).let {
		val arr = ByteArray(it.len())
		it.data().get(arr, 0, it.len())
		val dataval = objMapper.readTree(arr).get(0)
		//val metaval = objMapper.readTree(arr).get(1)
		val data = objMapper.convertValue(dataval, Entry::class.java)
		freepass_free_entry_cbor(it)
		data
	}
}

