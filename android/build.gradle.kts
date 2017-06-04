import java.util.Properties

buildscript {
	repositories {
		jcenter()
		mavenCentral()
        gradleScriptKotlin()
		maven { setUrl("https://maven.google.com") }
	}
	dependencies {
		//classpath(kotlinModule("gradle-plugin"))
        classpath("org.jetbrains.kotlin:kotlin-gradle-plugin:1.1.2-4")
		classpath("com.android.tools.build:gradle:3.0.0-alpha3")
    }
}


allprojects {
	repositories {
		jcenter()
		mavenCentral()
		maven { setUrl("https://jitpack.io") }
		maven { setUrl("https://maven.google.com") }
		//maven { setUrl("https://oss.sonatype.org/content/repositories/snapshots") }
	}
	// Allow override for e.g. storing the project on a network drive, but building to an SSD
	val properties = Properties()
	properties.load(project.rootProject.file("local.properties").inputStream())
	buildDir = File(properties.getProperty("build-dir", "build"))
}
