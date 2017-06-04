package technology.unrelenting.freepass

// Separate file to prevent JavaCPP from loading CBOR

import com.upokecenter.cbor.CBORObject

fun Vault.getEntry(name: String): CBORObject? {
	if (vaultObj == null) {
		return null
	}
	return freepass_vault_get_entry_cbor(vaultObj!!, name).let {
		if (it.len() < 1) return null
		val arr = ByteArray(it.len())
		it.data().get(arr, 0, it.len())
		val objs = CBORObject.DecodeFromBytes(arr).values.take(2)
		val dataval = objs[0]
		//val metaval = objs[1]
		freepass_free_entry_cbor(it)
		dataval
	}
}

fun CBORObject.AsNullableString(): String? {
	return if (isNull) null else AsString()
}
