use anyhow::{Result, bail};
use std::process::Command;

pub fn configure_peap_via_netsh(interface: &str, username: &str) -> Result<()> {
    let profile_xml = format!(
        r#"<?xml version="1.0"?>
<LANProfile xmlns="http://www.microsoft.com/networking/LAN/profile/v1">
    <MSM>
        <security>
            <OneXEnforced>false</OneXEnforced>
            <OneXEnabled>true</OneXEnabled>
            <OneX xmlns="http://www.microsoft.com/networking/OneX/v1">
                <authMode>user</authMode>
                <EAPConfig>
                    <EapHostConfig xmlns="http://www.microsoft.com/provisioning/EapHostConfig">
                        <EapMethod>
                            <Type xmlns="http://www.microsoft.com/provisioning/EapCommon">25</Type>
                            <VendorId xmlns="http://www.microsoft.com/provisioning/EapCommon">0</VendorId>
                            <VendorType xmlns="http://www.microsoft.com/provisioning/EapCommon">0</VendorType>
                            <AuthorId xmlns="http://www.microsoft.com/provisioning/EapCommon">0</AuthorId>
                        </EapMethod>
                        <Config xmlns="http://www.microsoft.com/provisioning/EapHostConfig">
                            <Eap xmlns="http://www.microsoft.com/provisioning/BaseEapConnectionPropertiesV1">
                                <Type>25</Type>
                                <EapType xmlns="http://www.microsoft.com/provisioning/MsPeapConnectionPropertiesV1">
                                    <ServerValidation>
                                        <DisableUserPromptForServerValidation>false</DisableUserPromptForServerValidation>
                                        <ServerNames></ServerNames>
                                    </ServerValidation>
                                    <FastReconnect>true</FastReconnect>
                                    <InnerEapOptional>false</InnerEapOptional>
                                    <Eap xmlns="http://www.microsoft.com/provisioning/BaseEapConnectionPropertiesV1">
                                        <Type>26</Type>
                                        <EapType xmlns="http://www.microsoft.com/provisioning/MsChapV2ConnectionPropertiesV1">
                                            <UseWinLogonCredentials>false</UseWinLogonCredentials>
                                        </EapType>
                                    </Eap>
                                    <EnableQuarantineChecks>false</EnableQuarantineChecks>
                                    <RequireCryptoBinding>false</RequireCryptoBinding>
                                    <PeapExtensions>
                                        <PerformServerValidation xmlns="http://www.microsoft.com/provisioning/MsPeapConnectionPropertiesV2">false</PerformServerValidation>
                                        <AcceptServerName xmlns="http://www.microsoft.com/provisioning/MsPeapConnectionPropertiesV2">false</AcceptServerName>
                                    </PeapExtensions>
                                </EapType>
                            </Eap>
                        </Config>
                    </EapHostConfig>
                </EAPConfig>
            </OneX>
        </security>
    </MSM>
</LANProfile>"#
    );

    let temp_file = std::env::temp_dir().join("lan_profile.xml");
    std::fs::write(&temp_file, profile_xml)?;

    let output = Command::new("netsh")
        .args(&[
            "lan",
            "add",
            "profile",
            &format!("filename={}", temp_file.display()),
            &format!("interface={}", interface),
        ])
        .output()?;

    std::fs::remove_file(&temp_file).ok();

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("netsh 配置失败: {}", stderr);
    }

    Ok(())
}

pub fn remove_profile_via_netsh(interface: &str) -> Result<()> {
    let output = Command::new("netsh")
        .args(&["lan", "delete", "profile", &format!("interface={}", interface)])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("netsh 删除配置失败: {}", stderr);
    }

    Ok(())
}

pub fn get_lan_status(interface: &str) -> Result<String> {
    let output = Command::new("netsh")
        .args(&["lan", "show", "interfaces"])
        .output()?;

    if !output.status.success() {
        bail!("无法查询网络状态");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.to_string())
}
