import UIKit

class EntryViewController: UIViewController {
	
	@IBOutlet weak var detailDescriptionLabel: UILabel!
	
	
	var detailItem: AnyObject? {
		didSet {
			self.configureView()
		}
	}
	
	func configureView() {
		if let detail = self.detailItem,
		   let label = self.detailDescriptionLabel {
			label.text = detail.description
			print(Vault.getEntry(detail.description))
		}
	}
	
	override func viewDidLoad() {
		super.viewDidLoad()
		self.configureView()
	}
	
	override func didReceiveMemoryWarning() {
		super.didReceiveMemoryWarning()
	}
	
	
}