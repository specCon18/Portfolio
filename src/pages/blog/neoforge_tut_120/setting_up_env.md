Clone the NeoForged MDK
1a. The Repository name should be the name of your mod ex) UtilityRings
specCon18 — 10/06/2023 9:26 PM
Clone the repo to your local development machine using the Code dropdown above the file browser in the repo and copying the link there.
2a. run the following command in your terminal in the working directory where you want the project:
git clone <YOUR_LINK_FROM_STEP_2>
 
2b. use cd to move into your projects directory
Run the following command to generate the runners for the mod in VSCode (Client,Server,Data,GameTestServer):
     Windows: ./gradlew.bat genVSCodeRuns
     Linux/MacOS: ./gradlew genVSCodeRuns
Open the project in VSCode by running code . when you do this it may prompt you to install a set of plugins... do so.
 
specCon18 — 10/06/2023 9:34 PM
open settings.gradle in the root of your project and replace its contents with the following:
pluginManagement {
    repositories {
        gradlePluginPortal()
        maven {
            name = 'NeoForged'
            url = 'https://maven.neoforged.net/releases'
        }
        maven { 
            name = 'ParchmentMC'
            url = 'https://maven.parchmentmc.org'
        }
    }
}

plugins {
    id 'org.gradle.toolchains.foojay-resolver-convention' version '0.5.0'
}
5a. add this line to your plugins section at the top of the build.gradle file in your projects root:
id 'org.parchmentmc.librarian.forgegradle' version '1.2.0'
5b. find the section of your build.gradle file that is named minecraft and change the inner mappings line to the following:
 minecraft {
     mappings channel: 'parchment', version: '2023.09.03-1.20.1'
     //Rest of section ...
 }
specCon18 — 10/06/2023 9:41 PM
5c. repeat step 3
specCon18 — 10/06/2023 9:53 PM
replace all instances of examplemod in mods.toml with your mods modid which is the mods name in snakecase ex) util_rings
6a. replace displayName="${mod_name}" #mandatory with displayName="your_mods_pretty_name" ex) modid = utility_rings mods display name is Utility Rings
6b. replace version="${mod_version}" #mandatory with version="1.20.1-1.0.0.0"
6c. replace authors="${mod_authors}" #optional with authors="YOUR_NAME_HERE"
6d. replace description='''${mod_description}''' with description='''A_MULTI_LINE_DESC_OF_YOUR_MOD'''
6e. replace license="${mod_license}" with the license of your choosing from https://choosealicense.com/
6f. remove this section:
# A dependency - use the . to indicate dependency for a specific modid. Dependencies are optional.
[[dependencies.${mod_id}]] #optional
    # the modid of the dependency
    modId="forge" #mandatory
    # Does this dependency have to exist - if not, ordering below must be specified
    mandatory=true #mandatory
    # The version range of the dependency
    versionRange="${neo_version_range}" #mandatory
    # An ordering relationship for the dependency - BEFORE or AFTER required if the dependency is not mandatory
    # BEFORE - This mod is loaded BEFORE the dependency
    # AFTER - This mod is loaded AFTER the dependency
    ordering="NONE"
    # Side this dependency is applied on - BOTH, CLIENT, or SERVER
    side="BOTH"
# Here's another dependency
[[dependencies.${mod_id}]]
    modId="minecraft"
    mandatory=true
    # This version range declares a minimum of the current minecraft version up to but not including the next major version
    versionRange="${minecraft_version_range}"
    ordering="NONE"
    side="BOTH"
 
6g. rename the directory <PROJECT_ROOT>/src/java/main/examplemod to <PROJECT_ROOT>/src/java/main/<YOUR_MOD_ID> 
6h. rename the file <PROJECT_ROOT>/src/java/main/<YOUR_MOD_ID>/ExampleMod.java to <PROJECT_ROOT>/src/java/main/<YOUR_MOD_ID>/<YOUR_MOD_NAME_NO_SPACES>.java 
specCon18 — 10/06/2023 10:09 PM
6i. in <PROJECT_ROOT>/src/java/main/<YOUR_MOD_ID>/<YOUR_MOD_NAME_NO_SPACES>.java replace public static final String MODID = "examplemod"; with public static final String MODID = "YOUR_MOD_ID";
Mod Id Replacement

Replace all occurrences of examplemod, including mods.toml and the main mod file with the mod id of your mod. This also includes changing the name of the file you build by setting base.archivesName (this is typically set to your mod id).

// In some build.gradle
base.archivesName = 'mymod'

Group Id

The group property should be set to your top-level package, which should either be a domain you own or your email address:
Type    Value    Top-Level Package
Domain    example.com    com.example
Subdomain    example.github.io    io.github.example
Email    example@gmail.com    com.gmail.example

// In some build.gradle
group = 'com.example'

The packages within your java source (src/main/java) should also now conform to this structure, with an inner package representing the mod id:

com
example (top-level package specified in group property)
mymod (the mod id)
MyMod.java (renamed ExampleMod.java)

Version

Set the version property to the current version of your mod. We recommend using a variation of Maven versioning.

// In some build.gradle
version = '1.20-1.0.0.0'
once you have completed those modifications press ctrl+shift+D in vs code to open the debug and run menu and use the runClient profile to launch the game and test build your mod