package technology.unrelenting.freepass

import android.text.InputFilter
import android.text.Spanned

class NumberLimitsInputFilter(val min: Long, val max: Long) : InputFilter {
	override fun filter(src: CharSequence?, start: Int, end: Int, dst: Spanned?, dstart: Int, dend: Int): CharSequence? {
		try {
			val num = java.lang.Long.parseLong(dst.toString() + src.toString())
			if (num >= min && num <= max) return null
		} catch (e: NumberFormatException) {}
		return ""
	}
}