import UIKit

class LoginViewController: UITableViewController, UISplitViewControllerDelegate {

	@IBOutlet weak var userName: UITextField!
	@IBOutlet weak var password: UITextField!

    override func viewDidLoad() {
        super.viewDidLoad()
		self.view.backgroundColor = Colors.primaryAccent
		print(documentsPath()?.path)
    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
    }

	func documentsPath() -> NSURL? {
		return NSFileManager.defaultManager().URLsForDirectory(.DocumentDirectory, inDomains: .UserDomainMask).first
	}


    // MARK: - Navigation

    override func prepareForSegue(segue: UIStoryboardSegue, sender: AnyObject?) {
		if segue.identifier == "openVault" {
			let splitViewController = segue.destinationViewController as! UISplitViewController
			let navigationController = splitViewController.viewControllers[splitViewController.viewControllers.count-1] as! UINavigationController
			navigationController.topViewController!.navigationItem.leftBarButtonItem = splitViewController.displayModeButtonItem()
			splitViewController.delegate = self
		}
    }

	override func shouldPerformSegueWithIdentifier(identifier: String, sender: AnyObject?) -> Bool {
		if identifier == "openVault" {
			do {
				let path = documentsPath()?.URLByAppendingPathComponent("test.fpass").path! // TODO file selection
				try Vault.open(path!, userName: userName.text!, password: password.text!)
			} catch {
				let alert = UIAlertController(title: "Error", message: "Error", preferredStyle: UIAlertControllerStyle.Alert)
				alert.addAction(UIAlertAction(title: "OK", style: .Default) { (action) in })
				self.presentViewController(alert, animated: true, completion: nil)
				return false
			}
		}
		return true
	}

	// MARK: - Split view

	func splitViewController(splitViewController: UISplitViewController, collapseSecondaryViewController secondaryViewController:UIViewController, ontoPrimaryViewController primaryViewController:UIViewController) -> Bool {
		guard let secondaryAsNavController = secondaryViewController as? UINavigationController else { return false }
		guard let topAsDetailController = secondaryAsNavController.topViewController as? EntryViewController else { return false }
		if topAsDetailController.entry == nil {
			// Return true to indicate that we have handled the collapse by doing nothing; the secondary controller will be discarded.
			return true
		}
		return false
	}

}
