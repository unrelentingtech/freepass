import UIKit
import RxSwift
import RxCocoa
import Cartography


class ShowFieldCell: UITableViewCell {
	var dbag = DisposeBag()
	lazy var name = UITextField()
	lazy var content = UITextField()

	override init(style: UITableViewCellStyle, reuseIdentifier: String?) {
		super.init(style: style, reuseIdentifier: reuseIdentifier)
		self.addSubview(name)
		self.addSubview(content)
	}

	required init?(coder aDecoder: NSCoder) { // REALLY?
		fatalError("init(coder:) has not been implemented")
	}

	override func layoutSubviews() {
		super.layoutSubviews()
		name.enabled = false
		name.textColor = Colors.primaryAccent
		name.font = name.font?.fontWithSize(16)

		content.enabled = false
		content.textColor = Colors.primaryContent
	}
	
	func setField(field: FieldViewModel) {
		dbag = DisposeBag()
		field.field_name.asObservable().bindTo(name.rx_text).addDisposableTo(dbag)
		updateConstraints()
	}

	override func updateConstraints() {
		constrain(name, content) { name, content in
			for v in [name, content] {
				v.centerX == name.superview!.centerX
				v.width == name.superview!.width - 20
//				v.height == 24
			}
			name.top == name.superview!.topMargin
			content.bottom == name.superview!.bottomMargin
			distribute(by: 4, vertically: name, content)
		}
		super.updateConstraints()
	}
}

class ShowPasswordFieldCell: ShowFieldCell {
	override init(style: UITableViewCellStyle, reuseIdentifier: String?) {
		super.init(style: style, reuseIdentifier: reuseIdentifier)
	}

	required init?(coder aDecoder: NSCoder) { // WHAT
		fatalError("init(coder:) has not been implemented")
	}

	override func layoutSubviews() {
		super.layoutSubviews()
		content.secureTextEntry = true
		content.text = "************"
	}
}

class EditFieldCell: UITableViewCell {
	var dbag = DisposeBag()
	weak var tableView: UITableView?
	weak var field: FieldViewModel?
	lazy var name_field = UITextField()
	lazy var type_selector = UISegmentedControl(items: ["Derived", "Stored"])
	lazy var derived_site_name_field = UITextField()
	lazy var derived_counter_label = UILabel()
	lazy var derived_counter_stepper = UIStepper()
	lazy var stored_string_field = UITextField()
	
	override init(style: UITableViewCellStyle, reuseIdentifier: String?) {
		super.init(style: style, reuseIdentifier: reuseIdentifier)
		self.addSubview(name_field)
		self.addSubview(type_selector)
		self.addSubview(derived_site_name_field)
		self.addSubview(derived_counter_stepper)
		self.addSubview(derived_counter_label)
		self.addSubview(stored_string_field)
	}
	
	override func layoutSubviews() {
		super.layoutSubviews()
		name_field.textColor = Colors.primaryContent

		derived_site_name_field.placeholder = "Site name (leave blank to use the entry name)"
		derived_site_name_field.textColor = Colors.primaryContent

		derived_counter_stepper.maximumValue = Double(UInt32.max)
		derived_counter_label.textColor = Colors.primaryContent

		stored_string_field.textColor = Colors.primaryContent
		updateConstraints()
	}

	func setField(field: FieldViewModel, row: NSIndexPath) {
		dbag = DisposeBag()
		self.field = field

		(name_field.rx_text <-> field.field_name).addDisposableTo(dbag)

		type_selector.rx_value.subscribeNext { field.field_type.value = $0 == 0 ? .Some(.Derived) : .Some(.Stored) }.addDisposableTo(dbag)
		field.field_type.asObservable().map { $0 == .Some(.Derived) ? 0 : 1 }.bindTo(type_selector.rx_value).addDisposableTo(dbag)

		field.field_type.asObservable().map { $0 == .Some(.Derived) }.distinctUntilChanged().subscribeNext {
			self.derived_site_name_field.hidden = !$0
			self.derived_counter_stepper.hidden = !$0
			self.derived_counter_label.hidden = !$0
			self.stored_string_field.hidden = $0
			self.updateConstraints()
			self.tableView?.reloadRowsAtIndexPaths([row], withRowAnimation: .None)
		}.addDisposableTo(dbag)

		(derived_site_name_field.rx_text <-> field.derived_site_name).addDisposableTo(dbag)

		field.derived_counter.asObservable().subscribeNext { self.derived_counter_stepper.value = Double($0 ?? 1) }.addDisposableTo(dbag)
		derived_counter_stepper.rx_value.map { UInt32($0) }.bindTo(field.derived_counter).addDisposableTo(dbag)
		field.derived_counter.asObservable().map { "Counter: \($0 ?? 1)" }
			.bindTo(derived_counter_label.rx_text).addDisposableTo(dbag)

		(stored_string_field.rx_text <->	 field.stored_data_string).addDisposableTo(dbag)

		field.stored_usage.asObservable().subscribeNext {
			switch $0 ?? .Text {
			case .Password: self.stored_string_field.placeholder = "Password"
			case .Text: self.stored_string_field.placeholder = "Text"
			}
		}.addDisposableTo(dbag)


		updateConstraints()
	}

	let group = ConstraintGroup()

	override func updateConstraints() {
		constrain([name_field, type_selector, derived_site_name_field, derived_counter_label, derived_counter_stepper, stored_string_field], replace: group) {
			let name_field = $0[0], type_selector = $0[1], derived_site_name_field = $0[2], derived_counter_label = $0[3], derived_counter_stepper = $0[4], stored_string_field = $0[5] //  FUCK
			guard let superview = name_field.superview else { return }
			let isDerived = self.field?.field_type.value == .Some(.Derived)
			for v in [name_field, type_selector, derived_site_name_field, stored_string_field] {
				v.left == superview.left + 10
				v.right == superview.right - 10
				v.height == 24
			}
			name_field.top == superview.topMargin
			if isDerived {
				derived_counter_stepper.bottom == superview.bottomMargin
				derived_counter_label.left == superview.left + 10
				derived_counter_label.right == derived_counter_stepper.left - 10
				derived_counter_label.top == derived_counter_stepper.top
				derived_counter_label.bottom == derived_counter_stepper.bottom
				derived_counter_stepper.right == superview.right - 10
				distribute(by: 10, vertically: name_field, type_selector, derived_site_name_field, derived_counter_stepper)
			} else {
				stored_string_field.bottom == superview.bottomMargin
				distribute(by: 10, vertically: name_field, type_selector, stored_string_field)
			}
		}
		super.updateConstraints()
	}

	required init?(coder aDecoder: NSCoder) { // SERIOUSLY?
		fatalError("init(coder:) has not been implemented")
	}
}