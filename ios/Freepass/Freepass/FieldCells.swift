import UIKit
import Bond
import Cartography

class ShowFieldCell: UITableViewCell {
	lazy var name = UITextField()
	lazy var content = UITextField()

	init(forField field: FieldViewModel) {
		super.init(style: .Default, reuseIdentifier: nil)

		self.addSubview(name)
		field.field_name.bindTo(name.bnd_text).disposeIn(self.bnd_bag)
		name.enabled = false
		name.textColor = Colors.primaryAccent
		name.font = name.font?.fontWithSize(16)

		self.addSubview(content)
		content.enabled = false
		content.textColor = Colors.primaryContent

		updateConstraints()
	}

	override func updateConstraints() {
		constrain(name, content) { name, content in
			for v in [name, content] {
				v.centerX == name.superview!.centerX
				v.width == name.superview!.width - 20
			}
			name.top == name.superview!.topMargin
			content.bottom == name.superview!.bottomMargin
			distribute(by: 4, vertically: name, content)
		}
		super.updateConstraints()
	}

	required init?(coder aDecoder: NSCoder) { // REALLY?
	    fatalError("init(coder:) has not been implemented")
	}
}

class ShowPasswordFieldCell: ShowFieldCell {
	override init(forField field: FieldViewModel) {
		super.init(forField: field)
		content.secureTextEntry = true
		content.text = "************"
	}

	required init?(coder aDecoder: NSCoder) { // WHAT
	    fatalError("init(coder:) has not been implemented")
	}
}

class EditFieldCell: UITableViewCell {
	weak var tableView: UITableView?
	weak var field: FieldViewModel?
	lazy var name_field = UITextField()
	lazy var type_selector = UISegmentedControl(items: ["Derived", "Stored"])
	lazy var derived_site_name_field = UITextField()
	lazy var derived_counter_label = UILabel()
	lazy var derived_counter_stepper = UIStepper()
	lazy var stored_string_field = UITextField()

	init(forField field: FieldViewModel) {
		super.init(style: .Default, reuseIdentifier: nil)
		self.field = field
		
		self.addSubview(name_field)
		field.field_name.bidirectionalBindTo(name_field.bnd_text).disposeIn(self.bnd_bag)
		name_field.textColor = Colors.primaryContent

		self.addSubview(type_selector)
		// No bidirectional map :-(
		field.field_type.map { $0 == .Some(.Derived) ? 0 : 1 }.distinct().bindTo(type_selector.bnd_selectedSegmentIndex).disposeIn(self.bnd_bag)
		type_selector.bnd_selectedSegmentIndex.distinct().map { $0 == 0 ? .Some(.Derived) : .Some(.Stored) }.bindTo(field.field_type).disposeIn(self.bnd_bag)

		self.addSubview(derived_site_name_field)
		derived_site_name_field.placeholder = "Site name (leave blank to use the entry name)"
		derived_site_name_field.textColor = Colors.primaryContent
		field.derived_site_name.bidirectionalBindTo(derived_site_name_field.bnd_text).disposeIn(self.bnd_bag)

		self.addSubview(derived_counter_stepper)
		derived_counter_stepper.maximumValue = Double(UInt32.max)
		field.derived_counter.observe { self.derived_counter_stepper.value = Double($0 ?? 1) }.disposeIn(self.bnd_bag)
		derived_counter_stepper.bnd_controlEvent.filter { $0 == .ValueChanged }.map { _ in UInt32(self.derived_counter_stepper.value) }.distinct().bindTo(field.derived_counter).disposeIn(self.bnd_bag)

		self.addSubview(derived_counter_label)
		derived_counter_label.textColor = Colors.primaryContent
		field.derived_counter.map { "Counter: \($0 ?? 1)" }.bindTo(derived_counter_label.bnd_text).disposeIn(self.bnd_bag)

		self.addSubview(stored_string_field)
		stored_string_field.textColor = Colors.primaryContent
		field.stored_data_string.bidirectionalBindTo(stored_string_field.bnd_text).disposeIn(self.bnd_bag)
		field.stored_usage.observe {
			switch $0 ?? .Text {
			case .Password: self.stored_string_field.placeholder = "Password"
			case .Text: self.stored_string_field.placeholder = "Text"
			}
		}.disposeIn(self.bnd_bag)

		field.field_type.map { $0 == .Some(.Derived) }.distinct().observe {
			self.derived_site_name_field.hidden = !$0
			self.derived_counter_stepper.hidden = !$0
			self.derived_counter_label.hidden = !$0
			self.stored_string_field.hidden = $0
			self.tableView?.reloadData() // If I try updateConstraints instead, the height won't change!
		}.disposeIn(self.bnd_bag)

		updateConstraints()
	}

	let group = ConstraintGroup()

	override func updateConstraints() {
		constrain([name_field, type_selector, derived_site_name_field, derived_counter_label, derived_counter_stepper, stored_string_field], replace: group) {
			let name_field = $0[0], type_selector = $0[1], derived_site_name_field = $0[2], derived_counter_label = $0[3], derived_counter_stepper = $0[4], stored_string_field = $0[5] // FUCK
			let superview = name_field.superview!
			let isDerived = self.field?.field_type.value == .Some(.Derived)
			for v in [name_field, type_selector, derived_site_name_field, stored_string_field] {
				v.left == superview.left + 10
				v.right == superview.right - 10
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