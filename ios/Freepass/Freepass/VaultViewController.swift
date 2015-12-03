import UIKit

let NEW_ENTRY_NAME = "New entry"

class VaultViewController: UITableViewController, UISearchResultsUpdating {

	var entryViewController: EntryViewController? = nil
	var entrySearchController = UISearchController(searchResultsController: nil)
	var entryNames = [String]()
	var filteredEntryNames = [String]()


	override func viewDidLoad() {
		super.viewDidLoad()
		entryNames = Vault.entryNames()

		self.definesPresentationContext = true
		let addButton = UIBarButtonItem(barButtonSystemItem: .Add, target: self, action: "insertNewObject:")
		self.navigationItem.rightBarButtonItem = addButton
		if let split = self.splitViewController {
			let controllers = split.viewControllers
			self.entryViewController = (controllers[controllers.count-1] as! UINavigationController).topViewController as? EntryViewController
		}
		self.entrySearchController.searchResultsUpdater = self
		self.entrySearchController.dimsBackgroundDuringPresentation = false
		self.entrySearchController.hidesNavigationBarDuringPresentation = false
		self.entrySearchController.searchBar.sizeToFit()
		self.entrySearchController.searchBar.searchBarStyle = .Minimal
		self.tableView.tableHeaderView = self.entrySearchController.searchBar
		self.tableView.reloadData()
	}

	override func viewWillAppear(animated: Bool) {
		self.clearsSelectionOnViewWillAppear = self.splitViewController!.collapsed
		super.viewWillAppear(animated)
	}

	override func didReceiveMemoryWarning() {
		super.didReceiveMemoryWarning()
	}

	func insertNewObject(sender: AnyObject) {
		entryNames.insert(NEW_ENTRY_NAME, atIndex: 0)
		let indexPath = NSIndexPath(forRow: 0, inSection: 0)
		self.tableView.insertRowsAtIndexPaths([indexPath], withRowAnimation: .Automatic)
	}

	// MARK: - Segues

	override func prepareForSegue(segue: UIStoryboardSegue, sender: AnyObject?) {
		if segue.identifier == "showEntry" {
			if let indexPath = self.tableView.indexPathForSelectedRow {
				let entryName = entryNames[indexPath.row]
				let controller = (segue.destinationViewController as! UINavigationController).topViewController as! EntryViewController
				controller.entryName = entryName
				if (entryName != NEW_ENTRY_NAME) {
					controller.entry = Vault.getEntry(entryName)
				} else {
					controller.entry = Entry()
				}
				controller.navigationItem.leftBarButtonItem = self.splitViewController?.displayModeButtonItem()
				controller.navigationItem.leftItemsSupplementBackButton = true
			}
		}
	}

	// MARK: - Table View

	override func numberOfSectionsInTableView(tableView: UITableView) -> Int {
		return 1
	}

	override func tableView(tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
		if (self.entrySearchController.active) {
			return self.filteredEntryNames.count
		} else {
			return self.entryNames.count
		}
	}

	override func tableView(tableView: UITableView, cellForRowAtIndexPath indexPath: NSIndexPath) -> UITableViewCell {
		let cell = tableView.dequeueReusableCellWithIdentifier("Cell", forIndexPath: indexPath)
		let entryName: String
		if (self.entrySearchController.active) {
			entryName = filteredEntryNames[indexPath.row]
		} else {
			entryName = entryNames[indexPath.row]
		}
		cell.textLabel!.text = entryName
		return cell
	}

	func updateSearchResultsForSearchController(searchController: UISearchController) {
		self.filteredEntryNames.removeAll(keepCapacity: false)
		let array = (self.entryNames as NSArray).filteredArrayUsingPredicate(
			NSPredicate(format: "SELF CONTAINS[c] %@", self.entrySearchController.searchBar.text!)
		)
		self.filteredEntryNames = array as! [String]
		self.tableView.reloadData()
	}

}