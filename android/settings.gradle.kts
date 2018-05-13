pluginManagement {
	repositories {
		gradlePluginPortal()
		google()
	}
	resolutionStrategy {
		eachPlugin {
			if (requested.id.id == "com.android.application") {
				useModule("com.android.tools.build:gradle:3.1.2")
			}
			if (requested.id.id.startsWith("org.jetbrains.kotlin")) {
				useVersion("1.2.41")
			}
		}
	}
}

rootProject.buildFileName = "build.gradle.kts"
include("app")