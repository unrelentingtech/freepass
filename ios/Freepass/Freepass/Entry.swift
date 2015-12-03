import SwiftCBOR

enum DerivedUsage {
	case Password(template: String)
	case Ed25519Key(usage: String)
	case RawKey

	init?(fromCbor obj: CBOR) {
		switch obj["variant"] {
		case "Password"?:
			guard case let CBOR.UTF8String(template)? = obj["fields"]?[0] else { return nil }
			self = .Password(template: template)
		case "Ed25519Key"?:
			guard case let CBOR.UTF8String(usage)? = obj["fields"]?[0] else { return nil }
			self = .Ed25519Key(usage: usage)
		case "RawKey"?:
			self = .RawKey
		default: return nil
		}
	}
}

enum Field {
	case Derived(counter: UInt32, site_name: String?, usage: DerivedUsage)
	case Stored(data: [UInt8], usage: String)

	init?(fromCbor obj: CBOR) {
		switch obj["variant"] {
		case "Derived"?:
			guard case let CBOR.UnsignedInt(counter)? = obj["fields"]?[0] else { return nil }
			let site_name : String?
			if case CBOR.UTF8String(let site_name_)? = obj["fields"]?[1] { site_name = site_name_ } else { site_name = nil }
			guard let uobj = obj["fields"]?[2], usage = DerivedUsage(fromCbor: uobj) else { return nil }
			self = .Derived(counter: UInt32(counter), site_name: site_name, usage: usage)
		case "Stored"?:
			guard case let CBOR.ByteString(data)? = obj["fields"]?[0] else { return nil }
			guard case let CBOR.UTF8String(usage)? = obj["fields"]?[1] else { return nil }
			self = .Stored(data: data, usage: usage)
		default: return nil
		}
	}
}

class Entry {
	var fields : [String : Field] = [:]

	init?(fromCbor arr: CBOR) {
		guard case let CBOR.Map(field_objs)? = arr[0]?["fields"] else { return nil }
		field_objs.forEach {
			guard case let CBOR.UTF8String(key) = $0 else { return }
			fields[key] = Field(fromCbor: $1)
		}
	}
}