import Foundation
import RxSwift
import RxCocoa

// https://github.com/ReactiveX/RxSwift/blob/master/RxExample/RxExample/Operators.swift#L20
// Two way binding operator between control property and variable, that's all it takes

infix operator <-> {}

func <-> <T>(property: ControlProperty<T>, variable: Variable<T>) -> Disposable {
	let bindToUIDisposable = variable.asObservable().bindTo(property)
	let bindToVariable = property
		.subscribe(onNext: { n in
			variable.value = n
			}, onCompleted:  {
				bindToUIDisposable.dispose()
			})
	return StableCompositeDisposable.create(bindToUIDisposable, bindToVariable)
}