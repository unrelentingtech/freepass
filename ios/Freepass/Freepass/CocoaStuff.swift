import UIKit

// Have to use this instead of a weak ref because for some weird FUCKING reason,
// a weak ref somehow prevents the first cell from being deleted when you stop editing
// https://stackoverflow.com/a/26469012
extension UITableViewCell {
	var tableView: UITableView? {
		get {
			for var view = self.superview; view != nil; view = view!.superview {
				if view! is UITableView {
					return (view! as! UITableView)
				}
			}
			return nil
		}
	}
}