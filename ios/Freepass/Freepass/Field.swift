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

enum StoredUsage {
	case Text
	case Password

	init?(fromCbor obj: CBOR) {
		switch obj {
		case "Text": self = .Text
		case "Password": self = .Password
		default: return nil
		}
	}
}

enum Field {
	case Derived(counter: UInt32, site_name: String?, usage: DerivedUsage)
	case Stored(data: [UInt8], usage: StoredUsage)

	init?(fromCbor obj: CBOR) {
		switch obj["variant"] {
		case "Derived"?:
			guard case let CBOR.UnsignedInt(counter)? = obj["fields"]?[0] else { return nil }
			let site_name: String?
			if case CBOR.UTF8String(let site_name_)? = obj["fields"]?[1] { site_name = site_name_ } else { site_name = nil }
			guard let uobj = obj["fields"]?[2], usage = DerivedUsage(fromCbor: uobj) else { return nil }
			self = .Derived(counter: UInt32(counter), site_name: site_name, usage: usage)
		case "Stored"?:
			guard case let CBOR.ByteString(data)? = obj["fields"]?[0] else { return nil }
			guard let uobj = obj["fields"]?[1], usage = StoredUsage(fromCbor: uobj) else { return nil }
			self = .Stored(data: data, usage: usage)
		default: return nil
		}
	}

	init() {
		self = .Derived(counter: 1, site_name: nil, usage: .Password(template: "Maximum"))
	}
}