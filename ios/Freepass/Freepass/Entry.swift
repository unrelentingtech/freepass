import SwiftCBOR

class Entry {
	var fields: [(String, Field)] = []

	init?(fromCbor arr: CBOR) {
		guard case let CBOR.Map(field_objs)? = arr[0]?["fields"] else { return nil }
		field_objs.forEach {
			guard case let CBOR.UTF8String(key) = $0 else { return }
			guard let value = Field(fromCbor: $1) else { return }
			fields.append((key, value))
		}
	}

	init() {}
}