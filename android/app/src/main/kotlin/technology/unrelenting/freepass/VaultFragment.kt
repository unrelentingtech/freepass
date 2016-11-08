package technology.unrelenting.freepass

import android.os.Bundle
import android.support.v4.app.Fragment
import android.support.v7.widget.LinearLayoutManager
import android.support.v7.widget.RecyclerView
import android.util.TypedValue
import android.view.*
import android.widget.TextView

import org.jetbrains.anko.*
import org.jetbrains.anko.recyclerview.v7.recyclerView
import org.jetbrains.anko.support.v4.*

class VaultFragment: Fragment() {

	val entryAdapter = EntryAdapter(Vault.entryNames(), this)

	override fun onCreateView(inflater: LayoutInflater?, container: ViewGroup?, savedInstanceState: Bundle?): View? {
		super.onCreateView(inflater, container, savedInstanceState)
		setHasOptionsMenu(true)
		return UI {
			verticalLayout {
				recyclerView {
					setHasFixedSize(true)
					layoutManager = LinearLayoutManager(ctx)
					adapter = entryAdapter
					addItemDecoration(DividerItemDecoration(ctx))
				}.lparams(width = matchParent, height = matchParent)
			}
		}.view
	}

	override fun onCreateOptionsMenu(menu: Menu?, inflater: MenuInflater?) {
		inflater?.inflate(R.menu.vault, menu)
	}

	override fun onOptionsItemSelected(item: MenuItem?): Boolean {
		when (item?.itemId) {
			R.id.add_entry -> {
				openEntry("New entry")
			}
		}
		return false
	}

	fun openEntry(name: String) {
		startActivity<EntryActivity>(EntryFragment.ENTRY_NAME to name)
	}

	class EntryAdapter(var entryNames: List<String>, val fragment: VaultFragment): RecyclerView.Adapter<EntryAdapter.ViewHolder>() {
		class ViewHolder(val view: View): RecyclerView.ViewHolder(view) {}

		class EntryViewUI(val fragment: VaultFragment): AnkoComponent<ViewGroup> {
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
							fragment.openEntry(textView.text.toString())
						}
					}
				}
			}
		}

		override fun onCreateViewHolder(parent: ViewGroup?, viewType: Int): ViewHolder? {
			return ViewHolder(EntryViewUI(fragment).createView(AnkoContext.create(parent!!.context, parent)))
		}

		override fun onBindViewHolder(holder: ViewHolder?, position: Int) {
			holder!!.view.find<TextView>(R.id.entry_name).text = entryNames[position]
		}

		override fun getItemCount(): Int {
			return entryNames.count()
		}
	}

}