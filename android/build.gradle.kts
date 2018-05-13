import java.util.Properties

allprojects {
	// Allow override for e.g. storing the project on a network drive, but building to an SSD
	val properties = Properties()
	properties.load(project.rootProject.file("local.properties").inputStream())
	buildDir = File(properties.getProperty("build-dir", "build"))
}
