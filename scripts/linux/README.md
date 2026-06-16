# Linux 终端脚本

本目录包含用于 Linux Server（无 GUI）的终端脚本。

## radius-setup.sh

**用途**：在无图形界面的 Linux 服务器上配置 802.1X PEAP 认证

**功能**：
- ✅ 启用/禁用 802.1X 认证
- ✅ 查看认证状态
- ✅ 系统诊断（检查依赖、服务）
- ✅ 列出可用网络接口

**使用方法**：

```bash
# 1. 赋予执行权限
chmod +x radius-setup.sh

# 2. 以 root 权限运行
sudo ./radius-setup.sh
```

**系统要求**：
- NetworkManager（推荐）或 wpa_supplicant
- systemd（用于服务管理）
- bash 4.0+

**支持的发行版**：
- Ubuntu 18.04+
- Debian 10+
- CentOS 7+
- RHEL 7+

**交互式菜单**：
```
=========================================
 RADIUS 认证客户端 - Linux 终端版
=========================================
1. 启用 802.1X 认证
2. 禁用 802.1X 认证
3. 查看状态
4. 系统诊断
5. 列出网络接口
0. 退出
=========================================
```

## 自动化部署

如需批量部署，可使用非交互模式：

```bash
#!/bin/bash
# 自动配置脚本示例

INTERFACE="eth0"
USERNAME="user@example.com"
PASSWORD="your_password"

# 静默配置
nmcli connection add \
    type ethernet \
    con-name "RADIUS-${INTERFACE}" \
    ifname "$INTERFACE" \
    802-1x.eap peap \
    802-1x.phase2-auth mschapv2 \
    802-1x.identity "$USERNAME" \
    802-1x.password "$PASSWORD" \
    802-1x.anonymous-identity "anonymous" \
    connection.autoconnect yes

nmcli connection up "RADIUS-${INTERFACE}"
```

## 故障排查

**问题 1**：NetworkManager 未运行
```bash
sudo systemctl start NetworkManager
sudo systemctl enable NetworkManager
```

**问题 2**：wpa_supplicant 缺失
```bash
# Debian/Ubuntu
sudo apt update && sudo apt install wpasupplicant

# CentOS/RHEL
sudo yum install wpa_supplicant
```

**问题 3**：权限不足
```bash
# 确保以 root 运行
sudo ./radius-setup.sh
```
