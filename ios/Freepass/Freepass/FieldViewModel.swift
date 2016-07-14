import Foundation
import RxSwift

class FieldViewModel {
	enum FieldType {
		case Stored
		case Derived
	}

	let dbag = DisposeBag()
	let field_type: Variable<FieldType?> = Variable(nil)
	let field_name = Variable("")
	let derived_counter: Variable<UInt32> = Variable(0)
	let derived_site_name = Variable("")
	let derived_usage: Variable<DerivedUsage?> = Variable(nil)
	let stored_data: Variable<[UInt8]?> = Variable(nil)
	let stored_data_string = Variable("")
	let stored_usage: Variable<StoredUsage?> = Variable(nil)

	func init_stored_data_conversion() {
		var updatingFromSelf = false
		stored_data.asObservable().subscribeNext {
			if updatingFromSelf { return }
			let b = $0 ?? []
			if let s = NSString(bytes: b, length: b.count, encoding: NSUTF8StringEncoding) as? String {
				self.stored_data_string.value = s
			}
		}.addDisposableTo(dbag)
		stored_data_string.asObservable().subscribeNext {
			updatingFromSelf = true
			self.stored_data.value = $0.utf8.map { $0 }
			updatingFromSelf = false
		}.addDisposableTo(dbag)
	}

	init(name: String, field: Field) {
		init_stored_data_conversion()
		field_name.value = name
		switch field {
		case .Derived(let counter, let site_name, let usage):
			field_type.value = .Derived
			derived_counter.value = counter
			derived_site_name.value = site_name ?? ""
			derived_usage.value = usage
		case .Stored(data: let data, usage: let usage):
			field_type.value = .Stored
			stored_data.value = data
			stored_usage.value = usage
		}
	}

	func toField() -> (String, Field)? {
		switch (field_name.value, field_type.value) {
		case (let name, .Derived?):
			return (name, .Derived(counter: derived_counter.value ?? 1, site_name: derived_site_name.value, usage: derived_usage.value ?? .Password(template: "Maximum")))
		case (let name, .Stored?):
			return (name, .Stored(data: stored_data.value ?? [], usage: stored_usage.value ?? .Password))
		default: return nil
		}
	}
}