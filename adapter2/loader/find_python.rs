use crate::Error;
use loading::*;
use semver::Version;
use std::ffi::CStr;
use std::mem::transmute;
use std::os::raw::c_char;
use std::path::PathBuf;

pub fn find_python() -> Result<PathBuf, Error> {
    let locations = get_candidate_locations();
    for path in locations {
        unsafe {
            match load_library(&path, true) {
                Ok(handle) => {
                    if let Ok(ptr) = find_symbol(handle, "Py_GetVersion") {
                        let py_getversion: unsafe extern "C" fn() -> *const c_char = transmute(ptr);
                        let version = CStr::from_ptr(py_getversion());
                        if let Ok(version) = version.to_str() {
                            if let Some(version) = version.split(" ").next() {
                                if let Ok(version) = Version::parse(version) {
                                    if version.major == 3 && version.minor >= 3 {
                                        free_library(handle)?;
                                        return Ok(path);
                                    }
                                }
                            }
                        }
                    }
                    free_library(handle)?;
                }
                Err(_) => {}
            }
        }
    }
    Err("No suitable Python 3 installation found.".into())
}

#[cfg(target_os = "linux")]
fn get_candidate_locations() -> Vec<PathBuf> {
    use std::io::{BufRead, BufReader};

    fn query_sysconfig() -> Result<PathBuf, Error> {
        let result = std::process::Command::new("python3")
            .arg("-c")
            .arg("import sysconfig; print(sysconfig.get_config_var('INSTSONAME'))")
            .output()?;
        if !result.status.success() {
            return Err(format!("python exit code: {:?}", result.status.code()).into());
        }
        let stdout = BufReader::new(&result.stdout[..]);
        let mut lines = stdout.lines();
        let path = PathBuf::from(lines.next().unwrap()?);
        Ok(path)
    }

    match query_sysconfig() {
        Ok(path) => vec![path],
        Err(err) => {
            eprintln!("{}", err);
            vec![]
        }
    }
}

#[cfg(target_os = "macos")]
fn get_candidate_locations() -> Vec<PathBuf> {
    use std::io::{BufRead, BufReader};

    fn query_sysconfig() -> Result<Vec<PathBuf>, Error> {
        let result = std::process::Command::new("python3")
            .arg("-c")
            .arg("import sysconfig; print(sysconfig.get_config_var('PYTHONFRAMEWORKPREFIX')); print(sysconfig.get_config_var('LDLIBRARY'))")
            .output()?;
        if !result.status.success() {
            return Err(format!("python exit code: {:?}", result.status.code()).into());
        }
        let stdout = BufReader::new(&result.stdout[..]);
        let mut lines = stdout.lines();
        let prefix = lines.next().unwrap()?;
        let libname = lines.next().unwrap()?;

        let mut results = vec![];
        results.push(PathBuf::from(&libname));

        let mut path = PathBuf::from(prefix);
        path.push(&libname);
        results.push(path);

        let mut path = PathBuf::from("/Library/Developer/CommandLineTools/Library/Frameworks");
        path.push(&libname);
        results.push(path);

        Ok(results)
    }

    match query_sysconfig() {
        Ok(results) => results,
        Err(err) => {
            eprintln!("{}", err);
            vec![]
        }
    }
}

#[cfg(target_os = "windows")]
fn get_candidate_locations() -> Vec<PathBuf> {
    use winreg::enums::*;

    fn probe_version(hk_version: winreg::RegKey) -> Result<PathBuf, Error> {
        let sys_version: String = hk_version.get_value("SysVersion")?;
        if sys_version.as_str() >= "3" {
            let hk_install_path = hk_version.open_subkey("InstallPath")?;
            let install_path: String = hk_install_path.get_value("")?;
            let mut path = PathBuf::from(install_path);
            path.push("python3.dll");
            Ok(path)
        } else {
            Err("Nope".into())
        }
    }

    let mut results = vec![];
    for hive in &[HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE] {
        if let Ok(hk_python) = winreg::RegKey::predef(*hive).open_subkey("Software\\Python\\PythonCore") {
            for ver_tag in hk_python.enum_keys() {
                if let Ok(ver_tag) = ver_tag {
                    if let Ok(hk_version) = hk_python.open_subkey(ver_tag) {
                        if let Ok(path) = probe_version(hk_version) {
                            results.push(path);
                        }
                    }
                }
            }
        }
    }
    results
}
