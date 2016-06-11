package technology.unrelenting.freepass

import android.os.Bundle
import android.support.v4.app.Fragment
import android.support.v7.widget.LinearLayoutManager
import android.support.v7.widget.RecyclerView
import android.util.TypedValue
import android.view.Gravity
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.TextView

import org.jetbrains.anko.*
import org.jetbrains.anko.recyclerview.v7.recyclerView
import org.jetbrains.anko.support.v4.UI

class VaultFragment: Fragment() {

	override fun onCreateView(inflater: LayoutInflater?, container: ViewGroup?, savedInstanceState: Bundle?): View? {
		super.onCreateView(inflater, container, savedInstanceState)
		return UI {
			verticalLayout {
				recyclerView {
					setHasFixedSize(true)
					layoutManager = LinearLayoutManager(ctx)
					adapter = EntryAdapter(Vault.entryNames())
					addItemDecoration(DividerItemDecoration(ctx))
				}.lparams(width = matchParent, height = matchParent)
			}
		}.view
	}

	class EntryAdapter(val entryNames: List<String>): RecyclerView.Adapter<EntryAdapter.ViewHolder>() {
		class ViewHolder(val view: View): RecyclerView.ViewHolder(view) {}

		class EntryViewUI: AnkoComponent<ViewGroup> {
			override fun createView(ui: AnkoContext<ViewGroup>): View {
				return with(ui) {
					frameLayout {
						lparams(height = dip(48))
						val textView = textView {
							id = R.id.entry_name
							setTextSize(TypedValue.COMPLEX_UNIT_SP, 18f)
							text = "..........."
						}.lparams(width = matchParent) {
							horizontalPadding = dip(16)
							gravity = Gravity.CENTER_VERTICAL
						}
						isClickable = true
						onClick {
							println("Sel ${textView.text} ${Vault.getEntry(textView.text.toString())}")
						}
					}
				}
			}
		}

		override fun onCreateViewHolder(parent: ViewGroup?, viewType: Int): ViewHolder? {
			return ViewHolder(EntryViewUI().createView(AnkoContext.create(parent!!.context, parent)))
		}

		override fun onBindViewHolder(holder: ViewHolder?, position: Int) {
			holder!!.view.find<TextView>(R.id.entry_name).text = entryNames[position]
		}

		override fun getItemCount(): Int {
			return entryNames.count()
		}
	}

}