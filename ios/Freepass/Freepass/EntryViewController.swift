import UIKit
import RxSwift

class EntryViewController: UITableViewController {

	@IBOutlet weak var editButton: UIBarButtonItem!

	let dbag = DisposeBag()
	let inEditMode = Variable(false)
	var fieldModels : [FieldViewModel] = []
	let entryName = Variable("")

	var entry: Entry? {
		didSet {
//			print(entry!.fields)
			setFieldModels(entry!.fields)
		}
	}

	func setFieldModels(fields: [(String, Field)]) {
		self.fieldModels = fields.map { (k, v) in FieldViewModel(name: k, field: v) }
		self.tableView.reloadData()
	}

	@IBAction func toggleEdit(sender: AnyObject) {
		self.inEditMode.value = !self.inEditMode.value
		if (!self.inEditMode.value) {
			entry!.fields = 	self.fieldModels.flatMap { $0.toField() }
//			print(entry!.fields)
			// TODO: save
		}
	}

	func cancelEdit(sender: AnyObject) {
		self.inEditMode.value = false
		setFieldModels(self.entry!.fields)
	}

	override func viewDidLoad() {
		super.viewDidLoad()
		self.tableView.backgroundColor = Colors.primaryBackground
		self.tableView.rowHeight = UITableViewAutomaticDimension
		self.tableView.allowsSelectionDuringEditing = false
		let cancelButton = UIBarButtonItem(title: "Cancel", style: .Plain, target: self, action: "cancelEdit:")
		self.entryName.asObservable().subscribeNext { self.title = $0 }.addDisposableTo(dbag)
		self.inEditMode.asObservable().distinctUntilChanged().subscribeNext {
			self.navigationItem.setHidesBackButton($0, animated: true)
			self.navigationController?.interactivePopGestureRecognizer?.enabled = !$0
			self.navigationItem.setLeftBarButtonItem($0 ? cancelButton : nil, animated: true)
			self.editButton.title = $0 ? "Save" : "Edit"
			self.tableView.editing = $0
			self.tableView.estimatedRowHeight = $0 ? 140.0 : 80.0
			self.tableView.reloadData()
		}.addDisposableTo(dbag)
		self.tableView.reloadData()
	}

	override func didReceiveMemoryWarning() {
		super.didReceiveMemoryWarning()
	}

	// MARK: - Table View

	override func numberOfSectionsInTableView(tableView: UITableView) -> Int {
		return 1
	}

	override func tableView(tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
		return self.fieldModels.count
	}

	override func tableView(tableView: UITableView, cellForRowAtIndexPath indexPath: NSIndexPath) -> UITableViewCell {
		let field = fieldModels[indexPath.row]
		if (inEditMode.value) {
			let cell = EditFieldCell.init(style: .Default, reuseIdentifier: nil)
			cell.setField(field)
			return cell
		} else {
			let cell = ShowPasswordFieldCell.init(style: .Default, reuseIdentifier: nil)
			cell.setField(field)
			return cell
		}
	}

}