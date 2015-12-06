import Foundation
import Bond

class FieldViewModel {
	enum FieldType {
		case Stored
		case Derived
	}

	let field_type: Observable<FieldType?> = Observable(nil)
	let field_name: Observable<String?> = Observable(nil)
	let derived_counter: Observable<UInt32?> = Observable(nil)
	let derived_site_name: Observable<String?> = Observable(nil)
	let derived_usage: Observable<DerivedUsage?> = Observable(nil)
	let stored_data: Observable<[UInt8]?> = Observable(nil)
	let stored_data_string: Observable<String?> = Observable(nil)
	let stored_usage: Observable<StoredUsage?> = Observable(nil)

	func init_stored_data_conversion() {
		var updatingFromSelf = false
		stored_data.observe {
			if updatingFromSelf { return }
			guard let b = $0 else {
				self.stored_data_string.value = nil
				return
			}
			self.stored_data_string.value = NSString(bytes: b, length: b.count, encoding: NSUTF8StringEncoding) as? String
		}
		stored_data_string.observe {
			updatingFromSelf = true
			self.stored_data.value = $0?.utf8.map { UInt8(bitPattern: Int8($0.value)) }
			updatingFromSelf = false
		}
		// XXX: fix updating via string
	}

	init(name: String, field: Field) {
		field_name.value = name
		switch field {
		case .Derived(let counter, let site_name, let usage):
			field_type.value = .Derived
			derived_counter.value = counter
			derived_site_name.value = site_name
			derived_usage.value = usage
		case .Stored(data: let data, usage: let usage):
			field_type.value = .Stored
			stored_data.value = data
			stored_usage.value = usage
		}
	}

	func toField() -> (String, Field)? {
		switch (field_name.value, field_type.value) {
		case (let name?, .Derived?):
			return (name, .Derived(counter: derived_counter.value ?? 1, site_name: derived_site_name.value, usage: derived_usage.value ?? .Password(template: "Maximum")))
		case (let name?, .Stored?):
			return (name, .Stored(data: stored_data.value ?? [], usage: stored_usage.value ?? .Password))
		default: return nil
		}
	}
}