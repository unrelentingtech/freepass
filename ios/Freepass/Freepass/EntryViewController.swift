import UIKit
import Bond

class FieldCell: UITableViewCell {
	@IBOutlet weak var fieldName: UITextField!
	@IBOutlet weak var fieldContent: UITextField!
}

class EntryViewController: UITableViewController {

	@IBOutlet weak var editButton: UIBarButtonItem!

	let inEditMode = Observable(false)
	let fields : ObservableArray<(String, Field)> = ObservableArray([])
	let entryName = Observable("")

	var entry: Entry? {
		didSet {
			print(entry!.fields)
			self.fields.array = entry!.fields.map { ($0, $1) }
		}
	}

	@IBAction func toggleEdit(sender: AnyObject) {
		self.inEditMode.value = !self.inEditMode.value
	}

	func cancelEdit(sender: AnyObject) {
		self.inEditMode.value = false
	}

	override func viewDidLoad() {
		super.viewDidLoad()
		self.tableView.backgroundColor = Colors.primaryBackground
		let cancelButton = UIBarButtonItem(title: "Cancel", style: .Plain, target: self, action: "cancelEdit:")
		self.entryName.observe { self.title = $0 }
		self.inEditMode.observe {
			self.navigationItem.setHidesBackButton($0, animated: true)
			self.navigationController?.interactivePopGestureRecognizer?.enabled = !$0
			self.navigationItem.setLeftBarButtonItem($0 ? cancelButton : nil, animated: true)
			self.editButton.title = $0 ? "Save" : "Edit"
		}
		self.fields.lift()
			.combineLatestWith(self.inEditMode).map { (e, _) in e } // Update when inEditMode is updated, but get inEditMode directly inside the block because bindTo needs the value to be of the event type
			.bindTo(self.tableView) { indexPath, dataSource, tableView in
				let inEdit = self.inEditMode.value
				let cell = tableView.dequeueReusableCellWithIdentifier("FieldCell", forIndexPath: indexPath) as! FieldCell
				let	(fieldName, field) = self.fields[indexPath.row]
				cell.fieldName.text = fieldName
				cell.fieldName.enabled = inEdit
				cell.fieldName.textColor = inEdit ? Colors.primaryContent : Colors.primaryAccent
				cell.fieldContent.enabled = inEdit
				cell.fieldContent.textColor = Colors.primaryContent
				return cell
			}
	}

	override func didReceiveMemoryWarning() {
		super.didReceiveMemoryWarning()
	}

}