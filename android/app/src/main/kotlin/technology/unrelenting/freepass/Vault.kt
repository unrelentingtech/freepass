package technology.unrelenting.freepass

import org.bytedeco.javacpp.*
import org.bytedeco.javacpp.annotation.*
import java.io.File

class VaultException(reason: String) : Exception(reason) {}

@Platform(library = "jniVault",
          cinclude = arrayOf("freepass_capi.h"))
object Vault {
	var masterKey: Pointer? = null
	var vaultObj: Pointer? = null
	var entriesKey: Pointer? = null
	var filePath: String? = null
	var isOpen = false

	init {
		//Loader.load() // javacpp's loader tries to compute the full path inside the jar for some reason
		                // and ends up with .../arm/... instead of .../armeabi-v7a/... or .../arm64-v7a/...
		System.loadLibrary("jniVault")
		freepass_init()
	}

	fun open(path: String, username: String, password: String) {
		if (isOpen) {
			throw VaultException("Already open")
		}
		masterKey = rusterpassword_gen_master_key(password, username)
		if (masterKey == null || masterKey!!.isNull) {
			throw VaultException("Got null master key")
		}
		val outerKey = freepass_gen_outer_key(masterKey!!)
		val entriesKey = freepass_gen_entries_key(masterKey!!)

		val file = File(path)
		vaultObj = if (file.exists()) {
			if (!file.canRead()) {
				throw VaultException("Not allowed to read file ${file.absolutePath}")
			}
			freepass_open_vault(file.absolutePath, entriesKey, outerKey)
		} else {
			freepass_new_vault(entriesKey, outerKey)
		}
		freepass_free_outer_key(outerKey)

		if (vaultObj == null || vaultObj!!.isNull) {
			rusterpassword_free_master_key(masterKey!!)
			freepass_free_entries_key(entriesKey)
			throw VaultException("Got null vault")
		}

		filePath = file.absolutePath
		isOpen = true
	}

	fun entryNames(): List<String> {
		if (vaultObj == null) {
			return listOf()
		}
		val iter = freepass_vault_get_entry_names_iterator(vaultObj!!)
		var curr: BytePointer? = freepass_entry_names_iterator_next(iter)
		val result = mutableListOf<String>()
		while (curr != null && !curr.isNull) {
			result.add(curr.string)
			freepass_free_entry_name(curr)
			curr = freepass_entry_names_iterator_next(iter)
		}
		freepass_free_entry_names_iterator(iter)
		return result.toList()
	}

	fun close() {
		if (masterKey != null) {
			rusterpassword_free_master_key(masterKey!!)
		}
		masterKey = null
		if (entriesKey != null) {
			freepass_free_entries_key(entriesKey!!)
		}
		entriesKey = null
		if (vaultObj != null) {
			freepass_close_vault(vaultObj!!)
		}
		vaultObj = null
		isOpen = false
	}

	@JvmStatic external fun rusterpassword_gen_master_key(username: String, password: String): Pointer
	@JvmStatic external fun rusterpassword_free_master_key(@Cast("secstr_t*") master_key: Pointer): Unit

	@JvmStatic external fun freepass_init(): Unit

	@JvmStatic external fun freepass_gen_outer_key(@Cast("secstr_t*") master_key: Pointer): Pointer
	@JvmStatic external fun freepass_gen_entries_key(@Cast("secstr_t*") master_key: Pointer): Pointer
	@JvmStatic external fun freepass_free_outer_key(@Cast("secstr_t*") outer_key: Pointer): Unit
	@JvmStatic external fun freepass_free_entries_key(@Cast("secstr_t*") entries_key: Pointer): Unit

	@JvmStatic external fun freepass_open_vault(path: String, @Cast("secstr_t*") entries_key: Pointer, @Cast("secstr_t*") outer_key: Pointer): Pointer
	@JvmStatic external fun freepass_new_vault(@Cast("secstr_t*") entries_key: Pointer, @Cast("secstr_t*") outer_key: Pointer): Pointer
	@JvmStatic external fun freepass_close_vault(@Cast("vault_t*") vault: Pointer): Unit

	@JvmStatic external fun freepass_vault_get_entry_names_iterator(@Cast("vault_t*") vault: Pointer): Pointer
	@JvmStatic external @Cast("signed char*") fun freepass_entry_names_iterator_next(@Cast("string_iter_t*") iter: Pointer): BytePointer
	@JvmStatic external fun freepass_free_entry_name(@Cast("char*") name: BytePointer): Unit
	@JvmStatic external fun freepass_free_entry_names_iterator(@Cast("string_iter_t*") iter: Pointer): Unit

	@JvmStatic @ByVal external fun freepass_vault_get_entry_cbor(@Cast("vault_t*") vault: Pointer, name: String): vector_t
	@JvmStatic external fun freepass_free_entry_cbor(@ByVal cbor: vector_t)
	@JvmStatic external fun freepass_vault_put_entry_cbor(@Cast("vault_t*") vault: Pointer, name: String, @Cast("uint8_t*") data: BytePointer, @Cast("size_t") len: Int)

	class vector_t : Pointer {
		init { Loader.load() }
		constructor(p: Pointer): super(p) {}
		external fun allocate()
		@Cast("uint8_t*") @MemberGetter external fun data(): BytePointer
		@MemberGetter external fun len(): Int
		@MemberGetter external fun cap(): Int
	}
}