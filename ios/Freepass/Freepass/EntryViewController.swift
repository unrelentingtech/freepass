import UIKit
import RxSwift

class EntryViewController: UITableViewController {

	@IBOutlet weak var editButton: UIBarButtonItem!

	let dbag = DisposeBag()
	let inEditMode = Variable(false)
	var fields : [FieldViewModel] = []
	let entryName = Variable("")

	var entry: Entry? {
		didSet {
//			print(entry!.fields)
			setFields(entry!.fields)
		}
	}

	func setFields(fields: [(String, Field)]) {
		self.fields = fields.map { (k, v) in FieldViewModel(name: k, field: v) }
		self.tableView!.reloadData()
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
		self.tableView.registerClass(ShowPasswordFieldCell.self, forCellReuseIdentifier: "ShowPasswordFieldCell")
		self.tableView.registerClass(EditFieldCell.self, forCellReuseIdentifier: "EditStoredFieldCell")
		self.tableView.registerClass(EditFieldCell.self, forCellReuseIdentifier: "EditDerivedFieldCell")
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
		return self.fields.count
	}

	override func tableView(tableView: UITableView, cellForRowAtIndexPath indexPath: NSIndexPath) -> UITableViewCell {
		if (self.inEditMode.value) {
			let field = fields[indexPath.row]
			let cell : EditFieldCell
			switch field.field_type.value ?? .Derived {
			case .Derived: cell = self.tableView.dequeueReusableCellWithIdentifier("EditDerivedFieldCell", forIndexPath: indexPath) as! EditFieldCell
			case .Stored:  cell = self.tableView.dequeueReusableCellWithIdentifier("EditStoredFieldCell", forIndexPath: indexPath) as! EditFieldCell
			}
			cell.tableView = self.tableView
			cell.setField(field, row: indexPath)
			return cell
		} else {
			let cell = self.tableView.dequeueReusableCellWithIdentifier("ShowPasswordFieldCell", forIndexPath: indexPath) as! ShowPasswordFieldCell
			cell.setField(fields[indexPath.row])
			return cell
		}
	}

}