import org.jetbrains.kotlin.gradle.dsl.JvmTarget

plugins {
    id("com.android.library")
    id("org.jetbrains.kotlin.android")
    id("maven-publish")
}

android {
    namespace = "org.optima"
    compileSdk = 36

    defaultConfig {
        minSdk = 24
        targetSdk = 36
        ndk {
            abiFilters += listOf("arm64-v8a", "armeabi-v7a", "x86", "x86_64")
        }
    }

    sourceSets {
        getByName("main") {
            jniLibs.srcDirs("src/main/jniLibs")
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
}

kotlin {
    compilerOptions {
        jvmTarget.set(JvmTarget.JVM_17)
    }
}

dependencies {
    implementation("androidx.core:core-ktx:1.19.0")
    testImplementation("junit:junit:4.13.2")
}

tasks.register<Exec>("buildRustAll") {
    workingDir = rootProject.projectDir.parentFile
    commandLine("./scripts/android/build-all.sh")
}

tasks.register<Copy>("copyRustLibs") {
    from(rootProject.projectDir.parentFile.resolve("target/aarch64-linux-android/release")) {
        include("*.so")
        into("arm64-v8a/")
    }
    from(rootProject.projectDir.parentFile.resolve("target/armv7-linux-androideabi/release")) {
        include("*.so")
        into("armeabi-v7a/")
    }
    from(rootProject.projectDir.parentFile.resolve("target/i686-linux-android/release")) {
        include("*.so")
        into("x86/")
    }
    from(rootProject.projectDir.parentFile.resolve("target/x86_64-linux-android/release")) {
        include("*.so")
        into("x86_64/")
    }
    into(layout.projectDirectory.dir("src/main/jniLibs"))
}

tasks.named("preBuild") {
    dependsOn("copyRustLibs")
}

val versionSuffix = System.getenv("VERSION_SUFFIX") ?: "SNAPSHOT"
val versionName = System.getenv("OPTIMA_VERSION") ?: "0.150.10-$versionSuffix"

publishing {
    publications {
        create<MavenPublication>("release") {
            groupId = "org.optima"
            artifactId = "optima"
            version = versionName
            artifact("$buildDir/outputs/aar/optima-release.aar")
        }
    }
    repositories {
        maven {
            name = "GitHubPackages"
            url = uri("https://maven.pkg.github.com/SolaraStudio/Optima")
            credentials {
                username = System.getenv("GITHUB_ACTOR")
                password = System.getenv("GITHUB_TOKEN")
            }
        }
    }
}
