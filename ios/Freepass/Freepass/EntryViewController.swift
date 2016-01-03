import UIKit
import Bond

class EntryViewController: UITableViewController {

	@IBOutlet weak var editButton: UIBarButtonItem!

	let inEditMode = Observable(false)
	let fields : ObservableArray<FieldViewModel> = ObservableArray([])
	let entryName = Observable("")

	var entry: Entry? {
		didSet {
//			print(entry!.fields)
			setFields(entry!.fields)
		}
	}

	func setFields(fields: [(String, Field)]) {
		self.fields.array = fields.map { (k, v) in FieldViewModel(name: k, field: v) }
	}

	@IBAction func toggleEdit(sender: AnyObject) {
		self.inEditMode.value = !self.inEditMode.value
		if (!self.inEditMode.value) {
			entry!.fields = 	self.fields.flatMap { $0.toField() }
			print(entry!.fields)
			// TODO: save
		}
	}

	func cancelEdit(sender: AnyObject) {
		self.inEditMode.value = false
		setFields(self.entry!.fields)
	}

	override func viewDidLoad() {
		super.viewDidLoad()
		self.tableView.backgroundColor = Colors.primaryBackground
		self.tableView.rowHeight = UITableViewAutomaticDimension
		self.tableView.allowsSelectionDuringEditing = false
		let cancelButton = UIBarButtonItem(title: "Cancel", style: .Plain, target: self, action: "cancelEdit:")
		self.entryName.observe { self.title = $0 }
		self.inEditMode.observe {
			self.navigationItem.setHidesBackButton($0, animated: true)
			self.navigationController?.interactivePopGestureRecognizer?.enabled = !$0
			self.navigationItem.setLeftBarButtonItem($0 ? cancelButton : nil, animated: true)
			self.editButton.title = $0 ? "Save" : "Edit"
			self.tableView.editing = $0
			self.tableView.estimatedRowHeight = $0 ? 140.0 : 60.0
		}
		self.fields.lift()
			.combineLatestWith(self.inEditMode).map { (e, _) in e } // Update when inEditMode is updated, but get inEditMode directly inside the block because bindTo needs the value to be of the event type
			.bindTo(self.tableView) { indexPath, dataSource, tableView in
				let	fieldViewModel = self.fields[indexPath.row]
				if (self.inEditMode.value) {
					let cell = EditFieldCell(forField: fieldViewModel)
					cell.tableView = self.tableView
					return cell
				}
				return ShowPasswordFieldCell(forField: fieldViewModel)
			}
	}

	override func didReceiveMemoryWarning() {
		super.didReceiveMemoryWarning()
	}

}