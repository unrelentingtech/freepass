import UIKit
import HexColor

// http://flatuicolors.com

struct Colors {
	static let wisteria = UIColor(0x8e44ad)
	static let clouds = UIColor(0xecf0f1)
	static let silver = UIColor(0xbdc3c7)
	static let wetAsphalt = UIColor(0x34495e)

	static let primaryBackground = silver
	static let primaryContent = wetAsphalt
	static let primaryAccent = wisteria

	static let secondaryBackground = clouds

	static let toolbarBackground = wisteria
	static let toolbarContent = clouds

	static func setup() {
		UILabel.appearance().tintColor = primaryContent
		UITextView.appearance().tintColor = primaryContent
		UITableView.appearance().sectionIndexBackgroundColor = primaryBackground
		UITableView.appearance().backgroundColor = secondaryBackground
		UITableViewCell.appearance().backgroundColor = secondaryBackground
		UIButton.appearance().tintColor = primaryAccent
		UISearchBar.appearance().tintColor = primaryAccent

		UINavigationBar.appearance().barTintColor = toolbarBackground
		UINavigationBar.appearance().tintColor = toolbarContent
		UINavigationBar.appearance().titleTextAttributes = [NSForegroundColorAttributeName: toolbarContent]
		UIBarButtonItem.appearance().tintColor = toolbarContent
	}
}