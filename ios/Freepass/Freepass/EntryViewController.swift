import UIKit

class EntryViewController: UIViewController {

	@IBOutlet weak var detailDescriptionLabel: UILabel!

	var entryName: String? {
		didSet {
			title = entryName
		}
	}
	
	var entry: Entry? {
		didSet {
			print(entry!.fields)
		}
	}

	override func viewDidLoad() {
		super.viewDidLoad()
	}

	override func didReceiveMemoryWarning() {
		super.didReceiveMemoryWarning()
	}

}