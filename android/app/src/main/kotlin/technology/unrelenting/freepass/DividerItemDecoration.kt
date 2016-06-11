package technology.unrelenting.freepass

import android.content.Context
import android.graphics.Canvas
import android.graphics.drawable.Drawable
import android.support.v4.content.ContextCompat
import android.support.v7.widget.RecyclerView

// https://stackoverflow.com/a/27037230
class DividerItemDecoration : RecyclerView.ItemDecoration {

	private var mDivider: Drawable? = null

	constructor(context: Context) {
		val styledAttributes = context.obtainStyledAttributes(ATTRS)
		mDivider = styledAttributes.getDrawable(0)
		styledAttributes.recycle()
	}

	constructor(context: Context, resId: Int) {
		mDivider = ContextCompat.getDrawable(context, resId)
	}

	override fun onDraw(c: Canvas, parent: RecyclerView, state: RecyclerView.State?) {
		val left = parent.paddingLeft
		val right = parent.width - parent.paddingRight
		val childCount = parent.childCount
		for (i in 0..childCount - 1) {
			val child = parent.getChildAt(i)
			val params = child.layoutParams as RecyclerView.LayoutParams
			val top = child.bottom + params.bottomMargin
			val bottom = top + mDivider!!.intrinsicHeight
			mDivider!!.setBounds(left, top, right, bottom)
			mDivider!!.draw(c)
		}
	}

	companion object {
		private val ATTRS = intArrayOf(android.R.attr.listDivider)
	}
}
