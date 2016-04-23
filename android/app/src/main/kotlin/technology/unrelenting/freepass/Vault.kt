import org.bytedeco.javacpp.*
import org.bytedeco.javacpp.annotation.*

@Platform(library = "jniVault",
          cinclude = arrayOf("freepass_capi.h"))
object Vault {
    init {
        Loader.load()
        freepass_init()
    }

    @JvmStatic external fun freepass_init(): Unit
}