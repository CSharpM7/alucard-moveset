use std::{path::Path, io::Cursor,str::FromStr,fs::*};
use camino::Utf8PathBuf;


pub mod updater{
    use super::*;
    use gh_updater::ReleaseFinderConfig;
    use semver::Version;

    pub fn get_latest_version() -> Option<Version> {
        std::fs::read_to_string("sd:/ultimate/alucard/latest_version.txt")
            .ok()
            .map(|x| Version::parse(
                x.as_str()
                .trim()
                .trim_start_matches("v")
            ).ok())
            .flatten()
    }

    pub enum VersionDifference {
        ChangeToStable,
        ChangeToBeta,
        Regular,
    }
    fn compare_tags(current: &str, target: &str) -> Result<Option<VersionDifference>, semver::Error> {
        let current = Version::parse(current)?;
        let target = Version::parse(target)?;

        if current.pre.is_empty() && !target.pre.is_empty() {
            Ok(Some(VersionDifference::ChangeToBeta))
        } else if !current.pre.is_empty() && target.pre.is_empty() && current < target {
            Ok(Some(VersionDifference::ChangeToStable))
        } else if target > current {
            Ok(Some(VersionDifference::Regular))
        } else {
            Ok(None)
        }
    }

    pub fn check_for_updates() {

        let current = env!("CARGO_PKG_VERSION");

        let release = ReleaseFinderConfig::new("alucard-release")
            .with_author("CSharpM7")
            .with_repository("alucard-release")
            .with_prereleases(true)
            .find_release();
        
        let (release, prerelease) = match release {
            Ok(r) => r,
            Err(e) => {
                println!("Failed to check for updates: {:?}", e);
                return;
            }
        };

        let prerelease_tag = prerelease
            .as_ref()
            .map(|x| Version::parse(x.get_release_tag().trim_start_matches('v')).expect("Failed to parse version strings!"));
        let release_tag = release
            .as_ref()
            .map(|x| Version::parse(x.get_release_tag().trim_start_matches('v')).expect("Failed to parse version strings!"));

        let release = match (prerelease_tag, release_tag) {
            (None, None) => {
                println!("No github releases were found!");
                return
            },
            (prerelease_tag, release_tag) => {
                if prerelease_tag > release_tag {
                    prerelease.unwrap()
                } else {
                    // even if they are equal it won't matter
                    release.unwrap()
                }
            },
        };

        let version_difference = match compare_tags(current, release.get_release_tag().trim_start_matches('v')) {
            Ok(diff) => diff,
            Err(e) => {
                println!("Failed to parse version strings: {:?}", e);
                return
            },
        };

        if let Some(update_kind) = version_difference {
            println!("[smashline_alucard::update] Alucard needs an update!");

            let mut latest = String::from(current);
            if (Path::new("sd:/ultimate/alucard/latest_version.txt").exists()) {
                latest = std::fs::read_to_string("sd:/ultimate/alucard/latest_version.txt").expect("Unable to read latest version");
            }
            else {
                std::fs::File::create("sd:/ultimate/alucard/latest_version.txt");
            };
            
            println!("[smashline_alucard::update] Latest version is: {}",latest);

            let latest_difference = match compare_tags(latest.trim_start_matches('v'), release.get_release_tag().trim_start_matches('v')){
                Ok(diff) => diff,
                Err(e) => {
                    println!("[smashline_alucard::update] Failed to parse latest version strings: {:?}", e);
                    return
                },
            };

            if let Some(latest_kind) = latest_difference
            {
                //let release_text = format!("{} ({})", release.get_release_tag().trim_start_matches('v'), &release.data["name"].to_string().trim_matches('\"'));
                let release_text = format!("A new update for Alucard is available! ({}) Please download from the github page!", release.get_release_tag().trim_start_matches('v'));
                println!("{}",current);
                std::fs::write("sd:/ultimate/alucard/latest_version.txt", release.get_release_tag().trim_start_matches('v')).expect("[smashline_alucard::update] Unable to version file");
                
                skyline_web::DialogOk::ok(release_text);
                return;
            }
            else{
                println!("[smashline_alucard::update] Alucard update was already notified");
                return;
            }

        }
        else{
            println!("[smashline_alucard::update] Alucard is on the latest version");
        }
    }
}

pub mod config {
    use super::*;
    use skyline_config::*;

    pub fn get_config() -> StorageHolder<SdCardStorage> {
        StorageHolder::new(SdCardStorage::new(config()))
    }
}

pub fn config() -> Utf8PathBuf {
    Utf8PathBuf::from("sd:/ultimate/alucard")
}

pub fn install() {
    std::fs::create_dir_all(config());
    std::thread::Builder::new()
        .stack_size(0x40_0000)
        .spawn(|| {
            updater::check_for_updates();
        })
        .unwrap()
        .join();
}