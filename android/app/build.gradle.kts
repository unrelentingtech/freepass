import com.android.build.gradle.AppExtension
import java.io.File

apply {
	plugin("com.android.application")
	plugin("kotlin-android")
	plugin("kotlin-android-extensions")
	plugin("kotlin-kapt")
}

configure<AppExtension> {
	buildToolsVersion("25.0.2")
	compileSdkVersion(25)

	defaultConfig {
		minSdkVersion(23)
		targetSdkVersion(25)
		applicationId = "technology.unrelenting.freepass"
		versionCode = 1
		versionName = "1.0"
		multiDexEnabled = true
		ndk {
			abiFilters("x86")//, "arm64-v8a", "armeabi-v7a"//, "armeabi"
		}
		externalNativeBuild {
			cmake {
				arguments.addAll(arrayOf("-DANDROID_PLATFORM=android-24", "-DANDROID_STL=stlport_shared", "-DANDROID_CPP_FEATURES=exceptions"))
			}
        }
	}

	externalNativeBuild {
		cmake {
			setPath("CMakeLists.txt")
		}
	}

	dexOptions {
		preDexLibraries = true
		dexInProcess = true
	}

	dataBinding {
		isEnabled = true
	}

	buildTypes {
		getByName("release") {
            isMinifyEnabled = false
			proguardFiles(getDefaultProguardFile("proguard-android.txt"), "proguard-rules.pro")
		}
		getByName("debug") {
            isJniDebuggable = true
		}
	}

	sourceSets {
        getByName("main") {
            java.srcDirs += File("src/main/kotlin")
            jni.srcDirs.clear()
        }
	}

	splits {
		abi {
			isEnable = true
			reset()
			include("x86")//, "arm64-v8a", "armeabi-v7a"//, "armeabi"
			isUniversalApk = true
		}
	}

	packagingOptions {
		exclude("META-INF/LICENSE")
    }

	applicationVariants.forEach { variant ->
		// Don"t move the task to the top level, it needs the `variant`!
		val javacppTask = task("build${variant.name.capitalize()}Javacpp") {
			inputs.file("src/main/kotlin/technology/unrelenting/freepass/Vault.kt")
			outputs.file("src/main/jni/jniVault.cpp")
			doLast {
				val path = configurations.compile
						.find { it.name.startsWith("javacpp") }
						?.absolutePath
				javaexec {
					main = "org.bytedeco.javacpp.tools.Builder"
					classpath(path)
					val jc = variant.javaCompiler as JavaCompile
					args("-cp", jc.destinationDir,
							"-cp", jc.destinationDir.toString()
							.replace("intermediates/classes", "tmp/kotlin-classes")
							.replace("intermediates\\classes", "tmp\\kotlin-classes"),
							"-d", "src/main/jni", "-nocompile", "technology.unrelenting.freepass.Vault")
				}
			}
		}
		javacppTask.dependsOn(variant.javaCompiler)
		variant.externalNativeBuildTasks.forEach { it.dependsOn(javacppTask) }
	}
}

dependencies {
	//compile(kotlinModule("stdlib"))
	compile("org.jetbrains.kotlin:kotlin-stdlib:1.1.2-4")
	compile("com.upokecenter:cbor:2.5")
	compile("io.reactivex.rxjava2:rxjava:2.1.0")
	compile("io.reactivex.rxjava2:rxkotlin:2.0.3")
	//compile("io.reactivex.rxjava2:rxandroid:2.0.1")
	//compile("com.jakewharton.rxbinding2:rxbinding-kotlin:2.0.0")
	compile("com.github.k-kagurazaka.rx-property-android:rx-property:3.1.0")
	compile("com.github.k-kagurazaka.rx-property-android:rx-property-kotlin:3.1.0")
	compile("org.bytedeco:javacpp:1.1")
	kapt("com.android.databinding:compiler:2.3.1")
}