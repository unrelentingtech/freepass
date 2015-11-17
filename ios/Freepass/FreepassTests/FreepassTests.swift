import XCTest
@testable import Freepass

class FreepassTests: XCTestCase {
	
	override func setUp() {
		super.setUp()
		// Put setup code here. This method is called before the invocation of each test method in the class.
	}
	
	override func tearDown() {
		// Put teardown code here. This method is called after the invocation of each test method in the class.
		super.tearDown()
	}
	
	func testExample() {
		// This is an example of a functional test case.
		// Use XCTAssert and related functions to verify your tests produce the correct results.
	}
	
	func testPerformanceMasterKeyDerivation() {
		self.measureBlock {
			let mkey = rusterpassword_gen_master_key("Correct Horse Battery Staple", "Cosima Niehaus")
			rusterpassword_free_master_key(mkey)
		}
	}
	
}