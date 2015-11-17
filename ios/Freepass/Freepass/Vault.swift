import Foundation

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
		let fm = NSFileManager.defaultManager()
		if !(fm.isReadableFileAtPath(path) && fm.isWritableFileAtPath(path)) {
			fm.createFileAtPath(path, contents: nil, attributes: nil)
		}
		filePath = path
		masterKey = rusterpassword_gen_master_key(password, userName)
		if masterKey == nil {
			throw VaultError.UnknownError
		}
		let outerKey = freepass_gen_outer_key(masterKey!)
		vaultObj = freepass_open_vault(path, outerKey)
		if vaultObj == nil {
			throw VaultError.WrongPassword
		}
		freepass_free_outer_key(outerKey)
		isOpen = true
	}
	
	static func close() {
		if let masterKey = masterKey {
			rusterpassword_free_master_key(masterKey)
		}
		masterKey = nil
		if let vaultObj = vaultObj {
			freepass_close_vault(vaultObj)
		}
		isOpen = false
	}
}