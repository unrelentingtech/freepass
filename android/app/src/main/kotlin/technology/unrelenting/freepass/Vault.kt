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
		Loader.load()
		freepass_init()
	}

	fun open(cache_dir: File, path: String, username: String, password: String) {
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

	@JvmStatic external fun freepass_init(): Unit
	@JvmStatic external fun rusterpassword_gen_master_key(username: String, password: String): Pointer
	@JvmStatic external fun rusterpassword_free_master_key(@Cast("secstr_t*") master_key: Pointer): Unit
	@JvmStatic external fun freepass_gen_outer_key(@Cast("secstr_t*") master_key: Pointer): Pointer
	@JvmStatic external fun freepass_gen_entries_key(@Cast("secstr_t*") master_key: Pointer): Pointer
	@JvmStatic external fun freepass_free_outer_key(@Cast("secstr_t*") outer_key: Pointer): Unit
	@JvmStatic external fun freepass_free_entries_key(@Cast("secstr_t*") entries_key: Pointer): Unit
	@JvmStatic external fun freepass_open_vault(path: String, @Cast("secstr_t*") entries_key: Pointer, @Cast("secstr_t*") outer_key: Pointer): Pointer
	@JvmStatic external fun freepass_new_vault(@Cast("secstr_t*") entries_key: Pointer, @Cast("secstr_t*") outer_key: Pointer): Pointer
	@JvmStatic external fun freepass_close_vault(@Cast("vault_t*") vault: Pointer): Unit
}