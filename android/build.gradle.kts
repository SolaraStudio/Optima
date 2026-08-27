plugins {
    id("com.android.library") version "8.5.0"
    id("org.jetbrains.kotlin.android") version "1.9.25"
}

android {
    namespace = "org.optima"
    compileSdk = 35

    defaultConfig {
        minSdk = 24
        targetSdk = 35
        ndk {
            abiFilters += listOf("arm64-v8a", "armeabi-v7a", "x86", "x86_64")
        }
    }

    sourceSets {
        getByName("main") {
            jniLibs.srcDirs("src/main/jniLibs")
        }
    }
}

dependencies {
    implementation("androidx.core:core-ktx:1.15.0")
}

tasks.register<Exec>("buildRustAll") {
    workingDir = projectDir.parentFile
    commandLine("./build-all.sh")
}

tasks.register<Copy>("copyRustLibs") {
    from("../target/aarch64-linux-android/release/") {
        include("*.so")
        into("arm64-v8a/")
    }
    from("../target/armv7-linux-androideabi/release/") {
        include("*.so")
        into("armeabi-v7a/")
    }
    from("../target/i686-linux-android/release/") {
        include("*.so")
        into("x86/")
    }
    from("../target/x86_64-linux-android/release/") {
        include("*.so")
        into("x86_64/")
    }
    into("src/main/jniLibs/")
}

tasks.named("preBuild") {
    dependsOn("buildRustAll", "copyRustLibs")
}
