import com.android.build.api.sourcesets.AndroidSourceSet
import java.io.File

repositories {
	jcenter()
	google()
	mavenCentral()
	maven("https://jitpack.io")
}

plugins {
	id("com.android.application")
	kotlin("android")
	kotlin("android.extensions")
	kotlin("kapt")
}

android {
	buildToolsVersion("28.0.0 rc2")
	compileSdkVersion(27)

	defaultConfig {
		minSdkVersion(23)
		targetSdkVersion(27)
		applicationId = "technology.unrelenting.freepass"
		versionCode = 1
		versionName = "1.0"
		multiDexEnabled = true
		ndk {
			//abiFilters("x86")//, "arm64-v8a", "armeabi-v7a"//, "armeabi"
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
			//jni.srcDirs.clear()
			withGroovyBuilder {
				"jniLibs" {
					"srcDir"("../../capi/target/android-all/debug/")
				}
			}
        }
	}

	splits {
		abi {
			isEnable = true
			reset()
			include("x86", "arm64-v8a", "armeabi-v7a")//, "armeabi"
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
				val path = configurations.getByName("compile")
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
	implementation(kotlin("stdlib", "1.2.41"))
	implementation("com.upokecenter:cbor:2.5")
	implementation("io.reactivex.rxjava2:rxjava:2.1.0")
	implementation("io.reactivex.rxjava2:rxkotlin:2.0.3")
	//implementation("io.reactivex.rxjava2:rxandroid:2.0.1")
	//implementation("com.jakewharton.rxbinding2:rxbinding-kotlin:2.0.0")
	implementation("com.github.k-kagurazaka.rx-property-android:rx-property:3.1.0")
	implementation("com.github.k-kagurazaka.rx-property-android:rx-property-kotlin:3.1.0")
	implementation("org.bytedeco:javacpp:1.1")
}