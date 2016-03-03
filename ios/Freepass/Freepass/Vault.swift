import Foundation
import SwiftCBOR

enum VaultError : ErrorType {
	case WrongPassword
	case UnknownError
}

struct Vault {
	private static var masterKey : COpaquePointer? = nil
	private static var vaultObj : COpaquePointer? = nil
	private static var entriesKey : COpaquePointer? = nil
	private static var filePath : String? = nil
	private static var isOpen = false

	static func open(path: String, userName: String, password: String) throws {
		masterKey = rusterpassword_gen_master_key(password, userName)
		if masterKey == COpaquePointer.init(nilLiteral: ()) {
			throw VaultError.UnknownError
		}
		let outerKey = freepass_gen_outer_key(masterKey!)
		entriesKey = freepass_gen_entries_key(masterKey!)

		let fm = NSFileManager.defaultManager()
		if fm.isReadableFileAtPath(path) {
			filePath = path
			vaultObj = freepass_open_vault(path, entriesKey!, outerKey)
		} else {
			vaultObj = freepass_new_vault(entriesKey!, outerKey)
		}
		freepass_free_outer_key(outerKey)

		if vaultObj == COpaquePointer.init(nilLiteral: ()) {
			rusterpassword_free_master_key(masterKey!)
			freepass_free_entries_key(entriesKey!)
			throw VaultError.WrongPassword
		}
		isOpen = true
	}

	static func entryNames() -> [String] {
		guard let vaultObj = vaultObj else { return [] }
		let names_iter = freepass_vault_get_entry_names_iterator(vaultObj)
		defer { freepass_free_entry_names_iterator(names_iter) }
		var names = [String]()
		var curr = freepass_entry_names_iterator_next(names_iter)
		while curr != UnsafeMutablePointer.init(nilLiteral: ()) {
			names.append(String.fromCString(curr)!)
			freepass_free_entry_name(curr)
			curr = freepass_entry_names_iterator_next(names_iter)
		}
		return names
	}

	static func getEntry(name: String) -> Entry? {
		guard let vaultObj = vaultObj else { return nil }
		guard let entriesKey = entriesKey else { return nil }
		let cbor = freepass_vault_get_entry_cbor(vaultObj, name)
		defer { freepass_free_entry_cbor(cbor) }
		let bytes = Array(UnsafeBufferPointer(start: cbor.data, count: cbor.len))
		guard let result = try! CBORDecoder(input: bytes).decodeItem() else { return nil }
		return Entry(fromCbor: result)
	}

	static func close() {
		if let masterKey = masterKey {
			rusterpassword_free_master_key(masterKey)
		}
		masterKey = nil
		
		if let entriesKey = entriesKey {
			freepass_free_entries_key(entriesKey)
		}
		
		masterKey = nil
		if let vaultObj = vaultObj {
			freepass_close_vault(vaultObj)
		}
		isOpen = false
	}
}