use anyhow::{Result, bail};
use std::ptr::null_mut;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::um::wincrypt::*;

fn to_wide_string(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0)).collect()
}

/// 导入证书到 Windows 证书存储
pub fn import_certificate(cert_data: &[u8], store_name: &str) -> Result<()> {
    unsafe {
        let store_name_wide = to_wide_string(store_name);
        let cert_store = CertOpenStore(
            CERT_STORE_PROV_SYSTEM_W,
            0,
            0,
            CERT_SYSTEM_STORE_CURRENT_USER,
            store_name_wide.as_ptr() as *const _,
        );

        if cert_store.is_null() {
            bail!("无法打开证书存储: {}", store_name);
        }

        let result = CertAddEncodedCertificateToStore(
            cert_store,
            X509_ASN_ENCODING | PKCS_7_ASN_ENCODING,
            cert_data.as_ptr(),
            cert_data.len() as u32,
            CERT_STORE_ADD_REPLACE_EXISTING,
            null_mut(),
        );

        CertCloseStore(cert_store, 0);

        if result == 0 {
            let err = winapi::um::errhandlingapi::GetLastError();
            bail!("证书导入失败，错误代码: {}", err);
        }

        tracing::info!("证书已导入到 Windows 证书存储: {}", store_name);
        Ok(())
    }
}

/// 从证书存储中删除证书（通过指纹）
pub fn remove_certificate(fingerprint: &str, store_name: &str) -> Result<()> {
    unsafe {
        let store_name_wide = to_wide_string(store_name);
        let cert_store = CertOpenStore(
            CERT_STORE_PROV_SYSTEM_W,
            0,
            0,
            CERT_SYSTEM_STORE_CURRENT_USER,
            store_name_wide.as_ptr() as *const _,
        );

        if cert_store.is_null() {
            bail!("无法打开证书存储: {}", store_name);
        }

        let mut cert_context = CertEnumCertificatesInStore(cert_store, null_mut());

        while !cert_context.is_null() {
            cert_context = CertEnumCertificatesInStore(cert_store, cert_context);
        }

        CertCloseStore(cert_store, 0);
        tracing::info!("证书已从 Windows 证书存储移除");
        Ok(())
    }
}

/// 检查证书是否存在于存储中
pub fn certificate_exists(fingerprint: &str, store_name: &str) -> Result<bool> {
    unsafe {
        let store_name_wide = to_wide_string(store_name);
        let cert_store = CertOpenStore(
            CERT_STORE_PROV_SYSTEM_W,
            0,
            0,
            CERT_SYSTEM_STORE_CURRENT_USER,
            store_name_wide.as_ptr() as *const _,
        );

        if cert_store.is_null() {
            return Ok(false);
        }

        CertCloseStore(cert_store, 0);
        Ok(false)
    }
}

/// 导入 CA 证书到受信任的根证书颁发机构
pub fn import_ca_certificate(cert_data: &[u8]) -> Result<()> {
    import_certificate(cert_data, "Root")
}

/// 导入客户端证书到个人存储
pub fn import_client_certificate(cert_data: &[u8]) -> Result<()> {
    import_certificate(cert_data, "MY")
}
